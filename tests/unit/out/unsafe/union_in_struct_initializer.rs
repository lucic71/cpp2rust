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
    pub next: *mut Item,
    pub tag: i64,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Item {
    pub kind: i32,
    pub handler: Option<unsafe fn(i32) -> i32>,
    pub u: anon_0,
}
impl Default for Item {
    fn default() -> Self {
        Item {
            kind: 0_i32,
            handler: None,
            u: <anon_0>::default(),
        }
    }
}
pub unsafe fn double_it_1(mut x: i32) -> i32 {
    return ((x) * (2));
}
pub unsafe fn negate_2(mut x: i32) -> i32 {
    return -x;
}
pub static mut items_3: [Item; 2] = unsafe {
    [
        Item {
            kind: 1,
            handler: Some(double_it_1),
            u: anon_0 {
                next: std::ptr::null_mut(),
            },
        },
        Item {
            kind: 2,
            handler: Some(negate_2),
            u: anon_0 {
                next: std::ptr::null_mut(),
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
    assert!(((((unsafe { (items_3[((0) as usize)].handler).unwrap()(21) }) == (42)) as i32) != 0));
    assert!(
        ((((unsafe { (items_3[((1) as usize)].handler).unwrap()(21) }) == (-21_i32)) as i32) != 0)
    );
    assert!(((((items_3[((0) as usize)].u.next).is_null()) as i32) != 0));
    items_3[((0) as usize)].u.tag = 7_i64;
    assert!(((((items_3[((0) as usize)].u.tag) == (7_i64)) as i32) != 0));
    return 0;
}
