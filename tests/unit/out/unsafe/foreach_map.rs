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
    let mut m: () = unimplemented!();
    let mut i: () = 0_();
    let mut k: () = 100_();
    'loop_: while ((i) < (100_())) {
        (*unimplemented!()) = ((k) / (2_f64));
        {
            i.prefix_inc();
            k.prefix_dec()
        };
    }
    let mut sum: () = 0_();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let i: *const () = &(*unimplemented!());
        sum += (*i).second;
    }
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: () = unimplemented!();
        sum += i.first;
    }
    unimplemented!();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: () = unimplemented!();
        i.second = -1_();
        unimplemented!();
    }
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let i: *const () = &(*unimplemented!());
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
