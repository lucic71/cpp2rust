extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn scan_0(mut s: *const libc::c_char, mut start_inside: i32) -> i32 {
    let mut depth: i32 = 0_i32;
    let mut seen: i32 = 0_i32;
    let mut i: i32 = 0_i32;
    goto_block!({
        '__entry: {
            depth = 0;
            seen = 0;
            i = 0;
            if !(start_inside != 0) {
                goto!('__f0_join);
            }
        }
        '__f1_then: {
            goto!('inside);
        }
        '__f0_join: {}
        '__f2_cond: {
            if !((*s.offset(((i) as isize))) != 0) {
                goto!('__f3_exit);
            }
        }
        '__f4_body: {
            if !(((((*s.offset(((i) as isize))) as i32) == ('(' as i32)) as i32) != 0) {
                goto!('__f5_join);
            }
        }
        '__f6_then: {
            i.postfix_inc();
        }
        'inside: {
            depth.postfix_inc();
            seen.postfix_inc();
            if !((((depth) > (3)) as i32) != 0) {
                goto!('__f7_join);
            }
        }
        '__f8_then: {
            goto!('__f3_exit);
        }
        '__f7_join: {
            goto!('__f2_cond);
        }
        '__f5_join: {
            i.postfix_inc();
            goto!('__f2_cond);
        }
        '__f3_exit: {
            return (((depth) * (10)) + (seen));
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
    assert!(
        ((((unsafe { scan_0((c"".as_ptr().cast_mut()).cast_const(), 0) }) == (0)) as i32) != 0)
    );
    assert!(
        ((((unsafe { scan_0((c"(()".as_ptr().cast_mut()).cast_const(), 0) }) == (22)) as i32) != 0)
    );
    assert!(
        ((((unsafe { scan_0((c"ab(cd".as_ptr().cast_mut()).cast_const(), 0) }) == (11)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe { scan_0((c"".as_ptr().cast_mut()).cast_const(), 1) }) == (11)) as i32) != 0)
    );
    assert!(
        ((((unsafe { scan_0((c"((((((".as_ptr().cast_mut()).cast_const(), 0) }) == (44)) as i32)
            != 0)
    );
    return 0;
}
