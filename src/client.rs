pub mod any_helper;
pub mod auth;
pub mod authz;
pub mod bank;

use crate::client::auth::AuthModule;
use crate::client::bank::BankModule;

use crate::client::authz::AuthzModule;
use std::rc::Rc;
use tendermint_rpc::HttpClient;

pub struct RpcClient {
    pub bank: BankModule,
    pub auth: AuthModule,
    pub authz: AuthzModule,
}

impl RpcClient {
    pub fn new(url: &str) -> Result<Self, anyhow::Error> {
        let rpc = Rc::new(HttpClient::new(url)?);

        Ok(RpcClient {
            bank: BankModule::new(rpc.clone()),
            auth: AuthModule::new(rpc.clone()),
            authz: AuthzModule::new(rpc),
        })
    }
}
