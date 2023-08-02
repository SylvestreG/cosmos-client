use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{
    QueryDelegationRequest, QueryDelegationResponse, QueryDelegatorDelegationsRequest,
    QueryDelegatorDelegationsResponse, QueryDelegatorUnbondingDelegationsRequest,
    QueryDelegatorUnbondingDelegationsResponse, QueryDelegatorValidatorRequest,
    QueryDelegatorValidatorResponse, QueryDelegatorValidatorsResponse, QueryHistoricalInfoRequest,
    QueryHistoricalInfoResponse, QueryParamsRequest, QueryParamsResponse, QueryPoolRequest,
    QueryPoolResponse, QueryRedelegationsRequest, QueryRedelegationsResponse,
    QueryUnbondingDelegationRequest, QueryUnbondingDelegationResponse,
    QueryValidatorDelegationsRequest, QueryValidatorDelegationsResponse, QueryValidatorRequest,
    QueryValidatorResponse, QueryValidatorUnbondingDelegationsRequest,
    QueryValidatorUnbondingDelegationsResponse, QueryValidatorsRequest, QueryValidatorsResponse,
};
use tendermint::abci::Code;
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
    pub async fn validator(
        &self,
        validator_addr: &str,
    ) -> Result<QueryValidatorResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryValidatorResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn validators(
        &self,
        status: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryValidatorsResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryValidatorsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn validator_delegations(
        &self,
        validator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryValidatorDelegationsResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryValidatorDelegationsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn validator_unbonding_delegations(
        &self,
        validator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryValidatorUnbondingDelegationsResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryValidatorUnbondingDelegationsResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn delegation(
        &self,
        delegator_addr: &str,
        validator_addr: &str,
    ) -> Result<QueryDelegationResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDelegationResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn unbonding_delegation(
        &self,
        delegator_addr: &str,
        validator_addr: &str,
    ) -> Result<QueryUnbondingDelegationResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryUnbondingDelegationResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn delegator_delegations(
        &self,
        delegator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDelegatorDelegationsResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDelegatorDelegationsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn delegator_unbonding_delegations(
        &self,
        delegator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDelegatorUnbondingDelegationsResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDelegatorUnbondingDelegationsResponse::decode(query.value.as_slice())
            .map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn redelegations(
        &self,
        delegator_addr: &str,
        src_validator_addr: &str,
        dst_validator_addr: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryRedelegationsResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryRedelegationsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
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
        pagination: Option<PageRequest>,
    ) -> Result<QueryDelegatorValidatorsResponse, CosmosClient> {
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
    pub async fn delegator_validator(
        &self,
        delegator_addr: &str,
        validator_addr: &str,
    ) -> Result<QueryDelegatorValidatorResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDelegatorValidatorResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn historical_info(
        &self,
        height: i64,
    ) -> Result<QueryHistoricalInfoResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryHistoricalInfoResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn pool(&self) -> Result<QueryPoolResponse, CosmosClient> {
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

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryPoolResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
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
                Some("/cosmos.staking.v1beta1.Query/Params".to_string()),
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
}
