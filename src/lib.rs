#[macro_use]
extern crate error_chain;
#[cfg(feature = "cc")]
extern crate libc;
extern crate log;
#[macro_use]
extern crate tera;
#[cfg(feature = "cc")]
pub mod interface;
pub mod generator;
mod errors;
#[cfg(feature = "cc")]
pub use interface::*;
pub use generator::*;
pub use tera::Context;
