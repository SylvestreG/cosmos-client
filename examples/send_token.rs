use cosmos_client::client::RpcClient;
use cosmos_client::cosmos_sdk::cosmos::bank::v1beta1::MsgSend;
use cosmos_client::cosmos_sdk::cosmos::base::v1beta1::Coin;
use cosmos_client::cosmos_sdk::traits::MessageExt;
use cosmos_client::error::CosmosClientError;
use cosmos_client::signer::Signer;
use cosmos_client::tx::CosmosTx;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;
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

    for _ in 0..20 {
        let mut payload = CosmosTx::build();
        payload = payload.memo("send token").add_msg(
            MsgSend {
                from_address: address.to_string(),
                to_address: address.to_string(),
                amount: vec![Coin {
                    denom: "uxki".to_string().parse()?,
                    amount: "1000".to_string(),
                }],
            }
            .to_any()?,
        );
        let response = client
            .sign_and_broadcast(payload, BroadcastMode::Sync)
            .await?;
        println!("{:#?}", response);
    }

    Ok(())
}
