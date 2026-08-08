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
pub struct Inner {
    pub value: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Outer {
    pub slots: [Inner; 2],
    pub cur: *mut Inner,
}
impl Default for Outer {
    fn default() -> Self {
        Outer {
            slots: [<Inner>::default(); 2],
            cur: std::ptr::null_mut(),
        }
    }
}
pub unsafe fn set_current_0(mut p: *mut Outer, mut src: *const i32) {
    (*(*p).cur).value = (*src);
}
pub unsafe fn bump_current_1(mut p: *mut Outer) {
    (*(*p).cur).value = (((*p).slots[((0) as usize)].value) + (1));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut Outer = (libcc2rs::malloc_unsafe(::std::mem::size_of::<Outer>()) as *mut Outer);
    let mut a: i32 = 7;
    let mut b: i32 = 8;
    (*p).slots[((0) as usize)].value = 1;
    (*p).slots[((1) as usize)].value = 2;
    (*p).cur = (&raw mut (*p).slots[((0) as usize)] as *mut Inner);
    (unsafe { set_current_0(p, (&raw mut a as *mut i32).cast_const()) });
    assert!((((((*p).slots[((0) as usize)].value) == (7)) as i32) != 0));
    (*p).cur = (&raw mut (*p).slots[((1) as usize)] as *mut Inner);
    (unsafe { set_current_0(p, (&raw mut b as *mut i32).cast_const()) });
    assert!((((((*p).slots[((1) as usize)].value) == (8)) as i32) != 0));
    (unsafe { bump_current_1(p) });
    assert!((((((*p).slots[((1) as usize)].value) == (8)) as i32) != 0));
    libcc2rs::free_unsafe(((p as *mut Outer) as *mut ::libc::c_void));
    return 0;
}
