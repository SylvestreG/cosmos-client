use cosmos_client::client::RpcClient;
use cosmos_client::cosmos_sdk::cosmos::bank::v1beta1::MsgSend;
use cosmos_client::cosmos_sdk::cosmos::base::v1beta1::Coin;
use cosmos_client::error::CosmosClientError;
use cosmos_client::signer::{CosmosTx, Signer};
use cosmos_client::cosmos_sdk::traits::MessageExt;

use std::io::stdin;

#[tokio::main]
async fn main() -> Result<(), CosmosClientError> {
    // ask for 24 workds
    println!("Give me you 24 words: ");
    let mut input: String = String::new();
    stdin().read_line(&mut input)?;
    input = input.trim().to_string();

    let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;
    let signer = Signer::from_pkey(input.trim(), "ki", "uxki")?;
    println!("signer loaded for {}", signer.get_address());

    let mut payload = CosmosTx::build();
    payload = payload.memo("send token").add_msg(
        MsgSend {
            from_address: signer.get_address(),
            to_address: signer.get_address(),
            amount: vec![Coin {
                denom: "uxki".to_string().parse().unwrap(),
                amount: "1000".to_string(),
            }],
        }
        .to_any()
        .unwrap(),
    );
    payload.sign(signer, client).await?;

    Ok(())
}
