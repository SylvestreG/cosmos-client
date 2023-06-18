use cosmos_client::client::RpcClient;
use cosmos_client::cosmos_sdk::cosmos::base::v1beta1::Coin;
use cosmos_client::error::CosmosClientError;
use cosmos_client::signer::Signer;
use std::io::stdin;

#[tokio::main]
async fn main() -> Result<(), CosmosClientError> {
    // ask for 24 workds
    env_logger::init();

    println!("Give me you 24 words: ");
    let mut input: String = String::new();
    stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    let mut client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;
    let signer = Signer::from_mnemonic(input.trim(), "ki", "uxki", None, 10, 2500)?;
    let address = signer.public_address.to_string();
    client.attach_signer(signer).await?;
    println!("signer loaded for {}", address);

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
    println!("{:#?}", response);

    Ok(())
}
