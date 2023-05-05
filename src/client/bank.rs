use cosmos_sdk_proto::cosmos::bank::v1beta1::{
    QueryAllBalancesRequest, QueryAllBalancesResponse, QueryBalanceRequest, QueryBalanceResponse,
    QueryDenomMetadataRequest, QueryDenomMetadataResponse, QueryDenomsMetadataRequest,
    QueryDenomsMetadataResponse, QueryParamsRequest, QueryParamsResponse,
    QuerySpendableBalancesRequest, QuerySpendableBalancesResponse, QuerySupplyOfRequest,
    QuerySupplyOfResponse, QueryTotalSupplyRequest, QueryTotalSupplyResponse,
};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
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
    ) -> Result<QueryBalanceResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryBalanceResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn all_balances(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAllBalancesResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryAllBalancesResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn spendable_balances(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QuerySpendableBalancesResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QuerySpendableBalancesResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn total_supply(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryTotalSupplyResponse, anyhow::Error> {
        let query = QueryTotalSupplyRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/TotalSupply".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryTotalSupplyResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn supply_of(&self, denom: &str) -> Result<QuerySupplyOfResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QuerySupplyOfResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn params(&self) -> Result<QueryParamsResponse, anyhow::Error> {
        let query = QueryParamsRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/Params".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryParamsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn denom_metadata(
        &self,
        denom: &str,
    ) -> Result<QueryDenomMetadataResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryDenomMetadataResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn denoms_metadata(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDenomsMetadataResponse, anyhow::Error> {
        let query = QueryDenomsMetadataRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.bank.v1beta1.Query/DenomsMetadata".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryDenomsMetadataResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }
}
