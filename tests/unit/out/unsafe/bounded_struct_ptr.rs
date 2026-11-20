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
pub struct Foo {
    pub x1: i32,
    pub x2: i32,
}
impl Foo {
    pub unsafe fn copy_from(this: *mut Foo, _a0: *const Foo) {
        (&raw mut (*this).x1).write((*_a0).x1);
        (&raw mut (*this).x2).write((*_a0).x2);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [Foo; 2] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut arr).cast::<Foo>();
        {
            let __d = __d.offset(0);
            (&raw mut (*__d).x1).write(1);
            (&raw mut (*__d).x2).write(2);
        };
        {
            let __d = __d.offset(1);
            (&raw mut (*__d).x1).write(3);
            (&raw mut (*__d).x2).write(4);
        };
    };
    let mut p1: *mut i32 = (&mut arr[(1) as usize].x1 as *mut i32);
    let mut a: i32 = (*p1);
    let mut p2: *mut Foo = (&mut arr[(0) as usize] as *mut Foo);
    assert!((((a) + ((*p2).x2)) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
