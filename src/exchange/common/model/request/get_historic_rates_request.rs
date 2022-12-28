use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use super::{Paginator, Interval};
use super::MarketPair;

/// This struct represents the historic of the rates
#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetHistoricRatesRequest {
    pub market_pair: MarketPair,
    pub paginator: Option<Paginator>,
    pub interval: Interval,
}