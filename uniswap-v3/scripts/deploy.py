from brownie import *

from .uni_math import *

# import brownie.network.contract
# import brownie.network.account
# import brownie.project

def main():
    deployer = accounts.add("a"*64)
    if len(accounts):
        accounts.default = accounts[0]
        accounts.default.transfer(deployer, accounts.default.balance()//10, required_confs=0)

    tx_control = {
        "from": deployer,
        "required_confs": 0
    }


    ## get imported contract container
    FactoryContainer = globals()['Uniswap/v3-core@1.0.0/UniswapV3Factory']
    UniswapV3PoolContainer = globals()['Uniswap/v3-core@1.0.0/UniswapV3Pool']

    weth = WETH.deploy(tx_control)
    nft_descriptor_lib = NFTDescriptor.deploy() # NFT descriptor need library contract

    history.wait()
    weth = Contract.from_abi('WETH', weth.contract_address, WETH.abi)

    ## deploy uniswap v3 ##
    descriptor = NonfungibleTokenPositionDescriptor.deploy(weth, tx_control)
    factory = FactoryContainer.deploy(tx_control)

    history.wait()

    factory = Contract.from_abi('WETH', factory.contract_address, FactoryContainer.abi)
    descriptor = Contract.from_abi('WETH', descriptor.contract_address, NonfungibleTokenPositionDescriptor.abi)

    manager = NonfungiblePositionManager.deploy(factory, weth, descriptor, tx_control)
    swap_router = SwapRouter.deploy(factory, weth, tx_control)

    history.wait()

    manager = Contract.from_abi('WETH', manager.contract_address, NonfungiblePositionManager.abi)
    swap_router = Contract.from_abi('WETH', swap_router.contract_address, SwapRouter.abi)
    ## done uniswap v3 deploy ##

    ## deploy token(for make v3 pair)
    stoken = ERC20Mintable.deploy("Stable Token", "STK", tx_control)

    history.wait()

    stoken = Contract.from_abi('STA', stoken.contract_address, ERC20Mintable.abi)

    ## liquidity
    accounts.default.transfer(weth, 2*10**18)
    stoken.mint(accounts.default, 1000*10**18, {'from': deployer})

    weth.approve(manager, (1<<256)-1)
    stoken.approve(manager, (1<<256)-1)
    weth.approve(swap_router, (1<<256)-1)
    stoken.approve(swap_router, (1<<256)-1)

    uni_math = UniswapMath()

    tokenA = weth
    tokenB = stoken

    test_amountA = 1
    test_amountB = 1000
    fee = 500 # denominated in hundredths of a bip: 0.01% * 10^-2

    # amountA * price == amountB
    price = 1000

    if int(tokenA.address, 16) >= int(tokenB.address, 16):
        tokenA, tokenB = tokenB, tokenA
        test_amountA, test_amountB = test_amountB, test_amountA
        price = 1/price

    # create and initialize pool
    create_pool_params = [tokenA, tokenB, fee,
                          uni_math.get_sqrtRatioX96(tokenA, tokenB, price)]
    manager.createAndInitializePoolIfNecessary(*create_pool_params)

    # fetch Uniswap pool
    pool_addr = factory.getPool(tokenA, tokenB, fee)
    pool = Contract.from_abi("", pool_addr, UniswapV3PoolContainer.abi)

    # get tick range bound
    lower_tick = tick_space_floor(
        pool, uni_math.get_tick(tokenA, tokenB, price*0.98))
    upper_tick = tick_space_floor(
        pool, uni_math.get_tick(tokenA, tokenB, price*1.02))

    # "mint", provide liquidity to pools
    minting_params = [tokenA, tokenB, fee, lower_tick, upper_tick,
                      # token_amount(tokenA, amountA),
                      # token_amount(tokenB, amountB),
                      token_amount(tokenA, test_amountA),
                      token_amount(tokenB, test_amountB),
                      0, 0, accounts.default, chain.time()*2]

    manager.mint(minting_params)


    balance_before = stoken.balanceOf( accounts.default ) / 10**18
    swap_router.exactInputSingle(
        [
            weth,
            stoken,
            fee,
            accounts.default,
            chain.time() + 100,
            10**17, # 0.1 eth
            0, # minumum
            0 # sqrtPriceLimitX96 == 0,
        ]
    )
    print("before", balance_before)
    print(" after", stoken.balanceOf( accounts.default ) / 10**18)