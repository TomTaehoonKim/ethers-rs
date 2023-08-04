use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    types::Address, signers::get_wallet,
};
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};
// Generate the type-safe contract bindings by providing the ABI
// definition
abigen!(
    USDTContract,
    "examples/transactions/examples/contracts/erc20_example/abi/usdt_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    const RPC_URL: &str = "http://localhost:9545";
    const NUM_TXS: u32 = 200;
    let router_address = "0x7B4f352Cd40114f12e82fC675b5BA8C7582FC513".parse::<Address>()?;
    let usdt_address = "0xcE0066b1008237625dDDBE4a751827de037E53D2".parse::<Address>()?;
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let mut handles = vec![];
    for i in 0..NUM_TXS {
        let wallet = SignerMiddleware::new(provider.clone(), get_wallet(i as u32));
        let contract = USDTContract::new(usdt_address, Arc::new(wallet.clone()));
        let handle = tokio::spawn(async move {
            let receipt = contract
                .approve(router_address, 100000000.into())
                .send()
                .await
                .unwrap()
                .await
                .unwrap();
            println!("Approve {i}: {:?}", receipt);
        });
        handles.push(handle);
    }
    futures::future::join_all(handles).await;
    Ok(())
}