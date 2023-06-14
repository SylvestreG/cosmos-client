#[cfg(test)]
mod params_tests {
    use crate::client::RpcClient;

    #[tokio::test]
    async fn evidence() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let params = client.params.params("distribution", "0").await?;

        assert!(params.param.is_some());
        let param = params.param.unwrap();
        assert_eq!(param.subspace, "distribution");
        assert_eq!(param.key, "0");
        assert_eq!(param.value, "");
        Ok(())
    }
}
