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
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct CallsHook {}
impl CallsHook {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (unsafe { this.hook() });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { CallsHook::callHookIndirectly(self) });
    }
    pub unsafe fn callHookIndirectly(&mut self) {
        (unsafe { self.hook() });
    }
}
impl Default for CallsHook {
    fn default() -> Self {
        unsafe { CallsHook::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct OverridesHookAndCallsIt {}
impl OverridesHookAndCallsIt {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (unsafe { CallsHook::callHookIndirectly(&mut this) });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { self.hook() });
    }
}
impl Default for OverridesHookAndCallsIt {
    fn default() -> Self {
        unsafe { OverridesHookAndCallsIt::new() }
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
    {
        let mut d: OverridesHookAgain = <OverridesHookAgain>::default();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_hook_calls_0)) == (1)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_hook_calls_1)) == (1)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut second_hook_calls_2)) == (0)));
        (unsafe { CallsHook::callHookIndirectly(&mut (d as CallsHook)) });
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut second_hook_calls_2)) == (1)));
        let mut p: *mut CallsHook = (&mut d as *mut OverridesHookAgain);
        (unsafe { (*p).hook() });
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut second_hook_calls_2)) == (2)));
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_hook_calls_0)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_hook_calls_1)) == (1))
        );
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_hook_calls_0)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_hook_calls_1)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut second_hook_calls_2)) == (2)));
    return 0;
}
unsafe impl OverridesHookAndCallsIt for OverridesHookAgain {
    unsafe fn hook(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_hook_calls_2)).prefix_inc();
    }
}
unsafe impl CallsHook for OverridesHookAndCallsIt {
    unsafe fn hook(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_hook_calls_1)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_hook_calls_0);
    std::cell::LazyCell::force(&*&raw const first_hook_calls_1);
    std::cell::LazyCell::force(&*&raw const second_hook_calls_2);
}
