extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut f_calls_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe trait DeclaresVirtual {
    unsafe fn f(&mut self);
}
pub unsafe fn setValueThroughPointer_1(mut object: *mut dyn DeclaresVirtual, mut value: i32) {
    (*object).value = value;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PassesThis {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: PassesThis = <PassesThis>::default();
    let mut base: *mut dyn DeclaresVirtual = (&mut d as *mut PassesThis);
    (unsafe { (*base).f() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut f_calls_0)) == (1)));
    assert!((((d as DeclaresVirtual).value) == (7)));
    assert!((((*base).value) == (7)));
    return 0;
}
unsafe impl DeclaresVirtual for PassesThis {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut f_calls_0)).prefix_inc();
        (unsafe { setValueThroughPointer_1((self as *mut PassesThis), 7) });
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const f_calls_0);
}
