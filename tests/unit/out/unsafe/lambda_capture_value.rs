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
    pub x: i32,
    pub y: i32,
}
impl S {
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
pub unsafe fn read_0(v: *const i32) -> i32 {
    return (*v);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut factor: i32 = 3;
    let mut scale: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let factor: i32 = factor;
        },
        |mut x: i32| -> i32 {
            return ((x) * (factor));
        }
    );
    assert!(((unsafe { scale.call(4,) }) == (12)));
    factor = 100;
    assert!(((unsafe { scale.call(4,) }) == (12)));
    let mut slot: i32 = 7;
    let mut p: *mut i32 = (&mut slot as *mut i32);
    let mut read_ptr: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let p: *mut i32 = p;
        },
        || -> i32 {
            return (*p);
        }
    );
    slot = 8;
    assert!(((unsafe { read_ptr.call() }) == (8)));
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).x).write(1);
        (&raw mut (*__d).y).write(2);
    };
    let mut sum: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let s: S = {
                let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
                S::copy_from(&raw mut __v, { &s });
                __v
            };
        },
        || -> i32 {
            return ((s.x) + (s.y));
        }
    );
    s.x = 50;
    assert!(((unsafe { sum.call() }) == (3)));
    let mut base: i32 = 10;
    let mut shifted: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let y: i32 = ((base) + (1));
        },
        |mut x: i32| -> i32 {
            return ((x) + (y));
        }
    );
    assert!(((unsafe { shifted.call(5,) }) == (16)));
    let k: i32 = 3;
    let mut by_copy: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let k: i32 = k;
        },
        |mut x: i32| -> i32 {
            return ((x) + (3));
        }
    );
    assert!(((unsafe { by_copy.call(1,) }) == (4)));
    let mut by_copy_used: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let k: i32 = k;
        },
        |mut x: i32| -> i32 {
            return ((x) + (unsafe { read_0(&k) }));
        }
    );
    assert!(((unsafe { by_copy_used.call(1,) }) == (4)));
    let mut implicit_used: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let k: i32 = k;
        },
        |mut x: i32| -> i32 {
            return ((x) + (unsafe { read_0(&k) }));
        }
    );
    assert!(((unsafe { implicit_used.call(1,) }) == (4)));
    let mut by_ref: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let k: *const i32 = &k;
        },
        |mut x: i32| -> i32 {
            return ((x) + (3));
        }
    );
    assert!(((unsafe { by_ref.call(1,) }) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
