extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut first_destructions_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_destructions_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe trait DeclaresVirtual {
    unsafe fn id(&mut self) -> i32;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct FirstOverrider {}
impl FirstOverrider {
    unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_destructions_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct SecondOverrider {}
impl SecondOverrider {
    unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_destructions_1)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut storage: [u8; 64] = [0_u8; 64];
    let mut object: *mut dyn DeclaresVirtual =
        (Box::leak(Box::new(<FirstOverrider>::default())) as *mut FirstOverrider);
    assert!(((unsafe { (*object).id() }) == (1)));
    (unsafe { (*object)() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_destructions_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_destructions_1)) == (0))
    );
    object = (Box::leak(Box::new(<SecondOverrider>::default())) as *mut SecondOverrider);
    assert!(((unsafe { (*object).id() }) == (2)));
    (unsafe { (*object)() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_destructions_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_destructions_1)) == (1))
    );
    return 0;
}
unsafe impl DeclaresVirtual for FirstOverrider {
    unsafe fn id(&mut self) -> i32 {
        return 1;
    }
}
unsafe impl DeclaresVirtual for SecondOverrider {
    unsafe fn id(&mut self) -> i32 {
        return 2;
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const first_destructions_0);
    std::cell::LazyCell::force(&*&raw const second_destructions_1);
}
