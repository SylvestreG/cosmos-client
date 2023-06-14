use std::rc::Rc;
use tendermint_rpc::HttpClient;

pub struct TxModule {
    _rpc: Rc<HttpClient>,
}

impl TxModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        TxModule { _rpc: rpc }
    }
}
