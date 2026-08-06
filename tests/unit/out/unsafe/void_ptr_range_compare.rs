extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct region {
    pub start: *mut ::libc::c_void,
    pub mid: *mut ::libc::c_void,
    pub end: *mut ::libc::c_void,
}
pub unsafe fn in_low_half_0(mut r: *mut region, mut p: *mut ::libc::c_void) -> i32 {
    return ((((((p) >= ((*r).start)) as i32) != 0) && ((((p) < ((*r).mid)) as i32) != 0)) as i32);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: *mut libc::c_char = (libcc2rs::malloc_unsafe(64_usize) as *mut libc::c_char);
    let mut r: region = <region>::default();
    r.start = ((buf as *mut libc::c_char) as *mut ::libc::c_void);
    r.mid = ((buf.offset(((32) as isize)) as *mut libc::c_char) as *mut ::libc::c_void);
    r.end = ((buf.offset(((64) as isize)) as *mut libc::c_char) as *mut ::libc::c_void);
    assert!(((((r.start) < (r.mid)) as i32) != 0));
    assert!(((((r.mid) < (r.end)) as i32) != 0));
    assert!(
        ((unsafe {
            in_low_half_0(
                (&raw mut r as *mut region),
                ((buf.offset(((10) as isize)) as *mut libc::c_char) as *mut ::libc::c_void),
            )
        }) != 0)
    );
    assert!(
        ((!((unsafe {
            in_low_half_0(
                (&raw mut r as *mut region),
                ((buf.offset(((40) as isize)) as *mut libc::c_char) as *mut ::libc::c_void),
            )
        }) != 0) as i32)
            != 0)
    );
    assert!(
        ((unsafe {
            in_low_half_0(
                (&raw mut r as *mut region),
                ((buf as *mut libc::c_char) as *mut ::libc::c_void),
            )
        }) != 0)
    );
    assert!(
        ((!((unsafe {
            in_low_half_0(
                (&raw mut r as *mut region),
                ((buf.offset(((32) as isize)) as *mut libc::c_char) as *mut ::libc::c_void),
            )
        }) != 0) as i32)
            != 0)
    );
    let mut other: *mut libc::c_char = (libcc2rs::malloc_unsafe(64_usize) as *mut libc::c_char);
    let mut op: *mut ::libc::c_void = ((other as *mut libc::c_char) as *mut ::libc::c_void);
    assert!(
        ((!(((((((op) >= (r.start)) as i32) != 0) && ((((op) < (r.end)) as i32) != 0)) as i32) != 0)
            as i32)
            != 0)
    );
    libcc2rs::free_unsafe(((other as *mut libc::c_char) as *mut ::libc::c_void));
    libcc2rs::free_unsafe(((buf as *mut libc::c_char) as *mut ::libc::c_void));
    return 0;
}
