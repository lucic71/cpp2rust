extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut abstract_hook_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_hook_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut helper_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe trait CallsHookInConstructor {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (unsafe { CallsHookInConstructor::helper(&mut this) });
        (unsafe { this.hook() });
        this
    }
    unsafe fn helper(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut helper_calls_2)).prefix_inc();
    }
    unsafe fn hook(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut abstract_hook_calls_0)).prefix_inc();
    }
    unsafe fn pure(&mut self);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesHook {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut d: OverridesHook = <OverridesHook>::default();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut helper_calls_2)) == (1)));
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut abstract_hook_calls_0)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_hook_calls_1)) == (0))
        );
        let mut p: *mut dyn CallsHookInConstructor = (&mut d as *mut OverridesHook);
        (unsafe { (*p).hook() });
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut abstract_hook_calls_0)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_hook_calls_1)) == (1))
        );
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut helper_calls_2)) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut abstract_hook_calls_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_hook_calls_1)) == (1))
    );
    return 0;
}
unsafe impl CallsHookInConstructor for OverridesHook {
    unsafe fn hook(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_hook_calls_1)).prefix_inc();
    }
    unsafe fn pure(&mut self) {}
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const abstract_hook_calls_0);
    std::cell::LazyCell::force(&*&raw const derived_hook_calls_1);
    std::cell::LazyCell::force(&*&raw const helper_calls_2);
}
