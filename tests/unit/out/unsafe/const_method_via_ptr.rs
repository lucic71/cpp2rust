extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Default)]
pub struct Item {
    pub value: Option<Box<i32>>,
}
#[repr(C)]
#[derive(Default)]
pub struct Wrapper {
    pub items: Vec<Item>,
    pub queue: Vec<Item>,
}
pub unsafe fn count_0(mut w: *mut Wrapper, mut q: *mut Vec<Item>) -> i32 {
    return ((((*w).items.len() as i32) + (if (*w).queue.is_empty() { 1 } else { 0 }))
        + (if (*(q).cast_const()).is_empty() { 2 } else { 0 }));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut w: Wrapper = <Wrapper>::default();
    assert!(
        ((unsafe {
            let _w: *mut Wrapper = (&raw mut w as *mut Wrapper);
            let _q: *mut Vec<Item> = (&raw mut w.queue as *mut Vec<Item>);
            count_0(_w, _q)
        }) == (3))
    );
    return 0;
}
