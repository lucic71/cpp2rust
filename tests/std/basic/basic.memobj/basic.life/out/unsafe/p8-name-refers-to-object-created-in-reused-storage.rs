extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut constructions_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut destructions_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresVirtual {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HoldsValue {
    pub held: i32,
}
impl HoldsValue {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { held: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructions_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destructions_1)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HoldsValue = HoldsValue::new({ 1 });
    let _dtor_object =
        std::mem::DropGuard::new(&raw mut object, |__p| unsafe { (*__p).destructor() });
    let mut pointer: *mut HoldsValue = (&mut object as *mut HoldsValue);
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut constructions_0)) == (1)));
    assert!(((unsafe { (*pointer).value() }) == (1)));
    (unsafe { HoldsValue::destructor(&mut object) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut destructions_1)) == (1)));
    (Box::leak(Box::new(HoldsValue::new({ 2 }))) as *mut HoldsValue);
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut constructions_0)) == (2)));
    assert!(((object.held) == (2)));
    assert!((((*pointer).held) == (2)));
    assert!(((unsafe { (*pointer).value() }) == (2)));
    let mut base: *mut DeclaresVirtual = pointer;
    assert!(((unsafe { (*base).value() }) == (2)));
    return 0;
}
unsafe impl DeclaresVirtual for HoldsValue {
    unsafe fn value(&mut self) -> i32 {
        return self.held;
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const constructions_0);
    std::cell::LazyCell::force(&*&raw const destructions_1);
}
