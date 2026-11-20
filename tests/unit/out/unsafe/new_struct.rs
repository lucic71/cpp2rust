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
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
impl Pair {
    pub unsafe fn new_1(this: *mut Pair) {}
    pub unsafe fn copy_from(this: *mut Pair, _a0: *const Pair) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
    pub unsafe fn move_from(this: *mut Pair, _a0: *mut Pair) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Triple {
    pub a: i32,
    pub b: i32,
    pub p: Pair,
}
impl Triple {
    pub unsafe fn new_1(this: *mut Triple) {
        Pair::new_1((&raw mut (*this).p));
    }
    pub unsafe fn copy_from(this: *mut Triple, _a0: *const Triple) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
        Pair::copy_from((&raw mut (*this).p), { &(*_a0).p });
    }
    pub unsafe fn move_from(this: *mut Triple, _a0: *mut Triple) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
        Pair::move_from((&raw mut (*this).p), { &mut (*_a0).p });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut Pair = {
        let __p = Box::into_raw(Box::<Pair>::new_uninit()) as *mut Pair;
        {
            let __d = __p;
            (&raw mut (*__d).x).write(1);
            (&raw mut (*__d).y).write(2);
        };
        __p
    };
    let mut out: i32 = (((*p).x) + ((*p).y));
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    assert!(((out) == (3)));
    let mut t: Triple = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t;
        (&raw mut (*__d).a).write(1);
        (&raw mut (*__d).b).write(0_i32);
        {
            let __d = (&raw mut (*__d).p);
            (&raw mut (*__d).x).write(0_i32);
            (&raw mut (*__d).y).write(0_i32);
        };
    };
    assert!(((t.a) == (1)));
    assert!(((t.b) == (0)));
    assert!(((t.p.x) == (0)) && ((t.p.y) == (0)));
    let mut q: *mut Triple = {
        let __p = Box::into_raw(Box::<Triple>::new_uninit()) as *mut Triple;
        {
            let __d = __p;
            (&raw mut (*__d).a).write(2);
            (&raw mut (*__d).b).write(3);
            {
                let __d = (&raw mut (*__d).p);
                (&raw mut (*__d).x).write(0_i32);
                (&raw mut (*__d).y).write(0_i32);
            };
        };
        __p
    };
    assert!((((*q).a) == (2)));
    assert!((((*q).b) == (3)));
    assert!((((*q).p.x) == (0)) && (((*q).p.y) == (0)));
    {
        let __p = q;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
