extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fallthrough_default_0(mut x: i32, mut flag: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 7 => {
            if (flag != 0) {
                r = 100;
                break;
            };
        }
        __v if false => '__default_1: {
            r = 42;
            break;
        }
        _ => {
            goto!('__default_1);
        }
    });
    return r;
}
pub unsafe fn breakless_default_1(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 7 => {
            r += 1;
        }
        __v if false => '__default_3: {
            r += 42;
        }
        _ => {
            goto!('__default_3);
        }
    });
    return ((r) + (1));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { fallthrough_default_0(7, 0) }) == (42)));
    assert!(((unsafe { fallthrough_default_0(7, 1) }) == (100)));
    assert!(((unsafe { fallthrough_default_0(99, 0) }) == (42)));
    assert!(((unsafe { breakless_default_1(7) }) == (44)));
    assert!(((unsafe { breakless_default_1(99) }) == (43)));
    return 0;
}
