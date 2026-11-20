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
    pub v: Vec<i32>,
    pub a: i32,
}
impl S {
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v.clone());
        (&raw mut (*this).a).write((*_a0).a);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).v).write(std::mem::take(&mut (*_a0).v));
        (&raw mut (*this).a).write((*_a0).a);
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        {
            self.v = std::mem::take(&mut (*_a0).v);
        };
        self.a = (*_a0).a;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn new_1(this: *mut S) {
        (&raw mut (*this).v).write(Vec::new());
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
    {
        let __a1 = 1;
        s.v.push(__a1)
    };
    'loop_: for e in IterRange::new(s.v.begin(), s.v.end()) {
        let mut e: i32 = (*e.iter_deref());
        s.a.postfix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
