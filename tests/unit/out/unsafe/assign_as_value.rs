extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [libc::c_char; 2] = [(0 as libc::c_char); 2];
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut q: *mut libc::c_char = std::ptr::null_mut();
    q = {
        p = (p).wrapping_add(((1 as i32) as usize));
        p
    };
    assert!(((((q) == (buf.as_mut_ptr().offset(((1) as isize)))) as i32) != 0));
    let mut src: [libc::c_char; 2] = [
        (('a' as i32) as libc::c_char),
        (('b' as i32) as libc::c_char),
    ];
    let mut dst: [libc::c_char; 2] = [
        (('x' as i32) as libc::c_char),
        (('y' as i32) as libc::c_char),
    ];
    let mut s: *mut libc::c_char = src.as_mut_ptr();
    let mut d: *mut libc::c_char = dst.as_mut_ptr();
    let mut last: libc::c_char = {
        let __rhs = (*s.postfix_inc());
        (*d.postfix_inc()) = __rhs;
        __rhs
    };
    assert!(((((last as i32) == ('a' as i32)) as i32) != 0));
    assert!(
        (((((((d) == (dst.as_mut_ptr().offset(((1) as isize)))) as i32) != 0)
            && ((((s) == (src.as_mut_ptr().offset(((1) as isize)))) as i32) != 0))
            as i32)
            != 0)
    );
    assert!(
        (((((((dst[((0) as usize)] as i32) == ('a' as i32)) as i32) != 0)
            && ((((dst[((1) as usize)] as i32) == ('y' as i32)) as i32) != 0)) as i32)
            != 0)
    );
    let mut out: libc::c_char = (0 as libc::c_char);
    'switch: {
        let __match_cond = (({
            out = (('x' as i32) as libc::c_char);
            out
        }) as i32);
        match __match_cond {
            __v if __v == ('x' as i32) => {
                assert!((1 != 0));
                break 'switch;
            }
            _ => {
                assert!((0 != 0));
                break 'switch;
            }
        }
    };
    assert!(((((out as i32) == ('x' as i32)) as i32) != 0));
    return 0;
}
