extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut g_cursor_0: libcc2rs::LazyStorage<*mut *mut i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub unsafe fn advance_1() -> i32 {
    (*(*(*&raw const g_cursor_0).get())).prefix_inc();
    return 10;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub ptr: *mut i32,
}
impl S {
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).ptr).write((*_a0).ptr);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).ptr).write((*_a0).ptr);
    }
}
pub unsafe fn by_ref_2(r: *mut *mut i32) {
    (*(*r).offset((1) as isize)) += (unsafe { advance_1() });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut q: *mut i32 = a.as_mut_ptr();
    (*(*&raw const g_cursor_0).get()) = (&mut q as *mut *mut i32);
    (*q.offset((1) as isize)) = (unsafe { advance_1() });
    assert!(((q) == (a.as_mut_ptr().offset((1) as isize))) && ((a[(2) as usize]) == (10)));
    (*q.offset((1) as isize)) += (unsafe { advance_1() });
    assert!(((q) == (a.as_mut_ptr().offset((2) as isize))) && ((a[(3) as usize]) == (14)));
    let mut pq: *mut *mut i32 = (&mut q as *mut *mut i32);
    (*(*pq).offset((1) as isize)) += (unsafe { advance_1() });
    assert!(((q) == (a.as_mut_ptr().offset((3) as isize))) && ((a[(4) as usize]) == (15)));
    (*(*pq).offset((1) as isize)) = (unsafe { advance_1() });
    assert!(((q) == (a.as_mut_ptr().offset((4) as isize))) && ((a[(5) as usize]) == (10)));
    (unsafe { by_ref_2(&mut q) });
    assert!(((q) == (a.as_mut_ptr().offset((5) as isize))) && ((a[(6) as usize]) == (17)));
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).ptr).write(a.as_mut_ptr());
    };
    (*(*&raw const g_cursor_0).get()) = (&mut s.ptr as *mut *mut i32);
    (*s.ptr.offset((1) as isize)) += (unsafe { advance_1() });
    assert!(((s.ptr) == (a.as_mut_ptr().offset((1) as isize))) && ((a[(2) as usize]) == (20)));
    let mut sp: *mut S = (&mut s as *mut S);
    (*(*sp).ptr.offset((1) as isize)) += (unsafe { advance_1() });
    assert!(((s.ptr) == (a.as_mut_ptr().offset((2) as isize))) && ((a[(3) as usize]) == (24)));
    let mut b: *mut u8 = (a.as_mut_ptr() as *mut u8);
    (*b.offset((0) as isize)) = 7_u8;
    (*b.offset((4) as isize)) = (((*b.offset((4) as isize)) as i32) + 1) as u8;
    assert!(((a[(0) as usize]) == (7)) && ((a[(1) as usize]) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const g_cursor_0).get();
}
