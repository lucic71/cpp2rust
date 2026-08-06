extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn pick_0(mut a: *const libc::c_char, mut b: *const libc::c_char, mut n: i32) -> i32 {
    return (((if ((((a) == (b)) as i32) != 0) { 10 } else { 20 }) + (n))
        + (((*a.offset(((0) as isize))) as i32) - ('a' as i32)));
}
pub unsafe fn total_1(mut x: *mut i32, mut y: *mut i32) -> i32 {
    (*x) += 1;
    return ((*x) + (*y));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: *const libc::c_char = (c"abc".as_ptr().cast_mut()).cast_const();
    let mut t: *const libc::c_char = (c"bcd".as_ptr().cast_mut()).cast_const();
    let mut n: i32 = 5;
    let mut v: i32 = 4;
    assert!(
        ((((unsafe {
            let _a: *const libc::c_char = s;
            let _b: *const libc::c_char = s;
            pick_0(_a, _b, n)
        }) == (15)) as i32)
            != 0)
    );
    assert!(((((unsafe { pick_0(s, t, n) }) == (25)) as i32) != 0));
    assert!(
        ((((unsafe {
            let _x: *mut i32 = (&raw mut v as *mut i32);
            let _y: *mut i32 = (&raw mut v as *mut i32);
            total_1(_x, _y)
        }) == (10)) as i32)
            != 0)
    );
    assert!(((((v) == (5)) as i32) != 0));
    return 0;
}
