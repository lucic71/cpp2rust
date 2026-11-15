extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut base_sibling_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut base_here_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut sibling_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut here_calls_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut most_derived_sibling_calls_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut most_derived_here_calls_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct VirtualBase {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SiblingOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*mut VirtualBase , new, value)]
pub struct CallsVirtualsInConstructor {}
impl CallsVirtualsInConstructor {
    pub unsafe fn new(mut base: *mut VirtualBase) -> Self {
        let mut this = Self {};
        (unsafe { this.overriddenBySibling() });
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_sibling_calls_0)) == (1)));
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut sibling_calls_2)) == (0))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_sibling_calls_4))
                    == (0))
        );
        (unsafe { this.overriddenHere() });
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut here_calls_3)) == (1)));
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_here_calls_1)) == (0))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_here_calls_5))
                    == (0))
        );
        (unsafe { (*base).overriddenHere() });
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut here_calls_3)) == (2)));
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_here_calls_1)) == (0))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_here_calls_5))
                    == (0))
        );
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct MostDerived {}
impl MostDerived {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for MostDerived {
    fn default() -> Self {
        unsafe { MostDerived::new() }
    }
}
impl CallsVirtualsInConstructor {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: MostDerived = MostDerived::new();
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_sibling_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut here_calls_3)) == (2))
    );
    let mut base: *mut VirtualBase = (&mut d as *mut MostDerived);
    (unsafe { (*base).overriddenBySibling() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_sibling_calls_4)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_here_calls_5)) == (0))
    );
    (unsafe { (*base).overriddenHere() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_sibling_calls_4)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_here_calls_5)) == (1))
    );
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_sibling_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_here_calls_1)) == (0))
    );
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut sibling_calls_2)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut here_calls_3)) == (2))
    );
    return 0;
}
unsafe impl VirtualBase for CallsVirtualsInConstructor {
    unsafe fn overriddenHere(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut here_calls_3)).prefix_inc();
    }
}
unsafe impl SiblingOverrider for MostDerived {
    unsafe fn overriddenBySibling(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_sibling_calls_4)).prefix_inc();
    }
    unsafe fn overriddenHere(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_here_calls_5)).prefix_inc();
    }
}
unsafe impl VirtualBase for SiblingOverrider {
    unsafe fn overriddenBySibling(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut sibling_calls_2)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_sibling_calls_0);
    std::cell::LazyCell::force(&*&raw const base_here_calls_1);
    std::cell::LazyCell::force(&*&raw const sibling_calls_2);
    std::cell::LazyCell::force(&*&raw const here_calls_3);
    std::cell::LazyCell::force(&*&raw const most_derived_sibling_calls_4);
    std::cell::LazyCell::force(&*&raw const most_derived_here_calls_5);
}
