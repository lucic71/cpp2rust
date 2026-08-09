extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0(mut op: i32, mut v: i32) -> i32 {
    let mut r: i32 = 0;
    let mut a: i32 = 0_i32;
    let mut a__1: *const libc::c_char = std::ptr::null();
    switch!(match op {
        __v if __v == 1 => {
            a = ((v) * (4));
            if v != 0 {
                goto!('l1);
            }
            break;
        }
        __v if false => 'l1: {
            r = a;
            break;
        }
        __v if __v == 2 => {
            a__1 = (c"abcd".as_ptr().cast_mut()).cast_const();
            if v != 0 {
                goto!('l2);
            }
            break;
        }
        __v if false => 'l2: {
            r = (libc::strlen(a__1) as i32).clone();
            break;
        }
        _ => {}
    });
    return r;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { f_0(1, 3) }) == (12)) as i32) != 0));
    assert!(((((unsafe { f_0(2, 1) }) == (4)) as i32) != 0));
    assert!(((((unsafe { f_0(1, 0) }) == (0)) as i32) != 0));
    return 0;
}
