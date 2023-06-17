use cosmrs::tx::{Body, BodyBuilder};
use prost_types::Any;

pub struct CosmosTx {
    tx: BodyBuilder,
}

impl CosmosTx {
    pub fn build() -> Self {
        CosmosTx {
            tx: BodyBuilder::new(),
        }
    }

    pub fn memo(mut self, memo: &str) -> Self {
        self.tx.memo(memo.to_string());
        self
    }

    pub fn add_msg(mut self, msg: Any) -> Self {
        self.tx.msg(msg);
        self
    }

    pub fn finish(&self) -> Body {
        self.tx.finish()
    }
}
