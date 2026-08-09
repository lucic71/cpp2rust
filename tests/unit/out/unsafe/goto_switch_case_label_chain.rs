extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn pick_0(mut op: i32, mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match op {
        __v if __v == 1 => {
            if (((x) == (0)) as i32) != 0 {
                r = 5;
                break;
            }
            goto!('shared);
        }
        __v if __v == 2 => 'shared: {
            let mut t: i32 = ((x) * (3));
            r = ((t) + (1));
            break;
        }
        __v if false => '__default_1: {
            r = -1_i32;
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
    assert!(((((unsafe { pick_0(1, 0) }) == (5)) as i32) != 0));
    assert!(((((unsafe { pick_0(1, 4) }) == (13)) as i32) != 0));
    assert!(((((unsafe { pick_0(2, 2) }) == (7)) as i32) != 0));
    assert!(((((unsafe { pick_0(0, 9) }) == (-1_i32)) as i32) != 0));
    return 0;
}
