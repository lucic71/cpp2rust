extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut successful_casts_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut failed_casts_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Base {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstDerived {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondDerived {}
pub unsafe fn castToFirstDerived_2(base: *mut Base) {
    {
        let derived: *mut FirstDerived = base;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut successful_casts_0)).prefix_inc();
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut first: FirstDerived = <FirstDerived>::default();
    (unsafe { castToFirstDerived_2(&mut first) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut successful_casts_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut failed_casts_1)) == (0))
    );
    let mut second: SecondDerived = <SecondDerived>::default();
    (unsafe { castToFirstDerived_2(&mut second) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut successful_casts_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut failed_casts_1)) == (1))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const successful_casts_0);
    std::cell::LazyCell::force(&*&raw const failed_casts_1);
}
