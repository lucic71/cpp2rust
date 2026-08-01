extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr {
    pub n: i32,
    pub name: [libc::c_char; 1],
}
impl Default for hdr {
    fn default() -> Self {
        hdr {
            n: 0_i32,
            name: [(0 as libc::c_char); 1],
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: *mut hdr = (libcc2rs::calloc_unsafe(
        1_usize,
        (::std::mem::size_of::<hdr>() as usize).wrapping_add(8_usize),
    ) as *mut hdr);
    {
        if 8_usize != 0 {
            ::std::ptr::copy_nonoverlapping(
                ((c"abcdefg".as_ptr().cast_mut() as *const libc::c_char) as *const ::libc::c_void),
                (((*h).name.as_mut_ptr() as *mut libc::c_char) as *mut ::libc::c_void),
                8_usize as usize,
            )
        }
        (((*h).name.as_mut_ptr() as *mut libc::c_char) as *mut ::libc::c_void)
    };
    (*h).n = 5;
    assert!((((((*h).n) == (5)) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            ((*h).name.as_mut_ptr()).cast_const(),
            (c"abcdefg".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    libcc2rs::free_unsafe(((h as *mut hdr) as *mut ::libc::c_void));
    return 0;
}
