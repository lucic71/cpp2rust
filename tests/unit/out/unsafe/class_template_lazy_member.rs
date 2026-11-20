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
    pub x: i32,
}
impl Point {
    pub unsafe fn copy_from(this: *mut Point, _a0: *const Point) {
        (&raw mut (*this).x).write((*_a0).x);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Box_int_ {
    pub val: i32,
}
impl Box_int_ {
    pub unsafe fn twice(&mut self) -> i32 {
        return ((self.val) + (self.val));
    }
    pub unsafe fn copy_from(this: *mut Box_int_, _a0: *const Box_int_) {
        (&raw mut (*this).val).write((*_a0).val);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Box_Point_ {
    pub val: Point,
}
impl Box_Point_ {
    pub unsafe fn get(&mut self, __ret: *mut Point) {
        Point::copy_from(__ret.clone(), { &self.val });
        return;
    }
    pub unsafe fn copy_from(this: *mut Box_Point_, _a0: *const Box_Point_) {
        Point::copy_from((&raw mut (*this).val), { &(*_a0).val });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Deduced_int_ {
    pub val: i32,
}
impl Deduced_int_ {
    pub unsafe fn twice(&mut self) -> i32 {
        return ((self.val) + (self.val));
    }
    pub unsafe fn copy_from(this: *mut Deduced_int_, _a0: *const Deduced_int_) {
        (&raw mut (*this).val).write((*_a0).val);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Deduced_Point_ {
    pub val: Point,
}
impl Deduced_Point_ {
    pub unsafe fn get(&mut self, __ret: *mut Point) {
        Point::copy_from(__ret.clone(), { &self.val });
        return;
    }
    pub unsafe fn copy_from(this: *mut Deduced_Point_, _a0: *const Deduced_Point_) {
        Point::copy_from((&raw mut (*this).val), { &(*_a0).val });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i: Box_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut i;
        (&raw mut (*__d).val).write(3);
    };
    assert!(((unsafe { Box_int_::twice(&mut i,) }) == (6)));
    let mut p: Box_Point_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut p;
        {
            let __d = (&raw mut (*__d).val);
            (&raw mut (*__d).x).write(4);
        };
    };
    assert!(
        (({
            let mut __v: Point = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { Box_Point_::get(&mut p, &raw mut __v) });
            __v
        }
        .x) == (4))
    );
    let mut d: Deduced_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut d;
        (&raw mut (*__d).val).write(5);
    };
    assert!(((unsafe { Deduced_int_::twice(&mut d,) }) == (10)));
    let mut dp: Deduced_Point_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut dp;
        {
            let __d = (&raw mut (*__d).val);
            (&raw mut (*__d).x).write(6);
        };
    };
    assert!(
        (({
            let mut __v: Point = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { Deduced_Point_::get(&mut dp, &raw mut __v) });
            __v
        }
        .x) == (6))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
