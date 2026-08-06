extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn step_0(mut mode: i32, mut v: i32) -> i32 {
    static mut base_1: [i32; 2] = unsafe { [100, 200] };
    static mut calls_2: i32 = unsafe { 0 };
    let mut r: i32 = 0_i32;
    goto_block!({
        '__entry: {
            r = 0;
            calls_2.postfix_inc();
            if !((((v) > (0)) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {}
        'from_positive: {
            r = ((base_1[((0) as usize)]) + (v));
            if !((((mode) == (1)) as i32) != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('from_negative);
        }
        '__f3_join: {
            goto!('__f0_join);
        }
        '__f1_else: {
            if !((((mode) == (2)) as i32) != 0) {
                goto!('__f5_join);
            }
        }
        '__f6_then: {
            goto!('from_positive);
        }
        '__f5_join: {}
        'from_negative: {
            r = ((base_1[((1) as usize)]) - (v));
        }
        '__f0_join: {
            return (((r) * (10)) + (calls_2));
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
    assert!(((((unsafe { step_0(0, 5) }) == (1051)) as i32) != 0));
    assert!(((((unsafe { step_0(1, 5) }) == (1952)) as i32) != 0));
    assert!(((((unsafe { step_0(0, -2_i32) }) == (2023)) as i32) != 0));
    assert!(((((unsafe { step_0(2, -2_i32) }) == (984)) as i32) != 0));
    return 0;
}
