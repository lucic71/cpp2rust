extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut default_ctors_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut int_ctors_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut shared_int_ctors_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct HasTwoConstructors {
    pub value: i32,
}
impl HasTwoConstructors {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { value: -1_i32 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctors_0)).prefix_inc();
        this
    }
    pub unsafe fn new_1(mut v: i32) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_1)).prefix_inc();
        this
    }
}
impl Default for HasTwoConstructors {
    fn default() -> Self {
        unsafe { HasTwoConstructors::new_2() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InheritsConstructors {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DoesNotInherit {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct JoinsInheritingAndPlain {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HasIntConstructor {
    pub value: i32,
}
impl HasIntConstructor {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut shared_int_ctors_2)).prefix_inc();
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SharedBase {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstVirtualPath {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondVirtualPath {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct JoinsVirtualPaths {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut joined: JoinsInheritingAndPlain = JoinsInheritingAndPlain::new_1({ 0 });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut int_ctors_1)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut default_ctors_0)) == (1))
    );
    assert!(((((joined as InheritsConstructors) as HasTwoConstructors).value) == (0)));
    assert!(((((joined as DoesNotInherit) as HasTwoConstructors).value) == (-1_i32)));
    let mut shared: JoinsVirtualPaths = JoinsVirtualPaths::new_1({ 5 });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut shared_int_ctors_2)) == (1)));
    assert!((((shared as HasIntConstructor).value) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const default_ctors_0);
    std::cell::LazyCell::force(&*&raw const int_ctors_1);
    std::cell::LazyCell::force(&*&raw const shared_int_ctors_2);
}
