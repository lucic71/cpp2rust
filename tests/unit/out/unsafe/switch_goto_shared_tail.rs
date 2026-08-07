extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut buf_0: [libc::c_char; 32] = unsafe { [(0 as libc::c_char); 32] };
pub static mut n_1: i32 = unsafe { 0_i32 };
pub unsafe fn emit_2(mut ch: libc::c_char) {
    buf_0[((n_1.postfix_inc()) as usize)] = ch;
}
pub unsafe fn step_3(mut c: i32, mut last: *mut i32) {
    goto_block!({
        '__entry: {
            match c {
                __v if __v == (')' as i32) => {
                    goto!('__f1_case);
                }
                __v if __v == ('(' as i32) => {
                    goto!('__f2_case);
                }
                __v if __v == ('.' as i32) => {
                    goto!('COPY);
                }
                __v if __v == ('^' as i32) => {
                    goto!('__f3_case);
                }
                _ => {
                    goto!('__default_1);
                }
            }
        }
        '__f1_case: {
            if !((((*last) == (0)) as i32) != 0) {
                goto!('__f4_join);
            }
        }
        '__f5_then: {
            goto!('ESCAPE);
        }
        '__f4_join: {
            goto!('COPY);
        }
        '__f2_case: {
            (*last) = ('(' as i32);
        }
        'COPY: {
            (unsafe { emit_2((c as libc::c_char)) });
            (*last) = c;
            goto!('__f0_swexit);
        }
        '__f3_case: {
            if !((((*last) == ('(' as i32)) as i32) != 0) {
                goto!('__f6_join);
            }
        }
        '__f7_then: {
            goto!('COPY);
        }
        '__f6_join: {}
        '__default_1: {
            if !(((((((c) == ('x' as i32)) as i32) != 0) || ((((c) == ('y' as i32)) as i32) != 0))
                as i32)
                != 0)
            {
                goto!('__f8_join);
            }
        }
        '__f9_then: {}
        'ESCAPE: {
            (unsafe { emit_2((('\\' as i32) as libc::c_char)) });
        }
        '__f8_join: {
            (unsafe { emit_2((c as libc::c_char)) });
            (*last) = 255;
            goto!('__f0_swexit);
        }
        '__f0_swexit: {}
    });
}
pub unsafe fn convert_4(mut s: *const libc::c_char) -> *const libc::c_char {
    let mut last: i32 = 0;
    n_1 = 0;
    'loop_: while ((*s) != 0) {
        (unsafe { step_3(((*s.postfix_inc()) as i32), (&raw mut last as *mut i32)) });
    }
    buf_0[((n_1) as usize)] = (0 as libc::c_char);
    return (buf_0.as_mut_ptr()).cast_const();
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((libc::strcmp(
            (unsafe { convert_4((c")a".as_ptr().cast_mut()).cast_const()) }),
            (c"\\)a".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (unsafe { convert_4((c"(.x".as_ptr().cast_mut()).cast_const()) }),
            (c"(.\\x".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (unsafe { convert_4((c"(^".as_ptr().cast_mut()).cast_const()) }),
            (c"(^".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (unsafe { convert_4((c"a^".as_ptr().cast_mut()).cast_const()) }),
            (c"a^".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (unsafe { convert_4((c"()".as_ptr().cast_mut()).cast_const()) }),
            (c"()".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (unsafe { convert_4((c"^x".as_ptr().cast_mut()).cast_const()) }),
            (c"^\\x".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (unsafe { convert_4((c")(".as_ptr().cast_mut()).cast_const()) }),
            (c"\\)(".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    return 0;
}
