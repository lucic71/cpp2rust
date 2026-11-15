extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut abstract_dtors_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut concrete_dtors_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
impl HasPureVirtualDestructor {
    unsafe fn destructor(&mut self);
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct OverridesDestructor {}
impl OverridesDestructor {
    unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut concrete_dtors_1)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut d: OverridesDestructor = <OverridesDestructor>::default();
        let _dtor_d = std::mem::DropGuard::new(&raw mut d, |__p| unsafe { (*__p).destructor() });
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut abstract_dtors_0)) == (0))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut concrete_dtors_1)) == (0))
        );
    }
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut abstract_dtors_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut concrete_dtors_1)) == (1))
    );
    let mut p: *mut dyn HasPureVirtualDestructor =
        (Box::leak(Box::new(<OverridesDestructor>::default())) as *mut OverridesDestructor);
    {
        let __p = p;
        if !__p.is_null() {
            HasPureVirtualDestructor::destructor(&mut *__p);
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut abstract_dtors_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut concrete_dtors_1)) == (2))
    );
    return 0;
}
unsafe impl HasPureVirtualDestructor for OverridesDestructor {}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const abstract_dtors_0);
    std::cell::LazyCell::force(&*&raw const concrete_dtors_1);
}
