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
pub struct holder {
    pub mask: [u8; 4],
    pub after: u32,
}
impl Default for holder {
    fn default() -> Self {
        holder {
            mask: [0_u8; 4],
            after: 0_u32,
        }
    }
}
pub unsafe fn encode_0(mut h: *mut holder, mut out: *mut u8) {
    (*((&raw mut (*h).mask as *mut [u8; 4]) as *mut u8)) = 7_u8;
    {
        if ::std::mem::size_of::<[u8; 4]>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                (((&raw mut (*h).mask as *mut [u8; 4]) as *const [u8; 4]) as *const ::libc::c_void),
                ((out as *mut u8) as *mut ::libc::c_void),
                ::std::mem::size_of::<[u8; 4]>() as usize,
            )
        }
        ((out as *mut u8) as *mut ::libc::c_void)
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: holder = holder {
        mask: [1_u8, 2_u8, 3_u8, 4_u8],
        after: 1432778632_u32,
    };
    let mut out: [u8; 4] = [0_u8; 4];
    (unsafe { encode_0((&raw mut h as *mut holder), out.as_mut_ptr()) });
    assert!(((((out[((0) as usize)] as i32) == (7)) as i32) != 0));
    assert!(((((out[((3) as usize)] as i32) == (4)) as i32) != 0));
    assert!(((((h.after) == (1432778632_u32)) as i32) != 0));
    return 0;
}
