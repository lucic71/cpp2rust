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
    let mut arr: [i32; 4] = [1, 2, 3, 4];
    let mut end: *mut i32 = arr.as_mut_ptr().offset(((4) as isize));
    {
        let byte_0 = ((end as *mut i32) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..0_usize {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((end as *mut i32) as *mut ::libc::c_void)
    };
    {
        if 0_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((arr.as_mut_ptr() as *const i32) as *const ::libc::c_void),
                ((end as *mut i32) as *mut ::libc::c_void),
                0_usize as usize,
            )
        }
        ((end as *mut i32) as *mut ::libc::c_void)
    };
    assert!(((((arr[((0) as usize)]) == (1)) as i32) != 0));
    assert!(((((arr[((3) as usize)]) == (4)) as i32) != 0));
    return 0;
}
