extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: i32 = unsafe { 0 };
pub unsafe fn bump_1(mut by: i32) {
    total_0 += by;
}
pub unsafe fn reset_2(mut ignored: i32) {
    &(ignored);
    total_0 = 0;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct handlers {
    pub cb: Option<unsafe fn(i32)>,
    pub n: i32,
}
impl Default for handlers {
    fn default() -> Self {
        handlers { cb: None, n: 0_i32 }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: *mut handlers =
        (libcc2rs::calloc_unsafe(1_usize, ::std::mem::size_of::<handlers>()) as *mut handlers);
    assert!(!(h).is_null());
    assert!((((((*h).cb).is_none()) as i32) != 0));
    ((*h).cb) = Some(bump_1);
    (*h).n = 7;
    assert!((((((*h).cb) == (Some(bump_1))) as i32) != 0));
    assert!((((((*h).cb) != (Some(reset_2))) as i32) != 0));
    (unsafe { ((*h).cb).unwrap()(3) });
    assert!(((((total_0) == (3)) as i32) != 0));
    (unsafe { ((*h).cb).unwrap()(4) });
    assert!(((((total_0) == (7)) as i32) != 0));
    ((*h).cb) = Some(reset_2);
    (unsafe { ((*h).cb).unwrap()(0) });
    assert!(((((total_0) == (0)) as i32) != 0));
    assert!((((((*h).n) == (7)) as i32) != 0));
    ((*h).cb) = None;
    assert!((((((*h).cb).is_none()) as i32) != 0));
    libcc2rs::free_unsafe(((h as *mut handlers) as *mut ::libc::c_void));
    return 0;
}
