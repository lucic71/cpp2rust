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
pub union anon_0 {
    pub p: *const libc::c_char,
    pub n: i32,
    pub c: [libc::c_char; 4],
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct item_struct {
    pub tag: i32,
    pub u: anon_0,
}
pub static mut items_1: [item_struct; 3] = unsafe {
    [
        item_struct {
            tag: 0,
            u: anon_0 {
                p: (c"xy".as_ptr().cast_mut()).cast_const(),
            },
        },
        item_struct {
            tag: 1,
            u: anon_0 { n: 42 },
        },
        item_struct {
            tag: 2,
            u: anon_0 {
                c: std::mem::transmute(*b"ab\0\0"),
            },
        },
    ]
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut it: *const item_struct = (&raw const items_1[((0) as usize)] as *const item_struct);
    assert!(
        ((((libc::strcmp((*it).u.p, (c"xy".as_ptr().cast_mut()).cast_const())) == (0)) as i32)
            != 0)
    );
    assert!(((((items_1[((1) as usize)].u.n) == (42)) as i32) != 0));
    assert!(((((items_1[((2) as usize)].u.c[((1) as usize)] as i32) == ('b' as i32)) as i32) != 0));
    return 0;
}
