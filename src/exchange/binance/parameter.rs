use super::BinanceCredentials;
use crate::exchange::common::Environment;

/// This struct represents the type of environment that will be used and receives a boolean and the credentials as parameters.
#[derive(Default, Clone, Debug)]
pub struct BinanceParameters {
    pub environment: Environment,
    pub credentials: Option<BinanceCredentials>,
}

impl BinanceParameters {
    /// Sandbox environment
    pub fn sandbox() -> Self {
        Self {
            environment: Environment::Sandbox,
            ..Default::default()
        }
    }

    /// Production environment
    pub fn production() -> Self {
        Self {
            environment: Environment::Production,
            ..Default::default()
        }
    }
}