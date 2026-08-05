extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn route_0(mut op: i32, mut v: i32) -> i32 {
    let mut out: i32 = 0;
    let mut base: i32 = 0_i32;
    switch!(match op {
        __v if __v == 1 => {
            let mut base: i32 = ((v) * (10));
            if ((((v) > (3)) as i32) != 0) {
                out = ((base) + (1));
                goto!('tail);
            }
            v = base;
        }
        __v if __v == 2 => {
            base = ((v) + (7));
            out = ((base) * (2));
        }
        __v if false => 'tail: {
            out += 3;
            break;
        }
        __v if __v == 3 => {
            out = -v;
            break;
        }
        _ => {
            out = 99;
            break;
        }
    });
    return out;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { route_0(1, 5) }) == (54)) as i32) != 0));
    assert!(((((unsafe { route_0(1, 2) }) == (57)) as i32) != 0));
    assert!(((((unsafe { route_0(2, 10) }) == (37)) as i32) != 0));
    assert!(((((unsafe { route_0(3, 4) }) == (-4_i32)) as i32) != 0));
    assert!(((((unsafe { route_0(9, 0) }) == (99)) as i32) != 0));
    return 0;
}
