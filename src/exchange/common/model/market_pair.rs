use std::fmt;
use serde::{Serialize, Deserialize};
pub use super::currency::Currency;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct MarketPair(pub Currency, pub Currency);

impl MarketPair {
    pub fn inverse(&self) -> MarketPair {
        MarketPair(self.1.clone(), self.0.clone())
    }
}

impl fmt::Display for MarketPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}


impl fmt::Debug for MarketPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}