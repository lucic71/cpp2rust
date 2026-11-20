extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Inner {
    pub x: i32,
    pub y: i32,
}
impl Inner {
    pub unsafe fn new_1(this: *mut Inner) {
        (&raw mut (*this).x).write(3);
        (&raw mut (*this).y).write(4);
    }
    pub unsafe fn copy_from(this: *mut Inner, _a0: *const Inner) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
    pub unsafe fn move_from(this: *mut Inner, _a0: *mut Inner) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
impl Default for Inner {
    fn default() -> Self {
        unsafe { Inner { x: 3, y: 4 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S {
    pub a: i32,
    pub b: libc::c_char,
    pub c: Inner,
    pub d: Inner,
}
impl S {
    pub unsafe fn new_1(this: *mut S) {
        (&raw mut (*this).a).write(1);
        (&raw mut (*this).b).write((2 as libc::c_char));
        {
            let __d = (&raw mut (*this).c);
            (&raw mut (*__d).x).write(3);
            (&raw mut (*__d).y).write(4);
        };
        Inner::new_1((&raw mut (*this).d));
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
        Inner::copy_from((&raw mut (*this).c), { &(*_a0).c });
        Inner::copy_from((&raw mut (*this).d), { &(*_a0).d });
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
        Inner::move_from((&raw mut (*this).c), { &mut (*_a0).c });
        Inner::move_from((&raw mut (*this).d), { &mut (*_a0).d });
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe {
            S {
                a: 1,
                b: (2 as libc::c_char),
                c: <Inner>::default(),
                d: <Inner>::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Boxed_int_ {
    pub v: i32,
    pub tag: i32,
}
impl Boxed_int_ {
    pub unsafe fn new(this: *mut Boxed_int_, mut x: i32, mut t: i32) {
        (&raw mut (*this).v).write(x);
        (&raw mut (*this).tag).write(t);
    }
    pub unsafe fn copy_from(this: *mut Boxed_int_, _a0: *const Boxed_int_) {
        (&raw mut (*this).v).write((*_a0).v);
        (&raw mut (*this).tag).write((*_a0).tag);
    }
    pub unsafe fn move_from(this: *mut Boxed_int_, _a0: *mut Boxed_int_) {
        (&raw mut (*this).v).write((*_a0).v);
        (&raw mut (*this).tag).write((*_a0).tag);
    }
}
impl Default for Boxed_int_ {
    fn default() -> Self {
        unsafe { Boxed_int_ { v: 0_i32, tag: 7 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::new_1(&raw mut s);
    assert!(((s.a) == (1)));
    assert!(((s.b as i32) == (2)));
    assert!(((s.c.x) == (3)));
    assert!(((s.c.y) == (4)));
    assert!(((s.d.x) == (3)));
    assert!(((s.d.y) == (4)));
    let mut boxed: Boxed_int_ = ::std::mem::MaybeUninit::uninit().assume_init();
    Boxed_int_::new(&raw mut boxed, { 5 }, { 9 });
    assert!(((boxed.v) == (5)));
    assert!(((boxed.tag) == (9)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
