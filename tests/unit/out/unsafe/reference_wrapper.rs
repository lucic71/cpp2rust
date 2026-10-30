extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Point {
    pub x: (),
    pub y: (),
}
pub unsafe fn set_0(mut ref_: (), mut val: ()) {
    (*unimplemented!()) = val;
}
pub unsafe fn read_1(mut ref_: ()) -> () {
    let r: *mut () = unimplemented!();
    return (*r);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut i1: () = 10_();
    let mut ref_1: () = unimplemented!();
    (*unimplemented!()) = 20_();
    let i2: *mut () = unimplemented!();
    (*i2) += 5_();
    unimplemented!();
    let mut i3: () = 1_();
    let mut i4: () = 2_();
    let mut ref_3: () = unimplemented!();
    let mut ref_4: () = unimplemented!();
    (*unimplemented!()) = (*unimplemented!());
    unimplemented!();
    (unsafe { set_0(unimplemented!(), 99_()) });
    unimplemented!();
    let mut point: Point = Point { x: 3_(), y: 4_() };
    let mut point_ref: () = unimplemented!();
    (*unimplemented!()).x = 30_();
    (*unimplemented!()).y = 40_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
