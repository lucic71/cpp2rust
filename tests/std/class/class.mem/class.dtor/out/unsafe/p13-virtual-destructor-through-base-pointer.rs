extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut sequence_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut base_stamp_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut first_stamp_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_stamp_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut base_dtors_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut first_dtors_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_dtors_6: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct HasVirtualDestructor {}
impl HasVirtualDestructor {
    unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut base_dtors_4)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut base_stamp_1)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct OverridesDestructor {}
impl OverridesDestructor {
    unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_dtors_5)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_stamp_2)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct OverridesDestructorAgain {}
impl OverridesDestructorAgain {
    unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_dtors_6)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_stamp_3)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut HasVirtualDestructor =
        (Box::leak(Box::new(<OverridesDestructorAgain>::default()))
            as *mut OverridesDestructorAgain);
    {
        let __p = p;
        if !__p.is_null() {
            HasVirtualDestructor::destructor(&mut *__p);
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut second_dtors_6)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_dtors_5)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_dtors_4)) == (1))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut second_stamp_3)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_stamp_2)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_stamp_1)) == (3)));
    p = (Box::leak(Box::new(<OverridesDestructor>::default())) as *mut OverridesDestructor);
    {
        let __p = p;
        if !__p.is_null() {
            HasVirtualDestructor::destructor(&mut *__p);
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut second_dtors_6)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_dtors_5)) == (2)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_dtors_4)) == (2))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_stamp_2)) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_stamp_1)) == (5)));
    return 0;
}
unsafe impl HasVirtualDestructor for OverridesDestructor {}
unsafe impl OverridesDestructor for OverridesDestructorAgain {}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sequence_0);
    std::cell::LazyCell::force(&*&raw const base_stamp_1);
    std::cell::LazyCell::force(&*&raw const first_stamp_2);
    std::cell::LazyCell::force(&*&raw const second_stamp_3);
    std::cell::LazyCell::force(&*&raw const base_dtors_4);
    std::cell::LazyCell::force(&*&raw const first_dtors_5);
    std::cell::LazyCell::force(&*&raw const second_dtors_6);
}
