extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut cond: () = true;
    let mut os1: *mut () = if cond {
        (&raw mut (*unimplemented!()) as *mut ())
    } else {
        (&raw mut (*unimplemented!()) as *mut ())
    };
    unimplemented!();
    let os2: *mut () = if cond {
        (&raw mut (*unimplemented!()) as *mut ())
    } else {
        (&raw mut (*unimplemented!()) as *mut ())
    };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
