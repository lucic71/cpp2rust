extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut shared_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(3) });
pub static mut calls_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_2_static_member_is_ordinary_pointer_HasStaticMembers {}
impl p3_2_static_member_is_ordinary_pointer_HasStaticMembers {
    pub unsafe fn run() {
        (*(*&raw const calls_1).get()).prefix_inc();
    }
    pub unsafe fn new_1(this: *mut p3_2_static_member_is_ordinary_pointer_HasStaticMembers) {}
    pub unsafe fn copy_from(
        this: *mut p3_2_static_member_is_ordinary_pointer_HasStaticMembers,
        _a0: *const p3_2_static_member_is_ordinary_pointer_HasStaticMembers,
    ) {
    }
    pub unsafe fn move_from(
        this: *mut p3_2_static_member_is_ordinary_pointer_HasStaticMembers,
        _a0: *mut p3_2_static_member_is_ordinary_pointer_HasStaticMembers,
    ) {
    }
}
pub unsafe fn test_2() {
    let mut object_pointer: *mut i32 = (&raw mut (*(*&raw const shared_0).get()) as *mut i32);
    assert!(((*object_pointer) == (3)));
    (*object_pointer) = 4;
    assert!(((*(*&raw const shared_0).get()) == (4)));
    let mut function_pointer: Option<unsafe fn()> =
        (Some(p3_2_static_member_is_ordinary_pointer_HasStaticMembers::run));
    (unsafe { (function_pointer).unwrap()() });
    assert!(((*(*&raw const calls_1).get()) == (1)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember {
    pub base: i32,
}
impl p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember {
    pub unsafe fn add(
        self_: *const p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember,
        mut amount: i32,
    ) -> i32 {
        return (((*self_).base) + (amount));
    }
    pub unsafe fn new_1(
        this: *mut p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember,
    ) {
    }
    pub unsafe fn copy_from(
        this: *mut p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember,
        _a0: *const p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember,
    ) {
        (&raw mut (*this).base).write((*_a0).base);
    }
    pub unsafe fn move_from(
        this: *mut p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember,
        _a0: *mut p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember,
    ) {
        (&raw mut (*this).base).write((*_a0).base);
    }
}
pub unsafe fn test_3() {
    let mut function_pointer: Option<
        unsafe fn(
            *const p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember,
            i32,
        ) -> i32,
    > = (Some(p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember::add));
    let mut object: p3_2_explicit_object_member_is_ordinary_pointer_HasExplicitObjectMember =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut object;
        (&raw mut (*__d).base).write(3);
    };
    assert!(((unsafe { (function_pointer).unwrap()(&object, 4,) }) == (7)));
    object.base = 5;
    assert!(((unsafe { (function_pointer).unwrap()(&object, 1,) }) == (6)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember {
    pub value: i32,
}
impl p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember {
    pub unsafe fn parenthesized(&mut self) -> *mut i32 {
        return (&mut (self.value) as *mut i32);
    }
    pub unsafe fn unqualified(&mut self) -> *mut i32 {
        return (&mut self.value as *mut i32);
    }
    pub unsafe fn new_1(
        this: *mut p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember,
    ) {
    }
    pub unsafe fn copy_from(
        this: *mut p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember,
        _a0: *const p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(
        this: *mut p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember,
        _a0: *mut p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub unsafe fn test_4() {
    let mut first: p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut first;
        (&raw mut (*__d).value).write(3);
    };
    let mut second: p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut second;
        (&raw mut (*__d).value).write(4);
    };
    let mut p: *mut i32 = (unsafe {
        p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember::parenthesized(
            &mut first,
        )
    });
    assert!(((p) == (&mut first.value as *mut i32)));
    (*p) = 5;
    assert!(((first.value) == (5)));
    assert!(((second.value) == (4)));
    let mut q: *mut i32 = (unsafe {
        p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember::unqualified(
            &mut second,
        )
    });
    assert!(((q) == (&mut second.value as *mut i32)));
    (*q) = 6;
    assert!(((first.value) == (5)));
    assert!(((second.value) == (6)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const shared_0).get();
    (*&raw const calls_1).get();
}
