#[cfg(test)]
mod bank_tests {
    use crate::client::RpcClient;
    use cosmos_sdk_proto::cosmos::bank::v1beta1::DenomUnit;

    #[tokio::test]
    async fn balances() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let balances = client
            .bank
            .all_balances("ki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8usr5tl8", None)
            .await?;
        assert!(!balances.balances.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn balance_by_denom() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let balance = client
            .bank
            .balance("ki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8usr5tl8", "uxki")
            .await?;
        assert!(balance.balance.is_some());
        assert_eq!(balance.balance.unwrap().denom, "uxki");
        Ok(())
    }

    #[tokio::test]
    async fn spendable_balances() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let balances = client
            .bank
            .spendable_balances("ki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8usr5tl8", None)
            .await?;
        assert!(!balances.balances.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn supply() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let supply = client.bank.total_supply(None).await?;
        assert!(!supply.supply.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn supply_by_denom() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let supply = client.bank.supply_of("uxki").await?;

        assert!(supply.amount.is_some());
        assert_eq!(supply.amount.clone().unwrap().denom, "uxki");
        assert!(supply.amount.unwrap().amount.parse::<u128>().unwrap() > 1_000_000_000_000_000u128);
        Ok(())
    }

    #[tokio::test]
    async fn params() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let params = client.bank.params().await?;
        assert!(params.params.is_some());
        assert!(params.params.unwrap().default_send_enabled);
        Ok(())
    }

    #[tokio::test]
    async fn metadata_by_denom() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let metadata = client.bank.denom_metadata("uxki").await?;

        assert!(metadata.metadata.is_some());
        let metadata = metadata.metadata.unwrap();
        assert_eq!(
            metadata.description,
            "The native staking token of the KiChain."
        );
        assert_eq!(metadata.base, "uxki");
        assert_eq!(metadata.name, "");
        assert_eq!(metadata.display, "xki");
        assert_eq!(metadata.symbol, "");
        assert_eq!(metadata.denom_units.len(), 3);
        assert_eq!(
            metadata.denom_units[0],
            DenomUnit {
                denom: "uxki".to_string(),
                exponent: 0,
                aliases: vec!["microxki".to_string()]
            }
        );
        assert_eq!(
            metadata.denom_units[1],
            DenomUnit {
                denom: "mxki".to_string(),
                exponent: 3,
                aliases: vec!["millixki".to_string()]
            }
        );
        assert_eq!(
            metadata.denom_units[2],
            DenomUnit {
                denom: "xki".to_string(),
                exponent: 6,
                aliases: vec![]
            }
        );
        Ok(())
    }

    #[tokio::test]
    async fn metadata() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let metadata = client.bank.denoms_metadata(None).await?;

        assert_eq!(metadata.metadatas.len(), 1);
        assert_eq!(
            metadata.metadatas[0].description,
            "The native staking token of the KiChain."
        );
        assert_eq!(metadata.metadatas[0].base, "uxki");
        assert_eq!(metadata.metadatas[0].name, "");
        assert_eq!(metadata.metadatas[0].display, "xki");
        assert_eq!(metadata.metadatas[0].symbol, "");
        assert_eq!(metadata.metadatas[0].denom_units.len(), 3);
        assert_eq!(
            metadata.metadatas[0].denom_units[0],
            DenomUnit {
                denom: "uxki".to_string(),
                exponent: 0,
                aliases: vec!["microxki".to_string()]
            }
        );
        assert_eq!(
            metadata.metadatas[0].denom_units[1],
            DenomUnit {
                denom: "mxki".to_string(),
                exponent: 3,
                aliases: vec!["millixki".to_string()]
            }
        );
        assert_eq!(
            metadata.metadatas[0].denom_units[2],
            DenomUnit {
                denom: "xki".to_string(),
                exponent: 6,
                aliases: vec![]
            }
        );
        Ok(())
    }
}
