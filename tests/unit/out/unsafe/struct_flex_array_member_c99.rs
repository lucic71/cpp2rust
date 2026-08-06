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
pub struct entry {
    pub id: i32,
    pub weight: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct table {
    pub n: i32,
    pub a: [entry; 0],
}
impl Default for table {
    fn default() -> Self {
        table { n: 0_i32, a: [] }
    }
}
pub unsafe fn table_create_0(mut n: i32) -> *mut table {
    let mut raw_: *mut ::libc::c_void = libcc2rs::malloc_unsafe(
        (::std::mem::offset_of!(table, a) as usize)
            .wrapping_add((n as usize).wrapping_mul((::std::mem::size_of::<entry>() as usize))),
    );
    let mut t: *mut table = (raw_ as *mut table);
    (*t).n = n;
    let mut i: i32 = 0;
    'loop_: while ((((i) < (n)) as i32) != 0) {
        (*(*t).a.as_mut_ptr().add((i) as usize)).id = ((i) * (10));
        (*(*t).a.as_mut_ptr().add((i) as usize)).weight = ((i) + (1));
        i.postfix_inc();
    }
    return t;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((((::std::mem::size_of::<table>()) == (::std::mem::offset_of!(table, a))) as i32) != 0)
    );
    let mut t: *mut table = (unsafe { table_create_0(3) });
    assert!((((((*t).n) == (3)) as i32) != 0));
    assert!((((((*(*t).a.as_mut_ptr().add((0) as usize)).id) == (0)) as i32) != 0));
    assert!((((((*(*t).a.as_mut_ptr().add((2) as usize)).id) == (20)) as i32) != 0));
    assert!((((((*(*t).a.as_mut_ptr().add((2) as usize)).weight) == (3)) as i32) != 0));
    (*(*t).a.as_mut_ptr().add((1) as usize)).id = 99;
    assert!((((((*(*t).a.as_mut_ptr().add((1) as usize)).id) == (99)) as i32) != 0));
    assert!((((((*(*t).a.as_mut_ptr().add((0) as usize)).id) == (0)) as i32) != 0));
    let mut next: *mut table = std::ptr::null_mut();
    assert!(((((next).is_null()) as i32) != 0));
    libcc2rs::free_unsafe(((t as *mut table) as *mut ::libc::c_void));
    return 0;
}
