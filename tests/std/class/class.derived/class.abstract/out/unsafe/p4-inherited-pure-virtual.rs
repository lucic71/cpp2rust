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
pub unsafe trait HasTwoPureVirtuals {
    unsafe fn first(&mut self);
    unsafe fn second(&mut self);
}
pub unsafe trait OverridesFirstOnly {
    unsafe fn first(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesSecond {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: OverridesSecond = <OverridesSecond>::default();
    let mut base: *mut dyn HasTwoPureVirtuals = (&mut d as *mut OverridesSecond);
    (unsafe { (*base).first() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (0))
    );
    (unsafe { (*base).second() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (1))
    );
    let mut stillAbstract: *mut dyn OverridesFirstOnly = (&mut d as *mut OverridesSecond);
    (unsafe { (*stillAbstract).first() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (1))
    );
    (unsafe { (*(stillAbstract as *mut dyn HasTwoPureVirtuals)).second() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (2))
    );
    return 0;
}
unsafe impl OverridesFirstOnly for OverridesSecond {
    unsafe fn second(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const first_calls_0);
    std::cell::LazyCell::force(&*&raw const second_calls_1);
}
