#[cfg(test)]
mod evidence_tests {
    use crate::client::any_helper::{any_to_cosmos, CosmosType};
    use crate::client::Rpc;
    use crate::error::CosmosClient;
    use std::str::FromStr;

    #[tokio::test]
    async fn evidence() -> Result<(), CosmosClient> {
        let client = Rpc::new("https://rpc-kichain-ia.cosmosia.notional.ventures/").await?;

        let evidence = client.evidence.all_evidence(None).await?;

        assert_eq!(evidence.pagination.unwrap().total, 2);
        if let CosmosType::Equivocation(evidence) = any_to_cosmos(&evidence.evidence[0])? {
            assert_eq!(evidence.power, 200_723);
            assert_eq!(evidence.height, 11_316_735);
            assert_eq!(
                evidence.time,
                Some(prost_types::Timestamp::from_str(
                    "2022-09-01T22:04:26.317651814Z"
                )?)
            );
            assert_eq!(
                evidence.consensus_address,
                "kivalcons1gljaqtldcvqc2dmwu28pdt4ghed38qyz39ts6s"
            );
        } else {
            unreachable!();
        }
        Ok(())
    }
}
