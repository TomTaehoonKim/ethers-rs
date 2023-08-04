use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    types::Address,
};
use ethers_signers::{get_wallet, Signer};
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};
// Generate the type-safe contract bindings by providing the ABI
// definition
abigen!(
    UniswapV2Router02,
    "./examples/contracts/examples/abi/UniswapV2Router02.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<()> {
    const RPC_URL: &str = "http://localhost:9545";
    const NUM_TXS: u32 = 200;
    let router_address = "0x7B4f352Cd40114f12e82fC675b5BA8C7582FC513".parse::<Address>()?;
    let usdt_address = "0xcE0066b1008237625dDDBE4a751827de037E53D2".parse::<Address>()?;
    let usdc_address = "0x82EdA215Fa92B45a3a76837C65Ab862b6C7564a8".parse::<Address>()?;
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let mut handles = vec![];
    for i in 0..NUM_TXS {
        let wallet = SignerMiddleware::new(provider.clone(), get_wallet(i as u32).with_chain_id(901u64));
        let contract = UniswapV2Router02::new(router_address, Arc::new(wallet.clone()));
        let handle = tokio::spawn(async move {
            let receipt = contract
                .swap_tokens_for_exact_tokens(
                    100.into(),
                    1000.into(),
                    vec![usdt_address, usdc_address],
                    wallet.address(),
                    "0x74b8ad58".into(),
                )
                .send()
                .await
                .unwrap()
                .await
                .unwrap();
            println!("Receipt {}: {:?}", i, receipt);
        });
        handles.push(handle);
    }
    futures::future::join_all(handles).await;
    Ok(())
}