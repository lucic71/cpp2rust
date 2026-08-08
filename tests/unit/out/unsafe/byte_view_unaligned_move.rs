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
    let mut words: [u32; 4] = [0_u32, 0_u32, 0_u32, 0_u32];
    let mut bytes: *mut u8 = (words.as_mut_ptr() as *mut u8);
    let mut i: i32 = 0;
    'loop_: while ((((i) < (6)) as i32) != 0) {
        (*bytes.offset((((6) + (i)) as isize))) = (((i) + (1)) as u8);
        i.postfix_inc();
    }
    (*bytes.offset(((12) as isize))) = 170_u8;
    {
        if 6_usize != 0 {
            ::std::ptr::copy(
                ((bytes.offset(((6) as isize)) as *const u8) as *const ::libc::c_void),
                ((bytes.offset(((7) as isize)) as *mut u8) as *mut ::libc::c_void),
                6_usize as usize,
            )
        }
        ((bytes.offset(((7) as isize)) as *mut u8) as *mut ::libc::c_void)
    };
    assert!((((((*bytes.offset(((6) as isize))) as i32) == (1)) as i32) != 0));
    assert!((((((*bytes.offset(((7) as isize))) as i32) == (1)) as i32) != 0));
    assert!((((((*bytes.offset(((8) as isize))) as i32) == (2)) as i32) != 0));
    assert!((((((*bytes.offset(((9) as isize))) as i32) == (3)) as i32) != 0));
    assert!((((((*bytes.offset(((10) as isize))) as i32) == (4)) as i32) != 0));
    assert!((((((*bytes.offset(((11) as isize))) as i32) == (5)) as i32) != 0));
    assert!((((((*bytes.offset(((12) as isize))) as i32) == (6)) as i32) != 0));
    assert!((((((*bytes.offset(((13) as isize))) as i32) == (0)) as i32) != 0));
    let mut src: [u8; 7] = [9_u8, 8_u8, 7_u8, 6_u8, 5_u8, 4_u8, 90_u8];
    {
        if 7_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((src.as_mut_ptr() as *const u8) as *const ::libc::c_void),
                ((bytes.offset(((2) as isize)) as *mut u8) as *mut ::libc::c_void),
                7_usize as usize,
            )
        }
        ((bytes.offset(((2) as isize)) as *mut u8) as *mut ::libc::c_void)
    };
    assert!((((((*bytes.offset(((1) as isize))) as i32) == (0)) as i32) != 0));
    assert!((((((*bytes.offset(((2) as isize))) as i32) == (9)) as i32) != 0));
    assert!((((((*bytes.offset(((7) as isize))) as i32) == (4)) as i32) != 0));
    assert!((((((*bytes.offset(((8) as isize))) as i32) == (90)) as i32) != 0));
    assert!((((((*bytes.offset(((9) as isize))) as i32) == (3)) as i32) != 0));
    return 0;
}
