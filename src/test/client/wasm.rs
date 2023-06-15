#[cfg(test)]
mod wasm_tests {
    use crate::client::RpcClient;
    use cosmos_sdk_proto::cosmwasm::wasm::v1::{
        AccessConfig, AccessType, ContractCodeHistoryOperationType,
    };
    use serde::{Deserialize, Serialize};

    #[tokio::test]
    async fn contract() -> Result<(), CosmosClientError> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let contrat = client
            .wasm
            .contract_info("ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gqw2adht")
            .await?;
        assert_eq!(
            contrat.address,
            "ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gqw2adht"
        );

        assert!(contrat.contract_info.is_some());
        let contract_info = contrat.contract_info.unwrap();

        assert!(contract_info.code_id > 38);
        assert_eq!(contract_info.label, "CosmonDeck");
        assert_eq!(
            contract_info.admin,
            "ki1cfy5hq7n35et7geqkc2d3xxjz6sl8dp8p5yejl"
        );
        assert_eq!(
            contract_info.creator,
            "ki1cfy5hq7n35et7geqkc2d3xxjz6sl8dp8p5yejl"
        );
        assert_eq!(contract_info.created, None);
        assert_eq!(contract_info.ibc_port_id, "");
        Ok(())
    }

    #[tokio::test]
    async fn bad_contract() -> Result<(), CosmosClientError> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let contrat = client
            .wasm
            .contract_info("ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59")
            .await?;
        assert_eq!(contrat.contract_info, None);

        Ok(())
    }

    #[tokio::test]
    async fn contract_history() -> Result<(), CosmosClientError> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let history = client
            .wasm
            .contract_history(
                "ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gqw2adht",
                None,
            )
            .await?;
        assert!(history.entries.len() >= 7);
        assert_eq!(
            ContractCodeHistoryOperationType::from_i32(history.entries[0].operation),
            ContractCodeHistoryOperationType::Init.into()
        );
        assert_eq!(history.entries[0].code_id, 6);
        assert_eq!(history.entries[0].updated, None);
        assert_eq!(
            ContractCodeHistoryOperationType::from_i32(history.entries[0].operation),
            ContractCodeHistoryOperationType::Init.into()
        );
        assert_eq!(history.entries[1].code_id, 13);
        assert_eq!(history.entries[1].updated, None);
        assert_eq!(
            ContractCodeHistoryOperationType::from_i32(history.entries[1].operation),
            ContractCodeHistoryOperationType::Migrate.into()
        );
        assert_eq!(history.entries[2].code_id, 14);
        assert_eq!(history.entries[2].updated, None);
        assert_eq!(
            ContractCodeHistoryOperationType::from_i32(history.entries[2].operation),
            ContractCodeHistoryOperationType::Migrate.into()
        );
        assert_eq!(history.entries[3].code_id, 19);
        assert_eq!(history.entries[3].updated, None);
        assert_eq!(
            ContractCodeHistoryOperationType::from_i32(history.entries[3].operation),
            ContractCodeHistoryOperationType::Migrate.into()
        );
        assert_eq!(history.entries[4].code_id, 25);
        assert_eq!(history.entries[4].updated, None);
        assert_eq!(
            ContractCodeHistoryOperationType::from_i32(history.entries[4].operation),
            ContractCodeHistoryOperationType::Migrate.into()
        );
        assert_eq!(history.entries[5].code_id, 33);
        assert_eq!(history.entries[5].updated, None);
        assert_eq!(
            ContractCodeHistoryOperationType::from_i32(history.entries[5].operation),
            ContractCodeHistoryOperationType::Migrate.into()
        );
        assert_eq!(history.entries[6].code_id, 38);
        assert_eq!(history.entries[6].updated, None);
        Ok(())
    }

    #[tokio::test]
    async fn contract_by_code_id() -> Result<(), CosmosClientError> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let contracts = client.wasm.contracts_by_code(35, None).await?;
        assert!(!contracts.contracts.is_empty());
        assert_eq!(
            contracts.contracts[0],
            "ki1ghyk8wtwnhu3pjtercnxr9ks9zmqz9j7y0d6xf9kwee4ctc4qz7qy9yym2"
        );
        Ok(())
    }

    #[tokio::test]
    async fn code() -> Result<(), CosmosClientError> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let code = client.wasm.code(35).await?;
        assert!(code.code_info.is_some());
        let code_info = code.code_info.unwrap();
        assert_eq!(code_info.code_id, 35);
        assert_eq!(
            code_info.creator,
            "ki12u4jtcczpg2m3nt50muh3srte7zed77qsfyng4"
        );
        assert_eq!(
            code_info.data_hash,
            vec![
                164, 217, 207, 99, 119, 240, 90, 91, 189, 183, 230, 83, 80, 12, 164, 21, 239, 186,
                72, 239, 250, 191, 162, 232, 210, 60, 207, 52, 77, 147, 42, 135
            ]
        );
        assert_eq!(
            code_info.instantiate_permission,
            Some(AccessConfig {
                permission: AccessType::Everybody.into(),
                address: "".to_string(),
                addresses: vec![]
            })
        );
        Ok(())
    }

    use crate::error::CosmosClientError;
    use cosmos_sdk_proto::cosmwasm::wasm::v1::AccessType::Everybody;
    use std::str;

    #[tokio::test]
    async fn codes() -> Result<(), CosmosClientError> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let codes = client.wasm.codes(None).await?;

        assert!(codes.code_infos.len() > 30);
        assert_eq!(codes.code_infos[22].code_id, 23);
        assert_eq!(
            codes.code_infos[22].data_hash,
            vec![
                74, 31, 129, 202, 51, 19, 182, 240, 209, 63, 128, 219, 51, 95, 238, 244, 239, 190,
                135, 20, 20, 149, 223, 54, 13, 189, 44, 113, 161, 139, 197, 149
            ]
        );
        assert_eq!(
            codes.code_infos[22].instantiate_permission,
            Some(AccessConfig {
                permission: Everybody.into(),
                address: "".to_string(),
                addresses: vec![]
            })
        );
        assert_eq!(
            codes.code_infos[22].creator,
            "ki12u4jtcczpg2m3nt50muh3srte7zed77qsfyng4"
        );
        Ok(())
    }

    #[tokio::test]
    async fn pinned_codes() -> Result<(), CosmosClientError> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let codes = client.wasm.pinned_codes(None).await?;
        assert!(codes.code_ids.is_empty());
        Ok(())
    }

    #[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        GetMaxDeckByAddress {},
    }

    #[tokio::test]
    async fn query_smart() -> Result<(), CosmosClientError> {
        let client = RpcClient::new("https://rpc-kichain-ia.cosmosia.notional.ventures/")?;

        let max_decks: u64 = client
            .wasm
            .smart_contract_state(
                "ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gqw2adht",
                QueryMsg::GetMaxDeckByAddress {},
            )
            .await?;
        assert_eq!(max_decks, 50u64);
        Ok(())
    }
}
