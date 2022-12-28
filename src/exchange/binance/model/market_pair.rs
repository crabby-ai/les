use crate::exchange::common::model::MarketPair as OMarketPair;
use crate::errors::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketPair(pub String);

impl From<OMarketPair> for MarketPair {
    fn from(from: OMarketPair) -> MarketPair {
        MarketPair(format!("{}{}", from.0, from.1).to_uppercase())
    }
}