extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn dispatch_0(mut kind: i32, mut v: i32) -> i32 {
    let mut acc: i32 = 0_i32;
    let mut scaled: i32 = 0_i32;
    goto_block!({
        '__entry: {
            acc = 0;
            scaled = 0;
            if !((((v) < (0)) as i32) != 0) {
                goto!('__f0_join);
            }
        }
        '__f1_then: {
            v = -v;
            goto!('negative_entry);
        }
        '__f0_join: {
            match kind {
                __v if __v == 1 => {
                    goto!('__f3_case);
                }
                __v if __v == 2 => {
                    goto!('__f4_case);
                }
                _ => {
                    goto!('__f5_case);
                }
            }
        }
        '__f3_case: {
            acc = ((v) + (1));
            goto!('__f2_swexit);
        }
        '__f4_case: {
            scaled = ((v) * (2));
        }
        'negative_entry: {
            acc = ((scaled) + (v));
            goto!('__f2_swexit);
        }
        '__f5_case: {
            acc = 999;
            goto!('__f2_swexit);
        }
        '__f2_swexit: {
            return acc;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn step_1(mut p: *const libc::c_char) -> i32 {
    let mut op: i32 = 0_i32;
    let mut acc: i32 = 0_i32;
    goto_block!({
        '__entry: {
            op = 0;
            acc = 0;
            if !(((((*p) as i32) == ('!' as i32)) as i32) != 0) {
                goto!('__f0_join);
            }
        }
        '__f1_then: {
            p.postfix_inc();
            goto!('forced);
        }
        '__f0_join: {
            match {
                op = ((*p.postfix_inc()) as i32);
                op
            } {
                __v if __v == ('a' as i32) => {
                    goto!('__f3_case);
                }
                __v if __v == ('b' as i32) => {
                    goto!('__f4_case);
                }
                _ => {
                    goto!('__f5_case);
                }
            }
        }
        '__f3_case: {
            acc = 1;
            goto!('__f2_swexit);
        }
        '__f4_case: {
            acc = 2;
        }
        'forced: {
            acc += 10;
            goto!('__f2_swexit);
        }
        '__f5_case: {
            acc = 100;
            goto!('__f2_swexit);
        }
        '__f2_swexit: {
            return ((acc) + (op));
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
        ((((unsafe { step_1((c"a".as_ptr().cast_mut()).cast_const()) }) == ((1) + ('a' as i32)))
            as i32)
            != 0)
    );
    assert!(
        ((((unsafe { step_1((c"b".as_ptr().cast_mut()).cast_const()) }) == ((12) + ('b' as i32)))
            as i32)
            != 0)
    );
    assert!(
        ((((unsafe { step_1((c"z".as_ptr().cast_mut()).cast_const()) }) == ((100) + ('z' as i32)))
            as i32)
            != 0)
    );
    assert!(
        ((((unsafe { step_1((c"!x".as_ptr().cast_mut()).cast_const()) }) == (10)) as i32) != 0)
    );
    assert!(((((unsafe { dispatch_0(1, 5) }) == (6)) as i32) != 0));
    assert!(((((unsafe { dispatch_0(2, 5) }) == (15)) as i32) != 0));
    assert!(((((unsafe { dispatch_0(7, 5) }) == (999)) as i32) != 0));
    assert!(((((unsafe { dispatch_0(7, -5_i32) }) == (5)) as i32) != 0));
    assert!(((((unsafe { dispatch_0(1, -3_i32) }) == (3)) as i32) != 0));
    return 0;
}
