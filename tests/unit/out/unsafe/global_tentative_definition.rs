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
pub struct ops_struct {
    pub first: Option<unsafe fn(i32) -> i32>,
    pub second: Option<unsafe fn(i32) -> i32>,
}
impl Default for ops_struct {
    fn default() -> Self {
        ops_struct {
            first: None,
            second: None,
        }
    }
}
pub unsafe fn twice_2(mut v: i32) -> i32 {
    return ((v) * (2));
}
pub static mut table_0: ops_struct = unsafe {
    ops_struct {
        first: None,
        second: Some(twice_2),
    }
};
pub static mut limits_1: [i32; 3] = unsafe { [4, 5, 6] };
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((table_0.first).is_none()) as i32) != 0));
    assert!((((!((table_0.second).is_none())) as i32) != 0));
    assert!(((((unsafe { (table_0.second).unwrap()(7) }) == (14)) as i32) != 0));
    assert!(((((limits_1[((1) as usize)]) == (5)) as i32) != 0));
    return 0;
}
