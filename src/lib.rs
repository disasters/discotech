#![crate_id = "discotech"]
#![crate_type = "lib"]

#![feature(btree_range)]
#![feature(collections_bound)]
#![feature(convert)]
#![feature(duration)]

#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate zookeeper;

pub use config::*;
pub use serverset::*;

pub mod config;
pub mod serverset;
