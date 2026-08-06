extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn compute_0(mut op: i32, mut a: i32, mut b: i32) -> i32 {
    let mut r: i32 = 0_i32;
    goto_block!({
        '__entry: {
            r = 0;
            if !((((a) > (0)) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {}
        'int_path: {
            r = ((a) + (b));
            if !(op != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('fp_path);
        }
        '__f3_join: {
            goto!('__f0_join);
        }
        '__f1_else: {
            if !((((b) > (0)) as i32) != 0) {
                goto!('__f5_join);
            }
        }
        '__f6_then: {
            goto!('int_path);
        }
        '__f5_join: {}
        'fp_path: {
            r = ((a) * (b));
        }
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
    assert!(((((unsafe { compute_0(0, 5, 3) }) == (8)) as i32) != 0));
    assert!(((((unsafe { compute_0(1, 5, 3) }) == (15)) as i32) != 0));
    assert!(((((unsafe { compute_0(0, -2_i32, 4) }) == (2)) as i32) != 0));
    assert!(((((unsafe { compute_0(0, -2_i32, -4_i32) }) == (8)) as i32) != 0));
    assert!(((((unsafe { compute_0(1, -2_i32, -4_i32) }) == (8)) as i32) != 0));
    return 0;
}
