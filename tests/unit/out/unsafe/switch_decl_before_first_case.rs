extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn reduce_0(mut rule: i32, mut v: i32) -> i32 {
    let mut acc: i32 = 0;
    let mut tmp: i32 = 0_i32;
    let mut wide: i64 = 0_i64;
    'switch: {
        let __match_cond = rule;
        match __match_cond {
            __v if __v == 0 => {
                tmp = ((v) * (2));
                acc = ((tmp) + (1));
                break 'switch;
            }
            __v if __v == 1 => {
                wide = ((v as i64) + (10_i64));
                acc = (((wide) * (2_i64)) as i32);
                break 'switch;
            }
            __v if __v == 2 => {
                tmp = ((v) - (1));
                wide = (tmp as i64);
                acc = ((wide as i32) * (3));
                break 'switch;
            }
            _ => {
                acc = -1_i32;
                break 'switch;
            }
        }
    };
    return acc;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { reduce_0(0, 5) }) == (11)) as i32) != 0));
    assert!(((((unsafe { reduce_0(1, 5) }) == (30)) as i32) != 0));
    assert!(((((unsafe { reduce_0(2, 5) }) == (12)) as i32) != 0));
    assert!(((((unsafe { reduce_0(9, 5) }) == (-1_i32)) as i32) != 0));
    return 0;
}
