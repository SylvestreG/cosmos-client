use cosmos_client::client::Rpc;
use cosmos_client::cosmos_sdk::cosmos::base::v1beta1::Coin;
use cosmos_client::error::CosmosClient;
use cosmos_client::signer::Signer;
use std::env;

#[tokio::main]
async fn main() -> Result<(), CosmosClient> {
    // ask for 24 workds
    env_logger::init();

    let input = env::var("WALLET_MNEMONIC");
    if input.is_err() {
        eprintln!("please set WALLET_MNEMONIC with your 24 words");
        return Ok(());
    }
    let input = input.unwrap_or_default();

    let mut client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;
    let signer = Signer::from_mnemonic(input.trim(), "ki", "uxki", None, 30, 25000)?;
    let address = signer.public_address.to_string();
    client.attach_signer(signer).await?;
    println!("signer loaded for {address}");

    let response = client
        .send(
            address.as_str(),
            vec![Coin {
                denom: "uxki".to_string().parse()?,
                amount: "1000".to_string(),
            }],
            None,
        )
        .await?;
    println!("response {response:#?}");

    Ok(())
}
