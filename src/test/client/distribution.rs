#[cfg(test)]
mod distribution_tests {
    use crate::client::Rpc;
    use crate::error::CosmosClient;

    #[tokio::test]
    async fn params() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let params = client.distribution.params().await?;
        assert!(params.params.is_some());
        let params = params.params.unwrap();
        assert!(params.withdraw_addr_enabled);
        assert_eq!(params.base_proposer_reward, "860000000000000000");
        assert_eq!(params.community_tax, "100000000000000000");
        assert_eq!(params.bonus_proposer_reward, "40000000000000000");
        Ok(())
    }

    #[tokio::test]
    async fn community_pool() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let pool = client.distribution.community_pool().await?;
        assert!(!pool.pool.is_empty());
        assert_eq!(pool.pool.first().unwrap().denom, "uxki");
        Ok(())
    }

    #[tokio::test]
    async fn validator_outstanding_rewards() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let outstanding_rewards = client
            .distribution
            .validator_outstanding_rewards("kivaloper1y2znqwwcw43zneg6zk0rvadzy6q890m4dhzpsh")
            .await?;
        assert!(outstanding_rewards.rewards.is_some());
        let rewards = outstanding_rewards.rewards.unwrap();
        assert!(!rewards.rewards.is_empty());
        assert_eq!(rewards.rewards[0].denom, "uxki");
        Ok(())
    }
}
