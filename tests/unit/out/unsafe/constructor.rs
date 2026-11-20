extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn const_method(&self) -> i32 {
        return ((self.v) * (2));
    }
    pub unsafe fn mut_method(&mut self) {
        self.v += 1;
    }
    pub unsafe fn new(this: *mut S, mut init: i32) {
        (&raw mut (*this).v).write(init);
        (unsafe { S::mut_method(&mut *this) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) +=
            (unsafe { S::const_method(&mut *this) });
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { S::mut_method(self) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) +=
            (unsafe { S::const_method(self) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub unsafe fn new_2(this: *mut Point, mut x: i32, mut y: i32) {
        (&raw mut (*this).x).write(x);
        (&raw mut (*this).y).write(y);
    }
    pub unsafe fn new_1(this: *mut Point, mut v: i32) {
        Point::new_2(this.clone(), { v }, { ((v) + (1)) });
        (*this).y *= 10;
    }
    pub unsafe fn new_3(this: *mut Point) {
        Point::new_1(this.clone(), { 4 });
        (*this).x += 100;
    }
}
impl Default for Point {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Point = ::std::mem::MaybeUninit::uninit().assume_init();
                Point::new_3(&raw mut __v);
                __v
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
        S::new(&raw mut s, { 3 });
        let _dtor_s = std::mem::DropGuard::new(&raw mut s, |__p| unsafe { (*__p).destructor() });
        assert!(((s.v) == (4)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) == (8)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) == (18)));
    let mut p: Point = ::std::mem::MaybeUninit::uninit().assume_init();
    Point::new_3(&raw mut p);
    assert!(((p.x) == (104)));
    assert!(((p.y) == (50)));
    let mut q: Point = ::std::mem::MaybeUninit::uninit().assume_init();
    Point::new_1(&raw mut q, { 7 });
    assert!(((q.x) == (7)));
    assert!(((q.y) == (80)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const total_0);
}
