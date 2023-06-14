#[cfg(test)]
mod mint_tests {
    use crate::client::RpcClient;
    use cosmos_sdk_proto::cosmos::mint::v1beta1::Params;
    use std::str;

    #[tokio::test]
    async fn params() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let params = client.mint.params().await?;
        assert_eq!(
            params.params,
            Some(Params {
                mint_denom: "uxki".to_string(),
                inflation_rate_change: "1000000000000000".to_string(),
                inflation_max: "90000000000000000".to_string(),
                inflation_min: "89000000000000000".to_string(),
                goal_bonded: "670000000000000000".to_string(),
                blocks_per_year: 6311520,
            })
        );
        Ok(())
    }

    #[tokio::test]
    async fn inflation() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let inflation = client.mint.inflation().await?;
        assert!(
            str::from_utf8(inflation.inflation.as_slice())?
                .parse::<f64>()
                .unwrap()
                > 1f64
        );
        Ok(())
    }

    #[tokio::test]
    async fn annual_provisions() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let annual_provisions = client.mint.annual_provisions().await?;
        let prov = str::from_utf8(annual_provisions.annual_provisions.as_slice())?
            .parse::<f32>()
            .unwrap();
        assert!(prov > 142613936805330.760000000000000000);
        Ok(())
    }
}
