use cosmrs::tx::{Body, BodyBuilder};
use prost_types::Any;

pub struct Cosmos {
    tx: BodyBuilder,
}

impl Cosmos {
    #[must_use]
    pub fn build() -> Self {
        Cosmos {
            tx: BodyBuilder::new(),
        }
    }

    #[must_use]
    pub fn memo(mut self, memo: &str) -> Self {
        self.tx.memo(memo.to_string());
        self
    }

    #[must_use]
    pub fn add_msg(mut self, msg: Any) -> Self {
        self.tx.msg(msg);
        self
    }

    #[must_use]
    pub fn finish(&self) -> Body {
        self.tx.finish()
    }
}
