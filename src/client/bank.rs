use crate::error::CosmosClientError;
use crate::error::CosmosClientError::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::bank::v1beta1::{
    MsgSend, QueryAllBalancesRequest, QueryAllBalancesResponse, QueryBalanceRequest,
    QueryBalanceResponse, QueryDenomMetadataRequest, QueryDenomMetadataResponse,
    QueryDenomsMetadataRequest, QueryDenomsMetadataResponse, QueryParamsRequest,
    QueryParamsResponse, QuerySpendableBalancesRequest, QuerySpendableBalancesResponse,
    QuerySupplyOfRequest, QuerySupplyOfResponse, QueryTotalSupplyRequest, QueryTotalSupplyResponse,
};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmrs::tendermint::abci::Code;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct BankModule {
    rpc: Rc<HttpClient>,
}

impl BankModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        BankModule { rpc }
    }

    pub async fn balance(
        &self,
        address: &str,
        denom: &str,
    ) -> Result<QueryBalanceResponse, CosmosClientError> {
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

    pub async fn all_balances(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAllBalancesResponse, CosmosClientError> {
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

    pub async fn spendable_balances(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QuerySpendableBalancesResponse, CosmosClientError> {
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

    pub async fn total_supply(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryTotalSupplyResponse, CosmosClientError> {
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

    pub async fn supply_of(&self, denom: &str) -> Result<QuerySupplyOfResponse, CosmosClientError> {
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

    pub async fn params(&self) -> Result<QueryParamsResponse, CosmosClientError> {
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

    pub async fn denom_metadata(
        &self,
        denom: &str,
    ) -> Result<QueryDenomMetadataResponse, CosmosClientError> {
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

    pub async fn denoms_metadata(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDenomsMetadataResponse, CosmosClientError> {
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

    pub fn msg_send_payload(from: &str, to: &str) -> Vec<u8> {
        MsgSend::encode_to_vec(&MsgSend {
            from_address: from.to_string(),
            to_address: to.to_string(),
            amount: vec![],
        })
    }
}
