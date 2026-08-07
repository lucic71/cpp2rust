extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn dispatch_0(mut op: i32, mut flags: i32) -> i32 {
    let mut r: i32 = 0_i32;
    let mut flags__1: i32 = 0_i32;
    goto_block!({
        '__entry: {
            r = 0;
            if !((((op) == (1)) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {}
        'from_op: {
            flags__1 = 7;
            r += flags__1;
            goto!('__f0_join);
        }
        '__f1_else: {
            if !((((op) == (2)) as i32) != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('from_op);
        }
        '__f3_join: {
            r += 100;
        }
        '__f0_join: {
            if !(((flags) & (4)) != 0) {
                goto!('__f5_join);
            }
        }
        '__f6_then: {
            r += 1000;
        }
        '__f5_join: {
            return r;
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
    assert!(((((unsafe { dispatch_0(1, 4) }) == (1007)) as i32) != 0));
    assert!(((((unsafe { dispatch_0(0, 4) }) == (1100)) as i32) != 0));
    assert!(((((unsafe { dispatch_0(2, 4) }) == (1007)) as i32) != 0));
    assert!(((((unsafe { dispatch_0(1, 0) }) == (7)) as i32) != 0));
    return 0;
}
