#[cfg(test)]
mod authz_tests {
    use crate::client::RpcClient;
    use std::str::FromStr;

    #[tokio::test]
    async fn grantee() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let grants = client
            .authz
            .grantee_grants("ki1a9j9ncrky0mt7hgdtuyzm08yvd3mdu0xrpjtf4", None)
            .await?;

        assert!(grants.pagination.is_some());
        let pagination = grants.pagination.unwrap();
        assert!(pagination.total >= 2);
        assert_eq!(
            grants.grants[0].grantee,
            "ki1a9j9ncrky0mt7hgdtuyzm08yvd3mdu0xrpjtf4"
        );
        assert_eq!(
            grants.grants[0].expiration,
            Some(prost_types::Timestamp::from_str("2023-06-24T14:49:02Z")?)
        );

        Ok(())
    }

    #[tokio::test]
    async fn granter() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let grants = client
            .authz
            .granter_grants("ki1cfy5hq7n35et7geqkc2d3xxjz6sl8dp8p5yejl", None)
            .await?;

        assert!(grants.pagination.is_some());
        let pagination = grants.pagination.unwrap();

        assert!(pagination.total >= 2);
        assert_eq!(
            grants.grants[0].granter,
            "ki1cfy5hq7n35et7geqkc2d3xxjz6sl8dp8p5yejl"
        );
        assert_eq!(
            grants.grants[0].expiration,
            Some(prost_types::Timestamp::from_str("2023-06-24T14:51:46Z")?)
        );
        Ok(())
    }
}
