#[macro_use]
extern crate error_chain;
#[cfg(feature = "cc")]
extern crate libc;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate log;
extern crate env_logger;

#[cfg(feature = "cc")]
pub mod interface;
pub mod generator;
pub mod errors;
#[cfg(feature = "cc")]
pub use interface::*;
pub use generator::*;
pub use tera::Context;
