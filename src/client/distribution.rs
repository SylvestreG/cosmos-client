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

    pub async fn params(&self) -> Result<QueryParamsResponse, anyhow::Error> {
        let query = QueryParamsRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/Params".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryParamsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn validator_outstanding_rewards(
        &self,
        validator_address: &str,
    ) -> Result<QueryValidatorOutstandingRewardsResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryValidatorOutstandingRewardsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn validator_commission(
        &self,
        validator_address: &str,
    ) -> Result<QueryValidatorCommissionResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryValidatorCommissionResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn validator_slashes(
        &self,
        validator_address: &str,
        starting_height: u64,
        ending_height: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryValidatorSlashesResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryValidatorSlashesResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn delegation_rewards(
        &self,
        delegator_addr: &str,
        validator_addr: &str,
    ) -> Result<QueryDelegationResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryDelegationResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn delegation_total_rewards(
        &self,
        delegator_address: &str,
    ) -> Result<QueryDelegationTotalRewardsResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryDelegationTotalRewardsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn delegator_validators(
        &self,
        delegator_addr: &str,
    ) -> Result<QueryDelegatorValidatorsResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryDelegatorValidatorsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn delegator_withdraw_address(
        &self,
        delegator_addr: &str,
    ) -> Result<QueryDelegatorWithdrawAddressResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryDelegatorWithdrawAddressResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn community_pool(&self) -> Result<QueryCommunityPoolResponse, anyhow::Error> {
        let query = QueryCommunityPoolRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.distribution.v1beta1.Query/CommunityPool".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryCommunityPoolResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }
}
