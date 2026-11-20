extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut stock_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(100) });
pub unsafe fn minimum_1() -> i32 {
    return 50;
}
pub type anon_2 = u32;
pub const anon_2_reserve: anon_2 = 30;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p7_1_function_body_names_later_member_HasLaterMembers {
    pub stock: i32,
}
impl p7_1_function_body_names_later_member_HasLaterMembers {
    pub unsafe fn take(&mut self, mut amount: i32) -> i32 {
        self.stock -= amount;
        return (((self.stock)
            + (unsafe { p7_1_function_body_names_later_member_HasLaterMembers::minimum(self) }))
            + (anon_2_reserve as i32));
    }
    pub unsafe fn minimum(&self) -> i32 {
        return 2;
    }
    pub unsafe fn new_1(this: *mut p7_1_function_body_names_later_member_HasLaterMembers) {
        (&raw mut (*this).stock).write(9);
    }
    pub unsafe fn copy_from(
        this: *mut p7_1_function_body_names_later_member_HasLaterMembers,
        _a0: *const p7_1_function_body_names_later_member_HasLaterMembers,
    ) {
        (&raw mut (*this).stock).write((*_a0).stock);
    }
}
impl Default for p7_1_function_body_names_later_member_HasLaterMembers {
    fn default() -> Self {
        unsafe { p7_1_function_body_names_later_member_HasLaterMembers { stock: 9 } }
    }
}
pub unsafe fn test_3() {
    let mut object: p7_1_function_body_names_later_member_HasLaterMembers =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p7_1_function_body_names_later_member_HasLaterMembers::new_1(&raw mut object);
    assert!(
        ((unsafe { p7_1_function_body_names_later_member_HasLaterMembers::take(&mut object, 3,) })
            == (38))
    );
    assert!(((object.stock) == (6)));
    assert!(((*(*&raw const stock_0).get()) == (100)));
}
pub static mut step_4: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(100) });
pub static mut step_5: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(4) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p7_2_default_argument_names_later_member_HasLaterStaticStep {
    pub total: i32,
}
impl p7_2_default_argument_names_later_member_HasLaterStaticStep {
    pub unsafe fn advance(&mut self, mut amount: i32) -> i32 {
        self.total += amount;
        return self.total;
    }
    pub unsafe fn new_1(this: *mut p7_2_default_argument_names_later_member_HasLaterStaticStep) {
        (&raw mut (*this).total).write(1);
    }
    pub unsafe fn copy_from(
        this: *mut p7_2_default_argument_names_later_member_HasLaterStaticStep,
        _a0: *const p7_2_default_argument_names_later_member_HasLaterStaticStep,
    ) {
        (&raw mut (*this).total).write((*_a0).total);
    }
}
impl Default for p7_2_default_argument_names_later_member_HasLaterStaticStep {
    fn default() -> Self {
        unsafe { p7_2_default_argument_names_later_member_HasLaterStaticStep { total: 1 } }
    }
}
pub unsafe fn test_6() {
    let mut object: p7_2_default_argument_names_later_member_HasLaterStaticStep =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p7_2_default_argument_names_later_member_HasLaterStaticStep::new_1(&raw mut object);
    assert!(
        ((unsafe {
            p7_2_default_argument_names_later_member_HasLaterStaticStep::advance(
                &mut object,
                (*(*&raw const step_5).get()),
            )
        }) == (5))
    );
    assert!(
        ((unsafe {
            p7_2_default_argument_names_later_member_HasLaterStaticStep::advance(&mut object, 2)
        }) == (7))
    );
    assert!(((*(*&raw const step_4).get()) == (100)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_3() });
    (unsafe { test_6() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const stock_0).get();
    (*&raw const step_4).get();
    (*&raw const step_5).get();
}
