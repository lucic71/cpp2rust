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
pub struct anon_1 {
    pub elem: *mut *mut libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union anon_0 {
    pub set: anon_1,
    pub other: i32,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct entry {
    pub c: anon_0,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct holder {
    pub table: *mut entry,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: *mut holder =
        (libcc2rs::malloc_unsafe(::std::mem::size_of::<holder>()) as *mut holder);
    (*h).table = (libcc2rs::malloc_unsafe(::std::mem::size_of::<entry>()) as *mut entry);
    (*(*h).table.offset(((0) as isize))).c.set.elem =
        (libcc2rs::malloc_unsafe(::std::mem::size_of::<*mut libc::c_char>())
            as *mut *mut libc::c_char);
    (*(*(*h).table.offset(((0) as isize)))
        .c
        .set
        .elem
        .offset(((0) as isize))) =
        libcc2rs::strdup_unsafe((c"alpha".as_ptr().cast_mut()).cast_const());
    assert!(
        ((((libc::strcmp(
            (*(*(*h).table.offset(((0) as isize)))
                .c
                .set
                .elem
                .offset(((0) as isize)))
            .cast_const(),
            (c"alpha".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    libcc2rs::free_unsafe(
        (((*(*(*h).table.offset(((0) as isize)))
            .c
            .set
            .elem
            .offset(((0) as isize))) as *mut libc::c_char) as *mut ::libc::c_void),
    );
    (*(*(*h).table.offset(((0) as isize)))
        .c
        .set
        .elem
        .offset(((0) as isize))) = std::ptr::null_mut();
    assert!(
        ((((*(*(*h).table.offset(((0) as isize)))
            .c
            .set
            .elem
            .offset(((0) as isize)))
        .is_null()) as i32)
            != 0)
    );
    libcc2rs::free_unsafe(
        (((*(*h).table.offset(((0) as isize))).c.set.elem as *mut *mut libc::c_char)
            as *mut ::libc::c_void),
    );
    (*(*h).table.offset(((0) as isize))).c.set.elem = std::ptr::null_mut();
    assert!((((((*(*h).table.offset(((0) as isize))).c.set.elem).is_null()) as i32) != 0));
    libcc2rs::free_unsafe((((*h).table as *mut entry) as *mut ::libc::c_void));
    libcc2rs::free_unsafe(((h as *mut holder) as *mut ::libc::c_void));
    return 0;
}
