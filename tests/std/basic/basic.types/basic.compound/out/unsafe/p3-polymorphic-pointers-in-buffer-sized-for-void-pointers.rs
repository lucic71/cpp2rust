extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut first_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut third_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe trait DeclaresVirtual {
    unsafe fn f(&mut self);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ThirdOverrider {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut first: FirstOverrider = <FirstOverrider>::default();
    let mut second: SecondOverrider = <SecondOverrider>::default();
    let mut third: ThirdOverrider = <ThirdOverrider>::default();
    let mut table: *mut *mut dyn DeclaresVirtual = (libcc2rs::malloc_unsafe(
        (3_usize).wrapping_mul((::std::mem::size_of::<*mut ::libc::c_void>() as usize)),
    ) as *mut *mut dyn DeclaresVirtual);
    (*table.offset((0) as isize)) = (&mut first as *mut FirstOverrider);
    (*table.offset((1) as isize)) = (&mut second as *mut SecondOverrider);
    (*table.offset((2) as isize)) = (&mut third as *mut ThirdOverrider);
    assert!(((*table.offset((0) as isize)) == (&mut first as *mut FirstOverrider)));
    assert!(((*table.offset((1) as isize)) == (&mut second as *mut SecondOverrider)));
    assert!(((*table.offset((2) as isize)) == (&mut third as *mut ThirdOverrider)));
    (unsafe { (*(*table.offset((0) as isize))).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut third_calls_2)) == (0))
    );
    (unsafe { (*(*table.offset((1) as isize))).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut third_calls_2)) == (0))
    );
    (unsafe { (*(*table.offset((2) as isize))).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut third_calls_2)) == (1))
    );
    libcc2rs::free_unsafe((table as *mut ::libc::c_void));
    return 0;
}
unsafe impl DeclaresVirtual for FirstOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)).prefix_inc();
    }
}
unsafe impl DeclaresVirtual for SecondOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)).prefix_inc();
    }
}
unsafe impl DeclaresVirtual for ThirdOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut third_calls_2)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const first_calls_0);
    std::cell::LazyCell::force(&*&raw const second_calls_1);
    std::cell::LazyCell::force(&*&raw const third_calls_2);
}
