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
    pub size: i64,
    pub idx: i64,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct anon_2 {
    pub min: i32,
    pub max: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union anon_0 {
    pub set: anon_1,
    pub range: anon_2,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct entry {
    pub kind: i32,
    pub c: anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut table: *mut entry =
        (libcc2rs::malloc_unsafe((2_usize).wrapping_mul((::std::mem::size_of::<entry>() as usize)))
            as *mut entry);
    assert!((((!((table).is_null())) as i32) != 0));
    {
        let byte_0 = ((table as *mut entry) as *mut ::libc::c_void) as *mut u8;
        for offset in 0..(2_usize).wrapping_mul((::std::mem::size_of::<entry>() as usize)) {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((table as *mut entry) as *mut ::libc::c_void)
    };
    let mut e: *mut entry = (&raw mut (*table.offset(((0) as isize))) as *mut entry);
    (*e).kind = 1;
    (*e).c.set.size = 7_i64;
    (*e).c.set.idx = 3_i64;
    (*e).c.set.elem = (libcc2rs::malloc_unsafe(::std::mem::size_of::<*mut libc::c_char>())
        as *mut *mut libc::c_char);
    assert!((((!(((*e).c.set.elem).is_null())) as i32) != 0));
    (*(*e).c.set.elem.offset(((0) as isize))) =
        libcc2rs::strdup_unsafe((c"alpha".as_ptr().cast_mut()).cast_const());
    assert!((((!((*(*e).c.set.elem.offset(((0) as isize))).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            (*(*e).c.set.elem.offset(((0) as isize))).cast_const(),
            (c"alpha".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!((((((*e).c.set.size) == (7_i64)) as i32) != 0));
    assert!((((((*e).c.set.idx) == (3_i64)) as i32) != 0));
    assert!((((((*e).kind) == (1)) as i32) != 0));
    e = (&raw mut (*table.offset(((1) as isize))) as *mut entry);
    (*e).kind = 2;
    (*e).c.range.min = 10;
    (*e).c.range.max = 20;
    assert!((((((*e).c.range.min) == (10)) as i32) != 0));
    assert!((((((*e).c.range.max) == (20)) as i32) != 0));
    assert!((((((*table.offset(((0) as isize))).c.set.size) == (7_i64)) as i32) != 0));
    libcc2rs::free_unsafe(
        (((*(*table.offset(((0) as isize)))
            .c
            .set
            .elem
            .offset(((0) as isize))) as *mut libc::c_char) as *mut ::libc::c_void),
    );
    libcc2rs::free_unsafe(
        (((*table.offset(((0) as isize))).c.set.elem as *mut *mut libc::c_char)
            as *mut ::libc::c_void),
    );
    libcc2rs::free_unsafe(((table as *mut entry) as *mut ::libc::c_void));
    return 0;
}
