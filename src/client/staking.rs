use crate::error::CosmosClientError;
use crate::error::CosmosClientError::ProstDecodeError;
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{
    QueryDelegationRequest, QueryDelegationResponse, QueryDelegatorDelegationsRequest,
    QueryDelegatorDelegationsResponse, QueryDelegatorUnbondingDelegationsRequest,
    QueryDelegatorUnbondingDelegationsResponse, QueryDelegatorValidatorRequest,
    QueryDelegatorValidatorResponse, QueryHistoricalInfoRequest, QueryHistoricalInfoResponse,
    QueryParamsRequest, QueryParamsResponse, QueryPoolRequest, QueryPoolResponse,
    QueryRedelegationsRequest, QueryRedelegationsResponse, QueryUnbondingDelegationRequest,
    QueryUnbondingDelegationResponse, QueryValidatorDelegationsRequest,
    QueryValidatorDelegationsResponse, QueryValidatorRequest, QueryValidatorResponse,
    QueryValidatorUnbondingDelegationsRequest, QueryValidatorUnbondingDelegationsResponse,
    QueryValidatorsRequest, QueryValidatorsResponse,
};
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct StakingModule {
    rpc: Rc<HttpClient>,
}

impl StakingModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        StakingModule { rpc }
    }

    pub async fn validator(
        &self,
        validator_addr: &str,
    ) -> Result<QueryValidatorResponse, CosmosClientError> {
        let query = QueryValidatorRequest {
            validator_addr: validator_addr.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/Validators".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryValidatorResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn validators(
        &self,
        status: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryValidatorsResponse, CosmosClientError> {
        let query = QueryValidatorsRequest {
            status: status.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/Validator".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryValidatorsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn validator_delegations(
        &self,
        validator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryValidatorDelegationsResponse, CosmosClientError> {
        let query = QueryValidatorDelegationsRequest {
            validator_addr: validator_addr.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/ValidatorDelegations".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryValidatorDelegationsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn validator_unbonding_delegations(
        &self,
        validator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryValidatorUnbondingDelegationsResponse, CosmosClientError> {
        let query = QueryValidatorUnbondingDelegationsRequest {
            validator_addr: validator_addr.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/ValidatorUnbondingDelegations".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryValidatorUnbondingDelegationsResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    pub async fn delegation(
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
                Some("/cosmos.staking.v1beta1.Query/Delegation".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryDelegationResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn unbonding_delegation(
        &self,
        delegator_addr: &str,
        validator_addr: &str,
    ) -> Result<QueryUnbondingDelegationResponse, CosmosClientError> {
        let query = QueryUnbondingDelegationRequest {
            delegator_addr: delegator_addr.to_string(),
            validator_addr: validator_addr.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/UnbondingDelegation".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryUnbondingDelegationResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn delegator_delegations(
        &self,
        delegator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDelegatorDelegationsResponse, CosmosClientError> {
        let query = QueryDelegatorDelegationsRequest {
            delegator_addr: delegator_addr.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/DelegatorDelegations".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryDelegatorDelegationsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn delegator_unbonding_delegations(
        &self,
        delegator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDelegatorUnbondingDelegationsResponse, CosmosClientError> {
        let query = QueryDelegatorUnbondingDelegationsRequest {
            delegator_addr: delegator_addr.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/DelegatorUnbondingDelegations".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryDelegatorUnbondingDelegationsResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    pub async fn redelegations(
        &self,
        delegator_addr: &str,
        src_validator_addr: &str,
        dst_validator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryRedelegationsResponse, CosmosClientError> {
        let query = QueryRedelegationsRequest {
            delegator_addr: delegator_addr.to_string(),
            src_validator_addr: src_validator_addr.to_string(),
            dst_validator_addr: dst_validator_addr.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/Redelegations".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryRedelegationsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn delegator_validators(
        &self,
        delegator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDelegatorDelegationsResponse, CosmosClientError> {
        let query = QueryDelegatorDelegationsRequest {
            delegator_addr: delegator_addr.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/DelegatorValidators".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryDelegatorDelegationsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn delegator_validator(
        &self,
        delegator_addr: &str,
        validator_addr: &str,
    ) -> Result<QueryDelegatorValidatorResponse, CosmosClientError> {
        let query = QueryDelegatorValidatorRequest {
            delegator_addr: delegator_addr.to_string(),
            validator_addr: validator_addr.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/DelegatorValidator".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryDelegatorValidatorResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn historical_info(
        &self,
        height: i64,
    ) -> Result<QueryHistoricalInfoResponse, CosmosClientError> {
        let query = QueryHistoricalInfoRequest { height };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/HistoricalInfo".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryHistoricalInfoResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn pool(&self) -> Result<QueryPoolResponse, CosmosClientError> {
        let query = QueryPoolRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/Pool".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryPoolResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn params(&self) -> Result<QueryParamsResponse, CosmosClientError> {
        let query = QueryParamsRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.staking.v1beta1.Query/Params".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        QueryParamsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
