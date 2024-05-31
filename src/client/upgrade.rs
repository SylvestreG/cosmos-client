use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::upgrade::v1beta1::{
    QueryAppliedPlanResponse, QueryCurrentPlanRequest, QueryCurrentPlanResponse,
    QueryModuleVersionsRequest, QueryModuleVersionsResponse, QueryUpgradedConsensusStateRequest,
    QueryUpgradedConsensusStateResponse,
};
use prost::Message;
use std::sync::Arc;
use tendermint::abci::Code;
use tendermint_rpc::{Client, HttpClient};

pub struct Module {
    rpc: Arc<HttpClient>,
}

impl Module {
    pub fn new(rpc: Arc<HttpClient>) -> Self {
        Module { rpc }
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn current_plan(&self) -> Result<QueryCurrentPlanResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryCurrentPlanResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn applied_plan(
        &self,
        height: i64,
    ) -> Result<QueryAppliedPlanResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryAppliedPlanResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn upgrade_consensus_state(
        &self,
        last_height: i64,
    ) -> Result<QueryUpgradedConsensusStateResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryUpgradedConsensusStateResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn module_versions(
        &self,
        module_name: &str,
    ) -> Result<QueryModuleVersionsResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryModuleVersionsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
