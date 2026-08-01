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
pub struct bin {
    pub idx: i32,
    pub buf: [libc::c_char; 8],
}
impl Default for bin {
    fn default() -> Self {
        bin {
            idx: 0_i32,
            buf: [(0 as libc::c_char); 8],
        }
    }
}
pub unsafe fn store_0(mut p: *mut ::libc::c_void, mut c: libc::c_char) {
    let mut b: *mut bin = (p as *mut bin);
    (*b).buf[(((*b).idx.postfix_inc()) as usize)] = c;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut b: *mut bin =
        (libcc2rs::calloc_unsafe(1_usize, ::std::mem::size_of::<bin>()) as *mut bin);
    (unsafe {
        store_0(
            ((b as *mut bin) as *mut ::libc::c_void),
            (('a' as i32) as libc::c_char),
        )
    });
    (unsafe {
        store_0(
            ((b as *mut bin) as *mut ::libc::c_void),
            (('b' as i32) as libc::c_char),
        )
    });
    assert!((((((*b).idx) == (2)) as i32) != 0));
    assert!((((((*b).buf[((0) as usize)] as i32) == ('a' as i32)) as i32) != 0));
    assert!((((((*b).buf[((1) as usize)] as i32) == ('b' as i32)) as i32) != 0));
    libcc2rs::free_unsafe(((b as *mut bin) as *mut ::libc::c_void));
    return 0;
}
