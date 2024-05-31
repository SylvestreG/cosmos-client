use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::bank::v1beta1::{
    QueryAllBalancesRequest, QueryAllBalancesResponse, QueryBalanceRequest, QueryBalanceResponse,
    QueryDenomMetadataRequest, QueryDenomMetadataResponse, QueryDenomsMetadataRequest,
    QueryDenomsMetadataResponse, QueryParamsRequest, QueryParamsResponse,
    QuerySpendableBalancesRequest, QuerySpendableBalancesResponse, QuerySupplyOfRequest,
    QuerySupplyOfResponse, QueryTotalSupplyRequest, QueryTotalSupplyResponse,
};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
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
    pub async fn balance(
        &self,
        address: &str,
        denom: &str,
    ) -> Result<QueryBalanceResponse, CosmosClient> {
        let query = QueryBalanceRequest {
            address: address.to_string(),
            denom: denom.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/Balance".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryBalanceResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn all_balances(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAllBalancesResponse, CosmosClient> {
        let query = QueryAllBalancesRequest {
            address: address.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/AllBalances".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryAllBalancesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn spendable_balances(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QuerySpendableBalancesResponse, CosmosClient> {
        let query = QuerySpendableBalancesRequest {
            address: address.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/SpendableBalances".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QuerySpendableBalancesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn total_supply(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryTotalSupplyResponse, CosmosClient> {
        let query = QueryTotalSupplyRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/TotalSupply".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryTotalSupplyResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn supply_of(&self, denom: &str) -> Result<QuerySupplyOfResponse, CosmosClient> {
        let query = QuerySupplyOfRequest {
            denom: denom.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/SupplyOf".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QuerySupplyOfResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
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
                Some("/cosmos.bank.v1beta1.Query/Params".to_string()),
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
    pub async fn denom_metadata(
        &self,
        denom: &str,
    ) -> Result<QueryDenomMetadataResponse, CosmosClient> {
        let query = QueryDenomMetadataRequest {
            denom: denom.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/DenomMetadata".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDenomMetadataResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn denoms_metadata(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDenomsMetadataResponse, CosmosClient> {
        let query = QueryDenomsMetadataRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/DenomsMetadata".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDenomsMetadataResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
