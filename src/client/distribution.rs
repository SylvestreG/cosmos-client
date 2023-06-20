use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::distribution::v1beta1::{
    QueryCommunityPoolRequest, QueryCommunityPoolResponse, QueryDelegationRewardsRequest,
    QueryDelegationRewardsResponse, QueryDelegationTotalRewardsRequest,
    QueryDelegationTotalRewardsResponse, QueryDelegatorValidatorsRequest,
    QueryDelegatorValidatorsResponse, QueryDelegatorWithdrawAddressRequest,
    QueryDelegatorWithdrawAddressResponse, QueryParamsRequest, QueryParamsResponse,
    QueryValidatorCommissionRequest, QueryValidatorCommissionResponse,
    QueryValidatorOutstandingRewardsRequest, QueryValidatorOutstandingRewardsResponse,
    QueryValidatorSlashesRequest, QueryValidatorSlashesResponse,
};
use cosmrs::tendermint::abci::Code;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct Module {
    rpc: Rc<HttpClient>,
}

impl Module {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        Module { rpc }
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn params(&self) -> Result<QueryParamsResponse, CosmosClient> {
        let query = QueryParamsRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/Params".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryParamsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn validator_outstanding_rewards(
        &self,
        validator_address: &str,
    ) -> Result<QueryValidatorOutstandingRewardsResponse, CosmosClient> {
        let query = QueryValidatorOutstandingRewardsRequest {
            validator_address: validator_address.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/ValidatorOutstandingRewards".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryValidatorOutstandingRewardsResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn validator_commission(
        &self,
        validator_address: &str,
    ) -> Result<QueryValidatorCommissionResponse, CosmosClient> {
        let query = QueryValidatorCommissionRequest {
            validator_address: validator_address.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/ValidatorCommission".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryValidatorCommissionResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn validator_slashes(
        &self,
        validator_address: &str,
        starting_height: u64,
        ending_height: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryValidatorSlashesResponse, CosmosClient> {
        let query = QueryValidatorSlashesRequest {
            validator_address: validator_address.to_string(),
            starting_height,
            ending_height,
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/ValidatorSlashes".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryValidatorSlashesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn delegation_rewards(
        &self,
        delegator_address: &str,
        validator_address: &str,
    ) -> Result<QueryDelegationRewardsResponse, CosmosClient> {
        let query = QueryDelegationRewardsRequest {
            delegator_address: delegator_address.to_string(),
            validator_address: validator_address.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/DelegationRewards".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDelegationRewardsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn delegation_total_rewards(
        &self,
        delegator_address: &str,
    ) -> Result<QueryDelegationTotalRewardsResponse, CosmosClient> {
        let query = QueryDelegationTotalRewardsRequest {
            delegator_address: delegator_address.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/DelegationTotalRewards".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDelegationTotalRewardsResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn delegator_validators(
        &self,
        delegator_addr: &str,
    ) -> Result<QueryDelegatorValidatorsResponse, CosmosClient> {
        let query = QueryDelegatorValidatorsRequest {
            delegator_address: delegator_addr.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/DelegatorValidators".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDelegatorValidatorsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn delegator_withdraw_address(
        &self,
        delegator_addr: &str,
    ) -> Result<QueryDelegatorWithdrawAddressResponse, CosmosClient> {
        let query = QueryDelegatorWithdrawAddressRequest {
            delegator_address: delegator_addr.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/DelegatorWithdrawAddress".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDelegatorWithdrawAddressResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn community_pool(&self) -> Result<QueryCommunityPoolResponse, CosmosClient> {
        let query = QueryCommunityPoolRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/CommunityPool".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryCommunityPoolResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
