use ethers::{
    core::types::TransactionRequest,
    prelude::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::get_wallet,
    utils::parse_ether,
};
use eyre::Result;
use std::convert::TryFrom;
#[tokio::main]
async fn main() -> Result<()> {
    const RPC_URL: &str = "http://localhost:9545";
    const NUM_TXS: u32 = 200;
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let from_wallet = SignerMiddleware::new(provider.clone(), get_wallet(0));
    // In Kroma devnet, the first 20 accounts are pre-funded with infinite amount of ETH
    for i in 20..NUM_TXS {
        let to_wallet = SignerMiddleware::new(provider.clone(), get_wallet(i as u32));
        let tx = TransactionRequest::new()
            .to(to_wallet.address())
            .value(parse_ether(1000000)?)
            .from(from_wallet.address());
        let receipt = from_wallet.send_transaction(tx, None).await.unwrap().await.unwrap();
        println!("tx {}: {:?}", i, receipt);
    }
    Ok(())
}
