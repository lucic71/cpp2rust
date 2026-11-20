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
pub struct POD {
    pub x1: i32,
    pub x2: i32,
    pub x3: i32,
}
impl POD {
    pub unsafe fn copy_from(this: *mut POD, _a0: *const POD) {
        (&raw mut (*this).x1).write((*_a0).x1);
        (&raw mut (*this).x2).write((*_a0).x2);
        (&raw mut (*this).x3).write((*_a0).x3);
    }
}
pub unsafe fn PODIncrement_0(pod: *mut POD) {
    (*pod).x1 += 1;
    (*pod).x2 += 2;
    (*pod).x3 += 3;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p1: POD = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut p1;
        (&raw mut (*__d).x1).write(10);
        (&raw mut (*__d).x2).write(11);
        (&raw mut (*__d).x3).write(12);
    };
    let mut p2: POD = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut p2;
        (&raw mut (*__d).x1).write(p1.x1);
        (&raw mut (*__d).x2).write(p1.x2);
        (&raw mut (*__d).x3).write(p1.x3);
    };
    (unsafe { PODIncrement_0(&mut p2) });
    assert!(((((p2.x1) + (p2.x2)) + (p2.x3)) == (39)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
