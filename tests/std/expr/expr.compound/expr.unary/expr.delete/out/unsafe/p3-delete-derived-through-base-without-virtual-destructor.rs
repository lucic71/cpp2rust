extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut base_dtors_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_dtors_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct HasNonVirtualDestructor {}
impl HasNonVirtualDestructor {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut base_dtors_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct DerivesWithOwnDestructor {}
impl DerivesWithOwnDestructor {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_dtors_1)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut HasNonVirtualDestructor =
        (Box::leak(Box::new(<DerivesWithOwnDestructor>::default()))
            as *mut DerivesWithOwnDestructor);
    {
        let __p = p;
        if !__p.is_null() {
            HasNonVirtualDestructor::destructor(&mut *__p);
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_dtors_0))
        + ((2) * (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_dtors_1))));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_dtors_0);
    std::cell::LazyCell::force(&*&raw const derived_dtors_1);
}
