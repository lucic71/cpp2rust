extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sm_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            switch!(match n {
                __v if __v == 0 => {
                    ret += 1;
                }
                __v if __v == 1 => {
                    ret += 10;
                    goto!('out);
                }
                __v if false => '__default_1: {
                    ret += 100;
                    break;
                }
                _ => {
                    goto!('__default_1);
                }
            });
            ret += 1000;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn scan_1(mut p: *const libc::c_char) -> i32 {
    let mut c: i32 = 0_i32;
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            c = 0;
            ret = 0;
            switch!(match {
                c = ((*p.postfix_inc()) as i32);
                c
            } {
                __v if __v == ('a' as i32) => {
                    ret = 1;
                }
                __v if __v == ('b' as i32) => {
                    ret += 10;
                    goto!('out);
                }
                __v if false => '__default_3: {
                    ret = 100;
                    break;
                }
                _ => {
                    goto!('__default_3);
                }
            });
            ret += 1000;
        }
        'out: {
            return ((ret) + (c));
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
    assert!(((((unsafe { sm_0(0) }) == (11)) as i32) != 0));
    assert!(((((unsafe { sm_0(1) }) == (10)) as i32) != 0));
    assert!(((((unsafe { sm_0(9) }) == (1100)) as i32) != 0));
    assert!(
        ((((unsafe { scan_1((c"a".as_ptr().cast_mut()).cast_const()) }) == ((11) + ('a' as i32)))
            as i32)
            != 0)
    );
    assert!(
        ((((unsafe { scan_1((c"b".as_ptr().cast_mut()).cast_const()) }) == ((10) + ('b' as i32)))
            as i32)
            != 0)
    );
    assert!(
        ((((unsafe { scan_1((c"z".as_ptr().cast_mut()).cast_const()) }) == ((1100) + ('z' as i32)))
            as i32)
            != 0)
    );
    return 0;
}
