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
impl Point {
    pub unsafe fn sum(&self) -> () {
        return ((self.x) + (self.y));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub values: (),
    pub points: (),
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            values: unimplemented!(),
            points: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut Holder));
    }
}
pub unsafe fn push_and_index_0(v: *mut ()) -> () {
    unimplemented!();
    return ((unimplemented!()) - (1_()));
}
pub unsafe fn sum_ref_1(v: *const ()) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        s += (*unimplemented!());
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: () = unimplemented!();
    (*unimplemented!()) = 10_();
    unimplemented!();
    (*unimplemented!()) += 5_();
    (*unimplemented!()).postfix_inc();
    unimplemented!();
    (*unimplemented!()) = (*unimplemented!());
    unimplemented!();
    (*unimplemented!()) = 0_();
    unimplemented!();
    let mut i: () = 0_();
    (*unimplemented!()) = 3_();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 5_();
    unimplemented!();
    let mut p: *mut () = (&mut (*unimplemented!()) as *mut ());
    (*p) = 9_();
    unimplemented!();
    unimplemented!();
    let mut h: Holder = <Holder>::default();
    unimplemented!();
    (*unimplemented!()) = 6_();
    unimplemented!();
    (*unimplemented!()).y = 5_();
    unimplemented!();
    unimplemented!();
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    (*unimplemented!()) = ((*unimplemented!()) + (1_()));
    (*unimplemented!()).x = (*unimplemented!());
    unimplemented!();
    let mut q: Point = (*unimplemented!());
    q.x = 0_();
    unimplemented!();
    let mut grid: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 8_();
    unimplemented!();
    (*unimplemented!()) = (*unimplemented!());
    (*unimplemented!()) += (*unimplemented!());
    unimplemented!();
    let mut a: () = unimplemented!();
    (*unimplemented!()) = ((*unimplemented!()) + (*unimplemented!()));
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
