use cosmos_sdk_proto::cosmos::upgrade::v1beta1::{
    QueryAppliedPlanResponse, QueryCurrentPlanRequest, QueryCurrentPlanResponse,
    QueryModuleVersionsRequest, QueryModuleVersionsResponse, QueryUpgradedConsensusStateRequest,
    QueryUpgradedConsensusStateResponse,
};
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct UpgradeModule {
    rpc: Rc<HttpClient>,
}

impl UpgradeModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        UpgradeModule { rpc }
    }

    pub async fn current_plan(&self) -> Result<QueryCurrentPlanResponse, anyhow::Error> {
        let query = QueryCurrentPlanRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.upgrade.v1beta1.Query/CurrentPlan".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryCurrentPlanResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn applied_plan(
        &self,
        height: i64,
    ) -> Result<QueryAppliedPlanResponse, anyhow::Error> {
        let query = QueryAppliedPlanResponse { height };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.upgrade.v1beta1.Query/AppliedPlan".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryAppliedPlanResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn upgrade_consensus_state(
        &self,
        last_height: i64,
    ) -> Result<QueryUpgradedConsensusStateResponse, anyhow::Error> {
        let query = QueryUpgradedConsensusStateRequest { last_height };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.upgrade.v1beta1.Query/UpgradedConsensusState".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryUpgradedConsensusStateResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn module_versions(
        &self,
        module_name: &str,
    ) -> Result<QueryModuleVersionsResponse, anyhow::Error> {
        let query = QueryModuleVersionsRequest {
            module_name: module_name.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.upgrade.v1beta1.Query/ModuleVersions".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryModuleVersionsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }
}
