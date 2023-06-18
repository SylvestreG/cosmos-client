use cosmos_client::client::RpcClient;
use cosmos_client::error::CosmosClientError;
use cosmos_client::signer::Signer;
use std::env;

#[tokio::main]
async fn main() -> Result<(), CosmosClientError> {
    // ask for 24 workds
    env_logger::init();

    let input = env::var("WALLET_MNEMONIC");
    if input.is_err() {
        eprintln!("please set WALLET_MNEMONIC with your 24 words");
        return Ok(());
    }
    let input = input.unwrap_or_default();

    let mut client = RpcClient::new("https://rpc-mainnet.blockchain.ki").await?;
    let signer = Signer::from_mnemonic(input.trim(), "ki", "uxki", None, 10, 2500)?;
    let address = signer.public_address.to_string();
    client.attach_signer(signer).await?;
    println!("signer loaded for {}", address);

    let validators = client
        .staking
        .delegator_validators(address.as_str(), None)
        .await?;
    println!("{:#?}", validators);

    Ok(())
}
