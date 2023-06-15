#[cfg(test)]
mod feegrant_tests {
    use crate::client::any_helper::{any_to_cosmos, CosmosType};
    use crate::client::RpcClient;
    use crate::error::CosmosClientError;
    use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
    use std::str::FromStr;

    #[tokio::test]
    async fn allowance() -> Result<(), CosmosClientError> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let allowance = client
            .feegrant
            .allowance(
                "ki1f2q7swkt8lexl2trkl88g5kc5lhmxf0tqvlrm8",
                "ki10ztpacs9u2retxkr2e2z5gsdrhw86c0gl26tsd",
            )
            .await?;

        assert!(allowance.allowance.is_some());
        let allowance = allowance.allowance.unwrap();
        assert_eq!(
            allowance.granter,
            "ki1f2q7swkt8lexl2trkl88g5kc5lhmxf0tqvlrm8"
        );
        assert_eq!(
            allowance.grantee,
            "ki10ztpacs9u2retxkr2e2z5gsdrhw86c0gl26tsd"
        );

        assert!(allowance.allowance.is_some());
        let allowance = allowance.allowance.unwrap();
        let allowance = any_to_cosmos(&allowance)?;

        match allowance {
            CosmosType::BasicAllowance(allowance) => {
                assert_eq!(
                    allowance.expiration,
                    Some(prost_types::Timestamp::from_str("2022-10-30T15:04:05Z")?)
                );
                assert_eq!(
                    allowance.spend_limit,
                    vec![Coin {
                        denom: "utki".to_string(),
                        amount: "1000".to_string()
                    }]
                );
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}
