#![crate_id = "discotech"]
#![crate_type = "lib"]
#![feature(convert)]
#![feature(duration)]

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rustc_serialize;
extern crate zookeeper;

pub use config::{read_config, DiscoConfig};
pub use serverset::Serverset;

pub mod config;
pub mod serverset;
