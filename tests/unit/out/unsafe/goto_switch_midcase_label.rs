extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn classify_0(mut kind: i32, mut x: i32) -> i32 {
    let mut len: i32 = 0;
    let mut width: i32 = 0;
    switch!(match kind {
        __v if __v == 0 => {
            width = ((x) * (2));
            goto!('finish_width);
        }
        __v if __v == 1 => {
            len = ((x) + (1));
            if ((((len) > (10)) as i32) != 0) {
                len = 10;
            }
        }
        __v if false => 'finish_width: {
            width += len;
            width.postfix_inc();
            break;
        }
        __v if __v == 2 => {
            len = 50;
            goto!('finish_width);
        }
        __v if false => '__default_1: {
            width = -1_i32;
            break;
        }
        _ => {
            goto!('__default_1);
        }
    });
    return width;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { classify_0(0, 4) }) == (9)) as i32) != 0));
    assert!(((((unsafe { classify_0(1, 2) }) == (4)) as i32) != 0));
    assert!(((((unsafe { classify_0(1, 42) }) == (11)) as i32) != 0));
    assert!(((((unsafe { classify_0(2, 0) }) == (51)) as i32) != 0));
    assert!(((((unsafe { classify_0(7, 0) }) == (-1_i32)) as i32) != 0));
    return 0;
}
