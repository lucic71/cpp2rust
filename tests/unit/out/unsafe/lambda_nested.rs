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
    pub v: i32,
}
impl S {
    pub unsafe fn nested_this(&mut self) -> i32 {
        let mut outer: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
            {
                let this_: *mut S = (self as *mut S);
            },
            |mut y: i32| -> i32 {
                let mut inner: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
                    {
                        let this_: *mut S = this_;
                        let y: i32 = y;
                    },
                    |mut z: i32| -> i32 {
                        return ((((*this_).v) + (y)) + (z));
                    }
                );
                return (unsafe { inner.call(1) });
            }
        );
        return (unsafe { outer.call(20) });
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 10;
    let mut outer: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let x: *mut i32 = &mut x;
        },
        |mut y: i32| -> i32 {
            let mut inner: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
                {
                    let x: *mut i32 = x;
                    let y: i32 = y;
                },
                |mut z: i32| -> i32 {
                    return (((*x) + (y)) + (z));
                }
            );
            return (unsafe { inner.call(1) });
        }
    );
    assert!(((unsafe { outer.call(20,) }) == (31)));
    x = 100;
    assert!(((unsafe { outer.call(20,) }) == (121)));
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).v).write(5);
    };
    assert!(((unsafe { S::nested_this(&mut s,) }) == (26)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
