use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use super::MarketPair;

/// This struct represents an order book request
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct OrderBookRequest {
    pub market_pair: MarketPair,
}