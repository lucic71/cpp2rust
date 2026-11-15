extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut sequence_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut indirect_base_stamp_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut first_base_stamp_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_base_stamp_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut first_member_stamp_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_member_stamp_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut body_stamp_6: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(*mut i32 , new, value)]
pub struct StampsDestruction {
    pub stamp: *mut i32,
}
impl StampsDestruction {
    pub unsafe fn new(mut s: *mut i32) -> Self {
        let mut this = Self { stamp: s };
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.stamp) = (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct IndirectBase {}
impl IndirectBase {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut indirect_base_stamp_1)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct FirstBase {}
impl FirstBase {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_base_stamp_2)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct SecondBase {}
impl SecondBase {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_base_stamp_3)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct HasBasesAndMembers {
    pub firstMember: StampsDestruction,
    pub secondMember: StampsDestruction,
}
impl HasBasesAndMembers {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            firstMember: StampsDestruction::new({
                (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut first_member_stamp_4))
                    as *mut i32)
            }),
            secondMember: StampsDestruction::new({
                (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut second_member_stamp_5))
                    as *mut i32)
            }),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut body_stamp_6)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
        StampsDestruction::destructor(&mut self.secondMember);
        StampsDestruction::destructor(&mut self.firstMember);
    }
}
impl Default for HasBasesAndMembers {
    fn default() -> Self {
        unsafe { HasBasesAndMembers::new() }
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
        let mut d: HasBasesAndMembers = HasBasesAndMembers::new();
        let _dtor_d = std::mem::DropGuard::new(&raw mut d, |__p| unsafe { (*__p).destructor() });
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)) == (0)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut body_stamp_6)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut second_member_stamp_5)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_member_stamp_4)) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut second_base_stamp_3)) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut first_base_stamp_2)) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut indirect_base_stamp_1)) == (6)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sequence_0);
    std::cell::LazyCell::force(&*&raw const indirect_base_stamp_1);
    std::cell::LazyCell::force(&*&raw const first_base_stamp_2);
    std::cell::LazyCell::force(&*&raw const second_base_stamp_3);
    std::cell::LazyCell::force(&*&raw const first_member_stamp_4);
    std::cell::LazyCell::force(&*&raw const second_member_stamp_5);
    std::cell::LazyCell::force(&*&raw const body_stamp_6);
}
