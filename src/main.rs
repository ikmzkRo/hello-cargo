use std::env;
use ethers::{Provider, SignerMiddleware, LocalWallet};
use ethers::core::types::Network; // Make sure to replace this with the actual path to your Network type
use ethers::prelude::{Http, EthersResult}; // Make sure to replace this with the actual path to your EthersResult type

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

pub async fn get_balance(network: Network) -> EthersResult<()> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

    let provider = Provider::<Http>::try_from(network.chain_url()).unwrap();
    let wallet = wallet_secret
        .parse::<LocalWallet>()?
        .with_chain_id(network.chain_id());

    let client = SignerMiddleware::new_with_provider_chain(provider, wallet.to_owned())
        .await
        .unwrap();

    let converted_balance = balance.to_string(); 

    println!("balance: {:?}", converted_balance);
    
    Ok(())
}
