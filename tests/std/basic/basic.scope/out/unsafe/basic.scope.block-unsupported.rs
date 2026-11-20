extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut handler_runs_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn test_1() {
    let mut code: i32 = 1;
    {
        2;
    };
    assert!(((*(*&raw const handler_runs_0).get()) == (1)));
    assert!(((code) == (1)));
}
pub static mut handler_runs_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn test_3() {
    let mut stage: i32 = 10;
    {
        let mut stage: i32 = 20;
        stage;
    };
    assert!(((*(*&raw const handler_runs_2).get()) == (1)));
    assert!(((stage) == (10)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const handler_runs_0).get();
    (*&raw const handler_runs_2).get();
}
