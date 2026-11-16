extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait DeclaresVirtuals {
    unsafe fn run(&mut self);
    unsafe fn increment(&mut self);
}
pub static mut other_pointer_to_object_0: std::cell::LazyCell<*mut dyn DeclaresVirtuals> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub unsafe fn incrementThroughOtherPointer_1() {
    (unsafe {
        (*(*std::cell::LazyCell::force_mut(&mut *&raw mut other_pointer_to_object_0))).increment()
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReEntered {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: ReEntered = <ReEntered>::default();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut other_pointer_to_object_0)) =
        (&mut d as *mut ReEntered);
    let mut base: *mut dyn DeclaresVirtuals = (&mut d as *mut ReEntered);
    (unsafe { (*base).run() });
    assert!((((d as DeclaresVirtuals).value) == (111)));
    (unsafe { (*base).increment() });
    assert!((((d as DeclaresVirtuals).value) == (112)));
    return 0;
}
unsafe impl DeclaresVirtuals for ReEntered {
    unsafe fn run(&mut self) {
        self.value = 10;
        (unsafe { incrementThroughOtherPointer_1() });
        self.value += 100;
    }
    unsafe fn increment(&mut self) {
        self.value.prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const other_pointer_to_object_0);
}
