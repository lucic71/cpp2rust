extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct S {
    pub tag: i32,
    pub v: Vec<i32>,
}
impl S {
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).tag).write((*_a0).tag);
        (&raw mut (*this).v).write((*_a0).v.clone());
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).tag).write((*_a0).tag);
        (&raw mut (*this).v).write(std::mem::take(&mut (*_a0).v));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        self.tag = (*_a0).tag;
        {
            self.v = std::mem::take(&mut (*_a0).v);
        };
        return &mut (*(self as *mut S));
    }
    pub unsafe fn new_1(this: *mut S) {
        (&raw mut (*this).v).write(Vec::new());
    }
}
pub unsafe fn add_0(mut v: *mut Vec<i32>, mut k: i32) {
    {
        let a0_clone = k.clone();
        (*v).push(a0_clone)
    };
}
pub unsafe fn run_1(mut h: *mut S) {
    (unsafe {
        let _v: *mut Vec<i32> = (&mut (*h).v as *mut Vec<i32>);
        let _k: i32 = (*h).tag;
        add_0(_v, _k)
    });
    let mut pv: *mut Vec<i32> = (&mut (*h).v as *mut Vec<i32>);
    {
        let __a1 = (((*(pv).cast_const()).len()) as i32);
        (*pv).push(__a1)
    };
    assert!(
        ((((*h).v.len()) == (2_usize)) && (((&mut (*h)).v[(0_usize)]) == (7)))
            && (((&mut (*h)).v[(1_usize)]) == (1))
    );
    assert!((((*h).tag) == (7)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut local: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::new_1(&raw mut local);
    local.tag = 7;
    (unsafe { run_1((&mut local as *mut S)) });
    let mut heap: *mut S = {
        let __p = Box::into_raw(Box::<S>::new_uninit()) as *mut S;
        S::new_1(__p);
        __p
    };
    (*heap).tag = 7;
    (unsafe { run_1(heap) });
    {
        let __p = heap;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
