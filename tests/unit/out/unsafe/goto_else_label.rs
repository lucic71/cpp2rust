extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut fails_0: i32 = unsafe { 0 };
pub unsafe fn fail_mark_1() -> i32 {
    fails_0.postfix_inc();
    return -1_i32;
}
pub unsafe fn helper_2(mut mode: i32, mut v: i32) -> i32 {
    let mut r: i32 = 0_i32;
    goto_block!({
        '__entry: {
            r = 0;
            if !((((mode) == (1)) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {
            if !((((v) < (0)) as i32) != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('bad_input);
        }
        '__f3_join: {
            r = ((v) * (2));
            goto!('__f0_join);
        }
        '__f1_else: {
            if !((((mode) == (2)) as i32) != 0) {
                goto!('__f6_else);
            }
        }
        '__f7_then: {
            if !((((v) == (0)) as i32) != 0) {
                goto!('__f8_join);
            }
        }
        '__f9_then: {
            goto!('bad_input);
        }
        '__f8_join: {
            r = ((100) / (v));
            goto!('__f5_join);
        }
        '__f6_else: {}
        'bad_input: {
            r = (unsafe { fail_mark_1() });
        }
        '__f5_join: {}
        '__f0_join: {
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
    assert!(((((unsafe { helper_2(1, 4) }) == (8)) as i32) != 0));
    assert!(((((unsafe { helper_2(1, -1_i32) }) == (-1_i32)) as i32) != 0));
    assert!(((((unsafe { helper_2(2, 5) }) == (20)) as i32) != 0));
    assert!(((((unsafe { helper_2(2, 0) }) == (-1_i32)) as i32) != 0));
    assert!(((((unsafe { helper_2(7, 3) }) == (-1_i32)) as i32) != 0));
    assert!(((((fails_0) == (3)) as i32) != 0));
    return 0;
}
