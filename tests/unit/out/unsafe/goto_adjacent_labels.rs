extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn run_0(mut x: i32) -> i32 {
    let mut steps: i32 = 0_i32;
    goto_block!({
        '__entry: {
            steps = 0;
            if ((((x) < (0)) as i32) != 0) {
                goto!('error);
            }
            steps = 1;
            if ((((x) == (0)) as i32) != 0) {
                goto!('done);
            }
            steps = 2;
        }
        'error: {}
        'done: {
            steps += 10;
            return steps;
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
    assert!(((((unsafe { run_0(-1_i32) }) == (10)) as i32) != 0));
    assert!(((((unsafe { run_0(0) }) == (11)) as i32) != 0));
    assert!(((((unsafe { run_0(5) }) == (12)) as i32) != 0));
    return 0;
}
