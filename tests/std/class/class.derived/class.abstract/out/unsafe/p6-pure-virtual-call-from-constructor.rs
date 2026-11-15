extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut derived_hook_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe trait CallsPureVirtualInConstructor {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (unsafe { CallsPureVirtualInConstructor::callHook(&mut this) });
        this
    }
    unsafe fn callHook(&mut self) {
        (unsafe { self.hook() });
    }
    unsafe fn hook(&mut self);
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
    let mut d: OverridesHook = <OverridesHook>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_hook_calls_0)) == (0)));
    return 0;
}
unsafe impl CallsPureVirtualInConstructor for OverridesHook {
    unsafe fn hook(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_hook_calls_0)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const derived_hook_calls_0);
}
