#![crate_id = "discotech"]
#![crate_type = "lib"]
#![feature(libc)]
#![feature(btree_range)]
#![feature(collections_bound)]
#![feature(convert)]
#![feature(duration)]

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rustc_serialize;
extern crate zookeeper;
#[macro_use]
extern crate redhook;
extern crate libc;


pub use config::*;
pub use serverset::*;

pub mod config;
pub mod serverset;

use libc::types::os::common::bsd44::{addrinfo, socklen_t, sockaddr};
use libc::{c_char, c_int, c_void};

hook! {
    fn connect(socket: c_int, address: *const sockaddr,
               len: socklen_t) -> c_int => my_connect {
        println!("HOOKING connect!!!!!!!!!!!!!!!!!");
        unsafe { real!(connect)(socket, address, len) }
    }
}

hook! {
    fn getaddrinfo(node: *const c_char, service: *const c_char,
                   hints: *const addrinfo,
                   res: *const *const addrinfo) -> c_int => my_getaddrinfo {
        println!("HOOKING getaddrinfo!!!!!!!!!!!!!!!!!");
        unsafe { real!(getaddrinfo)(node, service, hints, res) }
    }
}
