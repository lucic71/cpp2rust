extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn acc_0(mut x: i32) -> i32 {
    static mut total_1: i32 = unsafe { 5 };
    static mut limit_2: i32 = unsafe { 10 };
    goto_block!({
        '__entry: {
            if ((((x) < (0)) as i32) != 0) {
                goto!('done);
            }
            total_1 += x;
            if ((((total_1) > (limit_2)) as i32) != 0) {
                total_1 = limit_2;
            }
        }
        'done: {
            return total_1;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { acc_0(3) }) == (8)) as i32) != 0));
    assert!(((((unsafe { acc_0(-1_i32) }) == (8)) as i32) != 0));
    assert!(((((unsafe { acc_0(4) }) == (10)) as i32) != 0));
    return 0;
}
