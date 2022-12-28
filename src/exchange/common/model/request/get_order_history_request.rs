use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use super::{OrderStatus, Paginator, MarketPair};

/// This struct represents the historic of the orders
#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetOrderHistoryRequest {
    pub market_pair: Option<MarketPair>,
    pub order_status: Option<Vec<OrderStatus>>,
    pub paginator: Option<Paginator>,
}