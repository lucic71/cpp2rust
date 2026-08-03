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
pub struct item {
    pub value: i32,
}
pub unsafe fn read_item_0(mut it: *mut item) -> i32 {
    return (((*it).value) + (1));
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct holder {
    pub callback: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
}
impl Default for holder {
    fn default() -> Self {
        holder { callback: None }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: *mut holder =
        (libcc2rs::calloc_unsafe(1_usize, ::std::mem::size_of::<holder>()) as *mut holder);
    ((*h).callback) = std::mem::transmute::<
        Option<unsafe fn(*mut item) -> i32>,
        Option<unsafe fn(*mut ::libc::c_void) -> i32>,
    >(Some(read_item_0));
    let mut it: item = <item>::default();
    it.value = 41;
    assert!(
        ((((unsafe {
            ((*h).callback).unwrap()((((&mut it as *mut item) as *mut item) as *mut ::libc::c_void))
        }) == (42)) as i32)
            != 0)
    );
    libcc2rs::free_unsafe(((h as *mut holder) as *mut ::libc::c_void));
    return 0;
}
