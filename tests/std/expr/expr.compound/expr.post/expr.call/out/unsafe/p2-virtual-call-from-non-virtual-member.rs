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
impl DeclaresVirtual {
    pub unsafe fn callsF(&mut self) {
        (unsafe { self.f() });
    }
}
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
    let mut onlyBase: DeclaresVirtual = <DeclaresVirtual>::default();
    (unsafe { DeclaresVirtual::callsF(&mut onlyBase) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (0))
    );
    let mut d: Overrides = <Overrides>::default();
    (unsafe { DeclaresVirtual::callsF(&mut (d as DeclaresVirtual)) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1))
    );
    let mut base: *mut DeclaresVirtual = (&mut d as *mut Overrides);
    (unsafe { DeclaresVirtual::callsF(&mut (*base)) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (2))
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
