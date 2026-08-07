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
pub struct Handler {
    pub tag: i32,
    pub fn_: Option<unsafe fn()>,
}
impl Default for Handler {
    fn default() -> Self {
        Handler {
            tag: 0_i32,
            fn_: None,
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut data: i32 = 42;
    let mut p: *mut ::libc::c_void =
        (((&raw mut data as *mut i32) as *mut i32) as *mut ::libc::c_void);
    let mut a: Handler = <Handler>::default();
    {
        let byte_0 =
            (((&raw mut a as *mut Handler) as *mut Handler) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<Handler>() {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        (((&raw mut a as *mut Handler) as *mut Handler) as *mut ::libc::c_void)
    };
    {
        if ::std::mem::size_of::<*mut ::libc::c_void>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                (((&raw mut p as *mut *mut ::libc::c_void) as *const *mut ::libc::c_void)
                    as *const ::libc::c_void),
                (((&raw mut (a.fn_) as *mut Option<unsafe fn()>) as *mut Option<unsafe fn()>)
                    as *mut ::libc::c_void),
                ::std::mem::size_of::<*mut ::libc::c_void>() as usize,
            )
        }
        (((&raw mut (a.fn_) as *mut Option<unsafe fn()>) as *mut Option<unsafe fn()>)
            as *mut ::libc::c_void)
    };
    let mut b: Handler = <Handler>::default();
    {
        if ::std::mem::size_of::<Handler>() != 0 {
            ::std::ptr::copy_nonoverlapping(
                (((&raw mut a as *mut Handler) as *const Handler) as *const ::libc::c_void),
                (((&raw mut b as *mut Handler) as *mut Handler) as *mut ::libc::c_void),
                ::std::mem::size_of::<Handler>() as usize,
            )
        }
        (((&raw mut b as *mut Handler) as *mut Handler) as *mut ::libc::c_void)
    };
    assert!(((((b.tag) == (0)) as i32) != 0));
    assert!((((!((b.fn_).is_none())) as i32) != 0));
    assert!(((((data) == (42)) as i32) != 0));
    return 0;
}
