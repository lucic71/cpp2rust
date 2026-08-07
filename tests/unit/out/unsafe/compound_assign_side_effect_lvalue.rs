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
    let mut buf: [u8; 8] = [0_u8; 8];
    {
        let byte_0 = ((buf.as_mut_ptr() as *mut u8) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<[u8; 8]>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((buf.as_mut_ptr() as *mut u8) as *mut ::libc::c_void)
    };
    buf[((0) as usize)] = 1_u8;
    buf[((1) as usize)] = 2_u8;
    let mut p: *mut u8 = buf.as_mut_ptr();
    let __lhs = (&raw mut (*p.postfix_inc()) as *mut u8);
    (*__lhs) = ((((*__lhs) as i32) | 128) as u8);
    assert!(((((p) == (buf.as_mut_ptr().offset(((1) as isize)))) as i32) != 0));
    assert!(((((buf[((0) as usize)] as i32) == (129)) as i32) != 0));
    assert!(((((buf[((1) as usize)] as i32) == (2)) as i32) != 0));
    let mut r: *mut u8 = buf.as_mut_ptr();
    let __lhs = (&raw mut (*r.prefix_inc()) as *mut u8);
    (*__lhs) = ((((*__lhs) as i32) | 16) as u8);
    assert!(((((r) == (buf.as_mut_ptr().offset(((1) as isize)))) as i32) != 0));
    assert!(((((buf[((1) as usize)] as i32) == (18)) as i32) != 0));
    let mut words: [u32; 4] = [1_u32, 2_u32, 3_u32, 4_u32];
    let mut w: *mut u32 = words.as_mut_ptr();
    let __lhs = (&raw mut (*w.postfix_inc()) as *mut u32);
    (*__lhs) = (*__lhs).wrapping_add((10_u32 as u32));
    assert!(((((w) == (words.as_mut_ptr().offset(((1) as isize)))) as i32) != 0));
    assert!(((((words[((0) as usize)]) == (11_u32)) as i32) != 0));
    assert!(((((words[((1) as usize)]) == (2_u32)) as i32) != 0));
    let mut ptrs: [*mut u8; 2] = [std::ptr::null_mut(); 2];
    ptrs[((0) as usize)] = buf.as_mut_ptr();
    ptrs[((1) as usize)] = buf.as_mut_ptr();
    let mut pp: *mut *mut u8 = ptrs.as_mut_ptr();
    let __lhs = (&raw mut (*pp.postfix_inc()) as *mut *mut u8);
    (*__lhs) = (*__lhs).wrapping_add(((3 as i32) as usize));
    assert!(((((pp) == (ptrs.as_mut_ptr().offset(((1) as isize)))) as i32) != 0));
    assert!(((((ptrs[((0) as usize)]) == (buf.as_mut_ptr().offset(((3) as isize)))) as i32) != 0));
    assert!(((((ptrs[((1) as usize)]) == (buf.as_mut_ptr())) as i32) != 0));
    let mut q: *mut u8 = buf.as_mut_ptr();
    let mut v: i32 = (({
        let __lhs = (&raw mut (*q.postfix_inc()) as *mut u8);
        (*__lhs) = ((((*__lhs) as i32) | 64) as u8);
        (*__lhs)
    }) as i32);
    assert!(((((q) == (buf.as_mut_ptr().offset(((1) as isize)))) as i32) != 0));
    assert!(((((v) == (193)) as i32) != 0));
    assert!(((((buf[((0) as usize)] as i32) == (193)) as i32) != 0));
    return 0;
}
