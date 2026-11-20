extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut balance_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(50) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p1_member_hides_namespace_variable_HasBalance {
    pub balance: i32,
}
impl p1_member_hides_namespace_variable_HasBalance {
    pub unsafe fn deposit(&mut self, mut amount: i32) -> i32 {
        self.balance += amount;
        return self.balance;
    }
    pub unsafe fn deposit_globally(&mut self, mut amount: i32) -> i32 {
        (*(*&raw const balance_0).get()) += amount;
        return (*(*&raw const balance_0).get());
    }
    pub unsafe fn new_1(this: *mut p1_member_hides_namespace_variable_HasBalance) {
        (&raw mut (*this).balance).write(7);
    }
    pub unsafe fn copy_from(
        this: *mut p1_member_hides_namespace_variable_HasBalance,
        _a0: *const p1_member_hides_namespace_variable_HasBalance,
    ) {
        (&raw mut (*this).balance).write((*_a0).balance);
    }
    pub unsafe fn move_from(
        this: *mut p1_member_hides_namespace_variable_HasBalance,
        _a0: *mut p1_member_hides_namespace_variable_HasBalance,
    ) {
        (&raw mut (*this).balance).write((*_a0).balance);
    }
}
impl Default for p1_member_hides_namespace_variable_HasBalance {
    fn default() -> Self {
        unsafe { p1_member_hides_namespace_variable_HasBalance { balance: 7 } }
    }
}
pub unsafe fn test_1() {
    let mut object: p1_member_hides_namespace_variable_HasBalance =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p1_member_hides_namespace_variable_HasBalance::new_1(&raw mut object);
    assert!(
        ((unsafe { p1_member_hides_namespace_variable_HasBalance::deposit(&mut object, 3,) })
            == (10))
    );
    assert!(((object.balance) == (10)));
    assert!(((*(*&raw const balance_0).get()) == (50)));
    assert!(
        ((unsafe {
            p1_member_hides_namespace_variable_HasBalance::deposit_globally(&mut object, 4)
        }) == (54))
    );
    assert!(((object.balance) == (10)));
    assert!(((*(*&raw const balance_0).get()) == (54)));
}
pub static mut initial_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(100) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p1_out_of_class_definition_is_in_class_scope_HasStaticTotal {
    pub held: i32,
}
impl p1_out_of_class_definition_is_in_class_scope_HasStaticTotal {
    pub unsafe fn new_1(this: *mut p1_out_of_class_definition_is_in_class_scope_HasStaticTotal) {
        (&raw mut (*this).held).write(1);
    }
    pub unsafe fn copy_from(
        this: *mut p1_out_of_class_definition_is_in_class_scope_HasStaticTotal,
        _a0: *const p1_out_of_class_definition_is_in_class_scope_HasStaticTotal,
    ) {
        (&raw mut (*this).held).write((*_a0).held);
    }
    pub unsafe fn move_from(
        this: *mut p1_out_of_class_definition_is_in_class_scope_HasStaticTotal,
        _a0: *mut p1_out_of_class_definition_is_in_class_scope_HasStaticTotal,
    ) {
        (&raw mut (*this).held).write((*_a0).held);
    }
}
impl Default for p1_out_of_class_definition_is_in_class_scope_HasStaticTotal {
    fn default() -> Self {
        unsafe { p1_out_of_class_definition_is_in_class_scope_HasStaticTotal { held: 1 } }
    }
}
pub static mut initial_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(6) });
pub static mut total_4: libcc2rs::LazyStorage<i32> = libcc2rs::LazyStorage::new(|__p| unsafe {
    __p.write(((*(*&raw const initial_3).get()) + (1)))
});
impl p1_out_of_class_definition_is_in_class_scope_HasStaticTotal {
    pub unsafe fn add(&mut self, mut amount: i32) -> i32 {
        self.held += amount;
        (*(*&raw const total_4).get()) += (*(*&raw const initial_3).get());
        return self.held;
    }
}
pub unsafe fn test_5() {
    assert!(((*(*&raw const total_4).get()) == (7)));
    let mut object: p1_out_of_class_definition_is_in_class_scope_HasStaticTotal =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p1_out_of_class_definition_is_in_class_scope_HasStaticTotal::new_1(&raw mut object);
    assert!(
        ((unsafe {
            p1_out_of_class_definition_is_in_class_scope_HasStaticTotal::add(&mut object, 4)
        }) == (5))
    );
    assert!(((*(*&raw const total_4).get()) == (13)));
    assert!(((*(*&raw const initial_3).get()) == (6)));
    assert!(((*(*&raw const initial_2).get()) == (100)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const balance_0).get();
    (*&raw const initial_2).get();
    (*&raw const initial_3).get();
    (*&raw const total_4).get();
}
