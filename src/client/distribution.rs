use crate::error::CosmosClientError;
use crate::error::CosmosClientError::ProstDecodeError;
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::distribution::v1beta1::{
    QueryCommunityPoolRequest, QueryCommunityPoolResponse, QueryDelegationTotalRewardsRequest,
    QueryDelegationTotalRewardsResponse, QueryDelegatorValidatorsRequest,
    QueryDelegatorValidatorsResponse, QueryDelegatorWithdrawAddressRequest,
    QueryDelegatorWithdrawAddressResponse, QueryParamsRequest, QueryParamsResponse,
    QueryValidatorCommissionRequest, QueryValidatorCommissionResponse,
    QueryValidatorOutstandingRewardsRequest, QueryValidatorOutstandingRewardsResponse,
    QueryValidatorSlashesRequest, QueryValidatorSlashesResponse,
};
use cosmos_sdk_proto::cosmos::staking::v1beta1::{QueryDelegationRequest, QueryDelegationResponse};
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct DistributionModule {
    rpc: Rc<HttpClient>,
}

impl DistributionModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        DistributionModule { rpc }
    }

    pub async fn params(&self) -> Result<QueryParamsResponse, CosmosClientError> {
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

        QueryParamsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn validator_outstanding_rewards(
        &self,
        validator_address: &str,
    ) -> Result<QueryValidatorOutstandingRewardsResponse, CosmosClientError> {
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

        QueryValidatorOutstandingRewardsResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    pub async fn validator_commission(
        &self,
        validator_address: &str,
    ) -> Result<QueryValidatorCommissionResponse, CosmosClientError> {
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

        QueryValidatorCommissionResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn validator_slashes(
        &self,
        validator_address: &str,
        starting_height: u64,
        ending_height: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryValidatorSlashesResponse, CosmosClientError> {
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

        QueryValidatorSlashesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn delegation_rewards(
        &self,
        delegator_addr: &str,
        validator_addr: &str,
    ) -> Result<QueryDelegationResponse, CosmosClientError> {
        let query = QueryDelegationRequest {
            delegator_addr: delegator_addr.to_string(),
            validator_addr: validator_addr.to_string(),
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

        QueryDelegationResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn delegation_total_rewards(
        &self,
        delegator_address: &str,
    ) -> Result<QueryDelegationTotalRewardsResponse, CosmosClientError> {
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

        QueryDelegationTotalRewardsResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    pub async fn delegator_validators(
        &self,
        delegator_addr: &str,
    ) -> Result<QueryDelegatorValidatorsResponse, CosmosClientError> {
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

        QueryDelegatorValidatorsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn delegator_withdraw_address(
        &self,
        delegator_addr: &str,
    ) -> Result<QueryDelegatorWithdrawAddressResponse, CosmosClientError> {
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

        QueryDelegatorWithdrawAddressResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    pub async fn community_pool(&self) -> Result<QueryCommunityPoolResponse, CosmosClientError> {
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

        QueryCommunityPoolResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
