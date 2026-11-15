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
pub static mut first_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresTouch {}
impl DeclaresTouch {
    pub unsafe fn touch(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HidesTouch {}
impl HidesTouch {
    pub unsafe fn touch(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HidesTouchAgain {}
impl HidesTouchAgain {
    pub unsafe fn touch(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)).prefix_inc();
    }
    pub unsafe fn touchAll(&mut self) {
        (unsafe { HidesTouchAgain::touch(self) });
        (unsafe { HidesTouch::touch(self) });
        (unsafe { DeclaresTouch::touch(self) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: HidesTouchAgain = <HidesTouchAgain>::default();
    (unsafe { HidesTouchAgain::touch(&mut d) });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (1))
    );
    (unsafe { HidesTouch::touch(&mut (d as HidesTouch)) });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (1))
    );
    (unsafe { DeclaresTouch::touch(&mut (d as DeclaresTouch)) });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (1))
    );
    (unsafe { HidesTouchAgain::touchAll(&mut d) });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (2)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (2))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const first_calls_1);
    std::cell::LazyCell::force(&*&raw const second_calls_2);
}
