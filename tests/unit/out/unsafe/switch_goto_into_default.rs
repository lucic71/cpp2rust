extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn direct_label_0(mut x: i32, mut y: i32) -> i32 {
    switch!(match x {
        __v if __v == 1 => {
            if y != 0 {
                goto!('other);
            }
            return 10;
        }
        __v if __v == 2 => {
            return 30;
        }
        __v if false => 'other: {
            return 20;
        }
        _ => {
            goto!('other);
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn braced_label_1(mut x: i32, mut y: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        __v if __v == 1 => {
            if y != 0 {
                goto!('other);
            }
            r = 10;
            break;
        }
        __v if __v == 2 => {
            r = 30;
            break;
        }
        __v if false => '__default_1: {}
        __v if false => 'other: {
            r = 20;
            break;
        }
        _ => {
            goto!('__default_1);
        }
    });
    return r;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { direct_label_0(1, 0) }) == (10)) as i32) != 0));
    assert!(((((unsafe { direct_label_0(1, 1) }) == (20)) as i32) != 0));
    assert!(((((unsafe { direct_label_0(2, 0) }) == (30)) as i32) != 0));
    assert!(((((unsafe { direct_label_0(5, 0) }) == (20)) as i32) != 0));
    assert!(((((unsafe { direct_label_0(0, 1) }) == (20)) as i32) != 0));
    assert!(((((unsafe { braced_label_1(1, 0) }) == (10)) as i32) != 0));
    assert!(((((unsafe { braced_label_1(1, 1) }) == (20)) as i32) != 0));
    assert!(((((unsafe { braced_label_1(2, 0) }) == (30)) as i32) != 0));
    assert!(((((unsafe { braced_label_1(5, 0) }) == (20)) as i32) != 0));
    assert!(((((unsafe { braced_label_1(0, 1) }) == (20)) as i32) != 0));
    return 0;
}
