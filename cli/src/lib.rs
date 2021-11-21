

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

mod auth;
pub use auth::*;

mod actions;
pub use actions::*;
