#[cfg(test)]
mod gov_tests {
    use crate::client::Rpc;
    use crate::error::CosmosClient;
    use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
    use cosmos_sdk_proto::cosmos::gov::v1beta1::{
        DepositParams, QueryParamsResponse, TallyParams, TallyResult, VotingParams,
    };
    use prost_types::Duration;
    use std::str::FromStr;

    #[tokio::test]
    async fn proposals() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let proposals = client.gov.proposals(0, "", "", None).await?;
        assert!(proposals.proposals.len() >= 2);
        Ok(())
    }

    #[tokio::test]
    async fn proposal() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let proposal = client.gov.proposal(1).await?;
        assert!(proposal.proposal.is_some());
        let proposal = proposal.proposal.unwrap();
        assert_eq!(proposal.proposal_id, 1);
        assert_eq!(proposal.status, 3);
        assert_eq!(
            proposal.final_tally_result,
            Some(TallyResult {
                yes: "430961562446809".to_string(),
                no: "0".to_string(),
                abstain: "0".to_string(),
                no_with_veto: "0".to_string(),
            })
        );
        assert_eq!(
            proposal.deposit_end_time,
            Some(prost_types::Timestamp::from_str(
                "2022-06-17T13:08:32.166135052Z"
            )?)
        );
        assert_eq!(
            proposal.submit_time,
            Some(prost_types::Timestamp::from_str(
                "2022-06-03T13:08:32.166135052Z"
            )?)
        );
        assert_eq!(
            proposal.total_deposit,
            [Coin {
                denom: "uxki".to_string(),
                amount: 1_000_001_000.to_string(),
            }]
        );
        assert_eq!(
            proposal.voting_end_time,
            Some(prost_types::Timestamp::from_str(
                "2022-06-10T13:53:39.514058422Z"
            )?)
        );
        assert_eq!(
            proposal.voting_start_time,
            Some(prost_types::Timestamp::from_str(
                "2022-06-03T13:53:39.514058422Z"
            )?)
        );
        Ok(())
    }

    #[tokio::test]
    async fn votes() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let votes = client.gov.votes(2, None).await?;
        assert!(votes.votes.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn params() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let tallying = client.gov.params("tallying").await?;
        let deposit = client.gov.params("deposit").await?;
        let voting = client.gov.params("voting").await?;

        assert_eq!(
            tallying,
            QueryParamsResponse {
                voting_params: Some(VotingParams {
                    voting_period: Some(Duration {
                        seconds: 0,
                        nanos: 0
                    }),
                }),
                deposit_params: Some(DepositParams {
                    min_deposit: vec![],
                    max_deposit_period: Some(Duration {
                        seconds: 0,
                        nanos: 0
                    }),
                }),
                tally_params: Some(TallyParams {
                    quorum: vec![
                        51, 51, 52, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48
                    ],
                    threshold: vec![
                        53, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48
                    ],
                    veto_threshold: vec![
                        51, 51, 52, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48
                    ],
                }),
            }
        );
        assert_eq!(
            voting,
            QueryParamsResponse {
                voting_params: Some(VotingParams {
                    voting_period: Some(Duration {
                        seconds: 259_200,
                        nanos: 0
                    }),
                }),
                deposit_params: Some(DepositParams {
                    min_deposit: vec![],
                    max_deposit_period: Some(Duration {
                        seconds: 0,
                        nanos: 0
                    }),
                }),
                tally_params: Some(TallyParams {
                    quorum: vec![48],
                    threshold: vec![48],
                    veto_threshold: vec![48],
                }),
            }
        );
        assert_eq!(
            deposit,
            QueryParamsResponse {
                voting_params: Some(VotingParams {
                    voting_period: Some(Duration {
                        seconds: 0,
                        nanos: 0
                    }),
                }),
                deposit_params: Some(DepositParams {
                    min_deposit: vec![Coin {
                        denom: "uxki".to_string(),
                        amount: 500_000_000_000u128.to_string(),
                    },],
                    max_deposit_period: Some(Duration {
                        seconds: 172_800,
                        nanos: 0
                    }),
                }),
                tally_params: Some(TallyParams {
                    quorum: vec![48],
                    threshold: vec![48],
                    veto_threshold: vec![48],
                }),
            }
        );
        Ok(())
    }

    #[tokio::test]
    async fn deposit() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        assert!(client
            .gov
            .deposit(1, "ki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8usr5tl8")
            .await
            .is_err());
        Ok(())
    }

    #[tokio::test]
    async fn deposits() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let deposits = client.gov.deposits(1, None).await?;
        assert!(deposits.deposits.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn tally() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let tally = client.gov.tally_result(2).await?;
        assert!(tally.tally.is_some());
        let tally = tally.tally.unwrap();
        assert_eq!(tally.yes, "513662868572639");
        assert_eq!(tally.no, "0");
        assert_eq!(tally.abstain, "0");
        assert_eq!(tally.no_with_veto, "0");
        Ok(())
    }
}
