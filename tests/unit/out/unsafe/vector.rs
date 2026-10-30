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
pub struct S {
    pub x: (),
}
pub unsafe fn copy_0(mut copy_vector: ()) {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 40_();
    (*unimplemented!()) = 50_();
    unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { copy_0(unimplemented!()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = 7_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut src: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (4_())) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s2: () = unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v4: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v5: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v6: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (s2)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v7: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (200_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut p1: *const () = (unimplemented!()).cast_const();
    unimplemented!();
    let mut p2: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*p2) = 99_f64;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p2.prefix_inc();
    (*p2) = 98_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let ref0: *mut () = unimplemented!();
    (*ref0) = 5_f64;
    unimplemented!();
    let mut x0: () = (*unimplemented!());
    unimplemented!();
    x0 = 6_f64;
    unimplemented!();
    let mut idx: () = 0_();
    unimplemented!();
    unimplemented!();
    let ref1: *mut () = unimplemented!();
    (*ref1) += 1.5_f64;
    unimplemented!();
    let mut x1: () = (*unimplemented!());
    unimplemented!();
    x1 -= 1.5_f64;
    unimplemented!();
    unimplemented!();
    let mut a: S = S { x: 1_() };
    let mut pa: *mut S = (&mut a as *mut S);
    let mut v8: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ppa: *mut *mut S = (&mut pa as *mut *mut S);
    let mut v9: () = unimplemented!();
    unimplemented!();
    (*(*(*(*unimplemented!())))).x = 2_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
