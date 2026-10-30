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
    let mut p1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let arr: [(); 2] = [1_(), 2_()];
    let mut n: () = 2_();
    let mut const_ptr_pairs: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
