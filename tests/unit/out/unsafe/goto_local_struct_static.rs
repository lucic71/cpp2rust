extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn lookup_0(mut c: libc::c_char, mut fallback: i32) -> i32 {
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct Choice {
        pub key: libc::c_char,
        pub op: i32,
    }
    static mut aChoice_1: [Choice; 2] = unsafe {
        [
            Choice {
                key: (('a' as i32) as libc::c_char),
                op: 11,
            },
            Choice {
                key: (('b' as i32) as libc::c_char),
                op: 22,
            },
        ]
    };
    let mut i: i32 = 0_i32;
    let mut r: i32 = 0_i32;
    goto_block!({
        '__entry: {
            r = fallback;
            i = 0;
            'loop_: while (((i) < (2)) as i32) != 0 {
                if (((c as i32) == (aChoice_1[((i) as usize)].key as i32)) as i32) != 0 {
                    r = aChoice_1[((i) as usize)].op;
                    goto!('done);
                }
                i.postfix_inc();
            }
        }
        'done: {
            return r;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn classify_2(mut mode: i32, mut v: i32) -> i32 {
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct Weight {
        pub lo: i32,
        pub hi: i32,
    }
    static mut aWeight_3: [Weight; 2] =
        unsafe { [Weight { lo: 1, hi: 2 }, Weight { lo: 3, hi: 4 }] };
    let mut r: i32 = 0_i32;
    goto_block!({
        '__entry: {
            r = 0;
            if !((((v) > (0)) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {}
        'positive: {
            r = ((aWeight_3[((0) as usize)].lo) + (v));
            if !((((mode) == (1)) as i32) != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('negative);
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
            goto!('positive);
        }
        '__f5_join: {}
        'negative: {
            r = ((aWeight_3[((1) as usize)].hi) - (v));
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
    assert!(
        ((((unsafe { lookup_0((('a' as i32) as libc::c_char), -1_i32) }) == (11)) as i32) != 0)
    );
    assert!(
        ((((unsafe { lookup_0((('b' as i32) as libc::c_char), -1_i32) }) == (22)) as i32) != 0)
    );
    assert!(
        ((((unsafe { lookup_0((('z' as i32) as libc::c_char), -1_i32) }) == (-1_i32)) as i32) != 0)
    );
    assert!(((((unsafe { classify_2(0, 5) }) == (6)) as i32) != 0));
    assert!(((((unsafe { classify_2(1, 5) }) == (-1_i32)) as i32) != 0));
    assert!(((((unsafe { classify_2(0, -3_i32) }) == (7)) as i32) != 0));
    assert!(((((unsafe { classify_2(2, -3_i32) }) == (-2_i32)) as i32) != 0));
    return 0;
}
