#[cfg(test)]
mod auth_tests {
    use crate::client::any_helper::{any_to_cosmos, CosmosType};
    use crate::client::RpcClient;

    #[tokio::test]
    async fn accounts() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-challenge.blockchain.ki")?;
        let accounts = client.auth.accounts(None).await?;

        assert!(accounts.pagination.is_some());
        assert!(accounts.pagination.unwrap().total > 100);
        Ok(())
    }

    #[tokio::test]
    async fn account() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-challenge.blockchain.ki")?;
        let account = client
            .auth
            .account("tki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8u0n7zj3")
            .await?;

        assert!(account.account.is_some());
        let decoded_account = any_to_cosmos(&account.account.unwrap())?;

        match decoded_account {
            CosmosType::BaseAccount(account) => {
                assert_eq!(account.account_number, 2850);
                assert_eq!(
                    account.address,
                    "tki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8u0n7zj3"
                );
                assert!(account.sequence > 0);
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    #[tokio::test]
    async fn params() -> Result<(), anyhow::Error> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;
        let params = client.auth.params().await?;

        assert!(params.params.is_some());

        let params = params.params.unwrap();
        assert_eq!(params.sig_verify_cost_secp256k1, 1000);
        assert_eq!(params.max_memo_characters, 512);
        assert_eq!(params.tx_sig_limit, 7);
        assert_eq!(params.sig_verify_cost_ed25519, 590);
        assert_eq!(params.tx_size_cost_per_byte, 10);

        Ok(())
    }
}
