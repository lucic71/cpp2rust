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
    let N: () = 10000_();
    let sentinel: () = ((N) / (2_()));
    let mut m: () = unimplemented!();
    (*unimplemented!()) = sentinel;
    let mut it: () = unimplemented!();
    let mut p: *mut () = (&mut (*it).second as *mut ());
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (sentinel)) {
        (*unimplemented!()) = i;
        i.prefix_inc();
    }
    let mut i: () = ((sentinel) + (1_()));
    'loop_: while ((i) <= (N)) {
        (*unimplemented!()) = i;
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*it).second = 57005_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut prev: () = -1_();
    'loop_: for pair in IterRange::new(unimplemented!(), unimplemented!()) {
        let pair: *mut () = &mut (*unimplemented!());
        unimplemented!();
        prev = (*pair).first;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
