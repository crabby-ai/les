use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::{Serialize, Deserialize};

/// This struct represents the ask and bid
#[derive(Serialize, Deserialize, Copy, Clone, Constructor, Debug, Default, PartialEq)]
pub struct AskBid {
    pub price: Decimal,
    pub qty: Decimal,
}