use cosmos_client::client::Rpc;
use cosmos_client::cosmos_sdk::cosmos::base::v1beta1::{Coin, DecCoin};
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

    let mut client = Rpc::new("https://rpc-mainnet.blockchain.ki").await?;
    let signer = Signer::from_mnemonic(input.trim(), "ki", "uxki", None, 30, 25000)?;
    let address = signer.public_address.to_string();
    client.attach_signer(signer).await?;
    println!("signer loaded for {address}");

    let validators = client
        .staking
        .delegator_validators(address.as_str(), None)
        .await?;

    for validator in validators.validators {
        let delegation = client
            .staking
            .delegation(address.as_str(), validator.operator_address.as_str())
            .await?;
        let rewards = client
            .distribution
            .delegation_rewards(address.as_str(), validator.operator_address.as_str())
            .await?;
        let valop = delegation
            .delegation_response
            .unwrap_or_default()
            .delegation
            .unwrap_or_default()
            .validator_address;

        let rewards = rewards
            .rewards
            .iter()
            .filter(|&x| x.denom == "uxki")
            .collect::<Vec<&DecCoin>>();
        for reward in rewards {
            let uxki_to_claim = reward.amount.parse::<u128>()? / 1_000_000_000_000_000_000u128;
            if uxki_to_claim > 0 {
                println!("claiming {uxki_to_claim} uxki on {valop}");
                let tx = client
                    .claim_rewards(valop.as_str(), Some("AutoClaim"))
                    .await?;
                println!("Tx hash {}", tx.tx_response.unwrap().txhash);

                println!("staking {uxki_to_claim} uxki on {valop}");
                let tx = client
                    .stake(
                        valop.as_str(),
                        Coin {
                            denom: "uxki".to_string(),
                            amount: uxki_to_claim.to_string(),
                        },
                        Some("AutoStake"),
                    )
                    .await?;
                println!("Tx hash {}", tx.tx_response.unwrap().txhash);
            }
        }
    }

    Ok(())
}
