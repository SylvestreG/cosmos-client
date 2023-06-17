use crate::error::CosmosClientError;
use crate::error::CosmosClientError::ProstDecodeError;
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

    pub async fn current_plan(&self) -> Result<QueryCurrentPlanResponse, CosmosClientError> {
        let query = QueryCurrentPlanRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.upgrade.v1beta1.Query/CurrentPlan".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryCurrentPlanResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn applied_plan(
        &self,
        height: i64,
    ) -> Result<QueryAppliedPlanResponse, CosmosClientError> {
        let query = QueryAppliedPlanResponse { height };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.upgrade.v1beta1.Query/AppliedPlan".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryAppliedPlanResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn upgrade_consensus_state(
        &self,
        last_height: i64,
    ) -> Result<QueryUpgradedConsensusStateResponse, CosmosClientError> {
        let query = QueryUpgradedConsensusStateRequest { last_height };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.upgrade.v1beta1.Query/UpgradedConsensusState".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryUpgradedConsensusStateResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    pub async fn module_versions(
        &self,
        module_name: &str,
    ) -> Result<QueryModuleVersionsResponse, CosmosClientError> {
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
            .await?;

        QueryModuleVersionsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
