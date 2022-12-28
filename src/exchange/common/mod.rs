pub mod traits;
pub mod model;
pub mod shared;
pub mod message;

pub type Result<T> = std::result::Result<T, crate::errors::LessError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Environment {
    Production,
    Sandbox
}

impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}


// pub mod errors;
// pub mod prelude;
//
// pub use errors::*;