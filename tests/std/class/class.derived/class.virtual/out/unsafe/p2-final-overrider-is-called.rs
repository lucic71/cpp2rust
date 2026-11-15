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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DoesNotOverride {}
pub unsafe fn callThroughReference_2(r: *mut DeclaresVirtual) {
    (unsafe { (*r).f() });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: Overrides = <Overrides>::default();
    let mut p: *mut DeclaresVirtual = (&mut d as *mut Overrides);
    (unsafe { (*p).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1))
    );
    (unsafe { callThroughReference_2(&mut d) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (2))
    );
    (unsafe { d.f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (3))
    );
    let mut n: DoesNotOverride = <DoesNotOverride>::default();
    p = (&mut n as *mut DoesNotOverride);
    (unsafe { (*p).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (3))
    );
    (unsafe { callThroughReference_2(&mut n) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (3))
    );
    let mut copiedFromDerived: DeclaresVirtual = d;
    p = (&mut copiedFromDerived as *mut DeclaresVirtual);
    (unsafe { (*p).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (3))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (3))
    );
    (unsafe { callThroughReference_2(&mut copiedFromDerived) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (4))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (3))
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
