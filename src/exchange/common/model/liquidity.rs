use serde::{Serialize, Deserialize};

/// This enum represents the liquidity
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Liquidity {
    Maker,
    Taker,
}