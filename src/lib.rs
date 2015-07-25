#![crate_id = "discotech"]
#![crate_type = "lib"]
#![feature(libc)]
extern crate libc;

use libc::types::os::common::bsd44::{socklen_t, sockaddr};
use libc::{c_char, c_int, c_void};
use std::mem;

#[link(name="dl")]
extern {
    fn dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void;
}
const RTLD_NEXT: *const c_void = -1isize as *const c_void;
pub unsafe fn dlsym_next(symbol: &'static str) -> *const u8 {
    let ptr = dlsym(RTLD_NEXT, symbol.as_ptr() as *const c_char);
    if ptr.is_null() {
        panic!("discotech: Unable to find underlying function for {}", symbol);
    }
    ptr as *const u8
}

static mut real_connect : Option<fn(c_int, *const sockaddr, socklen_t) -> c_int> = None;
#[no_mangle]
pub unsafe extern "C" fn connect(socket: c_int, address: *const sockaddr,
                           len: socklen_t) -> c_int {
    println!("HOOKING CONNECT!!!!!!!!!!!!!!!!!");
    let f = match real_connect {
        Some(f) => f,
        None => {
            let ptr = dlsym_next("connect");
            let f: &mut fn(c_int, *const sockaddr, socklen_t) -> c_int = mem::transmute(ptr);
            real_connect = Some(*f);
            *f
        }
    };
    f(socket, address, len)
}

#[no_mangle]
pub extern "C" fn _init() {
    println!("in _init!!!!");
}

#[test]
fn lookup() {
    println!("lookup: {:?}", unsafe { dlsym_next("puts") });
    unsafe { connect(0, 0 as *const sockaddr, 4 as socklen_t); }
}
