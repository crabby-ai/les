//! This module contains all the implemented exchanges.

pub mod binance;
pub mod common;

pub use common::{traits, shared, model};


// TODO: remove this
// pub use openlimits_coinbase as coinbase;
// pub use openlimits_nash as nash;
//
// pub use openlimits_exchange::traits;
// pub use openlimits_exchange::shared;
// pub use openlimits_exchange::model;
// pub use openlimits_exchange::errors;
