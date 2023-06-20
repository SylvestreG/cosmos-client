#[cfg(test)]
mod params_tests {
    use crate::client::Rpc;
    use crate::error::CosmosClient;

    #[tokio::test]
    async fn evidence() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let params = client.params.params("distribution", "0").await?;

        assert!(params.param.is_some());
        let param = params.param.unwrap();
        assert_eq!(param.subspace, "distribution");
        assert_eq!(param.key, "0");
        assert_eq!(param.value, "");
        Ok(())
    }
}
