extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut base_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresVirtual {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Overrides {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: Overrides = <Overrides>::default();
    let mut original: *mut DeclaresVirtual = (&mut d as *mut Overrides);
    let mut copied: *mut DeclaresVirtual = std::ptr::null_mut();
    match (
        { ((&mut copied as *mut *mut DeclaresVirtual) as *mut ::libc::c_void) },
        { ((&mut original as *mut *mut DeclaresVirtual) as *const ::libc::c_void) },
    ) {
        (mut __tmp_0, mut __tmp_1) => {
            if ::std::mem::size_of::<*mut DeclaresVirtual>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    __tmp_1,
                    __tmp_0,
                    ::std::mem::size_of::<*mut DeclaresVirtual>(),
                )
            }
            __tmp_0
        }
    };
    assert!(((copied) == (original)));
    (unsafe { (*copied).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1))
    );
    return 0;
}
unsafe impl DeclaresVirtual for Overrides {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const derived_calls_1);
}
