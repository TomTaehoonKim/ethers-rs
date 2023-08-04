use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::get_wallet,
    types::{Address, U256},
};
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};
// Generate the ABI for the USDTContract. This is will define an `USDTContract` struct in
// this scope that will let us call the methods of the contract.
abigen!(
    USDTContract,
    "examples/transactions/examples/contracts/erc20_example/abi/usdt_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<()> {
    const RPC_URL: &str = "http://localhost:9545";
    const NUM_TXS: u32 = 200;
    let usdt_address = "0xcE0066b1008237625dDDBE4a751827de037E53D2".parse::<Address>()?;
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let from_wallet = SignerMiddleware::new(provider.clone(), get_wallet(0));
    // Create the contract instance to let us call methods of the contract and let it sign
    // transactions with the sender wallet.
    let contract = USDTContract::new(usdt_address, Arc::new(from_wallet.clone()));
    for i in 1..NUM_TXS {
        let wallet = SignerMiddleware::new(provider.clone(), get_wallet(i as u32));
        // Transfer the desired amount of tokens to the `to_address`
        let tx = contract.transfer(wallet.address(), U256::from(100000000) * U256::exp10(6 as usize));
        let pending_tx = tx.send().await?;
        let reciept = pending_tx.await?;
        println!("tx {}: {:?}", i, reciept);
    }
    Ok(())
}