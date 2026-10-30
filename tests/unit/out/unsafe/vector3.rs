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
    let mut v: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 1_();
    (*unimplemented!()) = 5_();
    (*unimplemented!()) = 6_();
    'loop_: for v2 in IterRange::new(unimplemented!(), unimplemented!()) {
        let v2: *mut () = &mut (*unimplemented!());
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let i: *mut () = &mut (*unimplemented!());
            (*i).prefix_inc();
        }
    }
    'loop_: for v2 in IterRange::new(unimplemented!(), unimplemented!()) {
        let v2: *mut () = &mut (*unimplemented!());
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut i: () = (*unimplemented!());
            unimplemented!();
        }
    }
    'loop_: for v2 in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v2: () = unimplemented!();
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut i: () = (*unimplemented!());
            unimplemented!();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
