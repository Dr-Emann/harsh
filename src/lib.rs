#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

mod error;
mod harsh;

pub use error::{Error, Result};
pub use harsh::{Harsh, HarshBuilder};
