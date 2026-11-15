extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut base_hook_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut first_hook_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_hook_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresHook {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CallsHookInMemberInitializers {
    pub fromMemInitializer: i32,
    pub fromDefaultMemberInitializer: i32,
}
impl CallsHookInMemberInitializers {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            fromMemInitializer: (unsafe { this.hook() }),
            fromDefaultMemberInitializer: (unsafe { this.hook() }),
        };
        this
    }
}
impl Default for CallsHookInMemberInitializers {
    fn default() -> Self {
        unsafe { CallsHookInMemberInitializers::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesHookAgain {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: OverridesHookAgain = <OverridesHookAgain>::default();
    assert!((((d as CallsHookInMemberInitializers).fromMemInitializer) == (1)));
    assert!((((d as CallsHookInMemberInitializers).fromDefaultMemberInitializer) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_hook_calls_0)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_hook_calls_1)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut second_hook_calls_2)) == (0)));
    let mut p: *mut DeclaresHook = (&mut d as *mut OverridesHookAgain);
    assert!(((unsafe { (*p).hook() }) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut second_hook_calls_2)) == (1)));
    return 0;
}
unsafe impl DeclaresHook for CallsHookInMemberInitializers {
    unsafe fn hook(&mut self) -> i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut first_hook_calls_1)).prefix_inc();
    }
}
unsafe impl CallsHookInMemberInitializers for OverridesHookAgain {
    unsafe fn hook(&mut self) -> i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut second_hook_calls_2)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_hook_calls_0);
    std::cell::LazyCell::force(&*&raw const first_hook_calls_1);
    std::cell::LazyCell::force(&*&raw const second_hook_calls_2);
}
