#![crate_id = "discotech"]
#![crate_type = "lib"]

#![feature(btree_range)]
#![feature(collections_bound)]

pub use store::Store;

pub mod store;

extern crate zookeeper;
