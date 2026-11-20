extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn triple_it_1(mut x: i32) -> i32 {
    return ((x) * (3));
}
pub static mut g_op_2: libcc2rs::LazyStorage<Option<unsafe fn(i32) -> i32>> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(None) });
pub unsafe fn set_op_3(mut fn_: Option<unsafe fn(i32) -> i32>) {
    (*(*&raw const g_op_2).get()) = fn_;
}
pub unsafe fn call_op_4(mut x: i32) -> i32 {
    if !(*(*&raw const g_op_2).get()).is_none() {
        return (unsafe { (*(*&raw const g_op_2).get()).unwrap()(x) });
    }
    return x;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { call_op_4(5,) }) == (5)));
    (unsafe { set_op_3(Some(double_it_0)) });
    assert!(!((*(*&raw const g_op_2).get()).is_none()));
    assert!(((*(*&raw const g_op_2).get()) == (Some(double_it_0))));
    assert!(((unsafe { call_op_4(5,) }) == (10)));
    (unsafe { set_op_3(Some(triple_it_1)) });
    assert!(((*(*&raw const g_op_2).get()) == (Some(triple_it_1))));
    assert!(((unsafe { call_op_4(5,) }) == (15)));
    (unsafe { set_op_3(None) });
    assert!((*(*&raw const g_op_2).get()).is_none());
    assert!(((unsafe { call_op_4(5,) }) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const g_op_2).get();
}
