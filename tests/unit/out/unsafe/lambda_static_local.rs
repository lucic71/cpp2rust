extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut a_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut next: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return (*(*(*&raw const n_1).get()).prefix_inc());
        }
    });
    assert!(((unsafe { next.call() }) == (1)));
    assert!(((unsafe { next.call() }) == (2)));
    let mut copy: FnPtr<fn() -> i32> = next.copy_from();
    assert!(((unsafe { copy.call() }) == (3)));
    assert!(((unsafe { next.call() }) == (4)));
    let mut fresh: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return (*(*(*&raw const n_1).get()).prefix_inc());
        }
    });
    assert!(((unsafe { fresh.call() }) == (5)));
    assert!(((unsafe { next.call() }) == (6)));
    let mut per_type: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut x: i32| -> i32 {
            (*(*&raw const calls_2).get()).postfix_inc();
            return (*(*&raw const calls_2).get());
        },
        |mut x: f64| -> i32 {
            (*(*&raw const calls_3).get()).postfix_inc();
            return (*(*&raw const calls_3).get());
        }
    );
    assert!(((unsafe { per_type.spec::<fn(i32) -> i32>(0).call(1,) }) == (1)));
    assert!(((unsafe { per_type.spec::<fn(i32) -> i32>(0).call(2,) }) == (2)));
    assert!(((unsafe { per_type.spec::<fn(f64) -> i32>(1).call(1_f64,) }) == (1)));
    assert!(((unsafe { per_type.spec::<fn(i32) -> i32>(0).call(3,) }) == (3)));
    let mut read: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut _a0: i32| -> i32 {
        unsafe {
            return (*(*&raw const a_0).get());
        }
    });
    assert!(((unsafe { read.call(0,) }) == (0)));
    (*(*&raw const a_0).get()) = 1;
    assert!(((unsafe { read.call(0,) }) == (1)));
    let mut local: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            let mut p: P_4 = ::std::mem::MaybeUninit::uninit().assume_init();
            {
                let __d = &raw mut p;
                (&raw mut (*__d).x).write(2);
                (&raw mut (*__d).y).write(3);
            };
            return ((p.x) * (p.y));
        }
    });
    assert!(((unsafe { local.call() }) == (6)));
    return 0;
}
static mut n_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
static mut calls_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
static mut calls_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct P_4 {
    pub x: i32,
    pub y: i32,
}
impl P_4 {
    pub unsafe fn new_1(this: *mut P_4) {}
    pub unsafe fn copy_from(this: *mut P_4, _a0: *const P_4) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
    pub unsafe fn move_from(this: *mut P_4, _a0: *mut P_4) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const a_0).get();
}
