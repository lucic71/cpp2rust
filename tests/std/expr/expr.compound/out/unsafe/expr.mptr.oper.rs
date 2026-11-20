extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_arrow_star_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
impl p3_arrow_star_HasTwoInts {
    pub unsafe fn copy_from(
        this: *mut p3_arrow_star_HasTwoInts,
        _a0: *const p3_arrow_star_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
pub unsafe fn test_0() {
    let mut pm: isize = (::std::mem::offset_of!(p3_arrow_star_HasTwoInts, second) as isize);
    let mut obj: p3_arrow_star_HasTwoInts = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut obj;
        (&raw mut (*__d).first).write(3);
        (&raw mut (*__d).second).write(4);
    };
    let mut p: *mut p3_arrow_star_HasTwoInts = (&mut obj as *mut p3_arrow_star_HasTwoInts);
    assert!(((*(p).wrapping_byte_offset(pm).cast::<i32>()) == (4)));
    (*(p).wrapping_byte_offset(pm).cast::<i32>()) = 5;
    assert!(((obj.second) == (5)));
    assert!(((obj.first) == (3)));
    assert!(
        ((&mut (*(p).wrapping_byte_offset(pm).cast::<i32>()) as *mut i32)
            == (&mut (*(&mut (*p) as *mut p3_arrow_star_HasTwoInts)
                .wrapping_byte_offset(pm)
                .cast::<i32>()) as *mut i32))
    );
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_object_expression_sequenced_first_HasValue {
    pub value: i32,
}
impl p4_object_expression_sequenced_first_HasValue {
    pub unsafe fn copy_from(
        this: *mut p4_object_expression_sequenced_first_HasValue,
        _a0: *const p4_object_expression_sequenced_first_HasValue,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub static mut sequence_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut object_stamp_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut member_stamp_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut object_4: libcc2rs::LazyStorage<p4_object_expression_sequenced_first_HasValue> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __d = __p;
            (&raw mut (*__d).value).write(7);
        }
    });
pub unsafe fn select_object_5() -> *mut p4_object_expression_sequenced_first_HasValue {
    (*(*&raw const object_stamp_2).get()) = (*(*(*&raw const sequence_1).get()).prefix_inc());
    return &mut (*(*&raw const object_4).get());
}
pub unsafe fn select_member_6() -> isize {
    (*(*&raw const member_stamp_3).get()) = (*(*(*&raw const sequence_1).get()).prefix_inc());
    return (::std::mem::offset_of!(p4_object_expression_sequenced_first_HasValue, value) as isize);
}
pub unsafe fn test_7() {
    assert!(
        ((*(unsafe { select_object_5() })
            .wrapping_byte_offset((unsafe { select_member_6() }))
            .cast::<i32>())
            == (7))
    );
    assert!(((*(*&raw const object_stamp_2).get()) == (1)));
    assert!(((*(*&raw const member_stamp_3).get()) == (2)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p6_data_member_of_lvalue_is_lvalue_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
impl p6_data_member_of_lvalue_is_lvalue_HasTwoInts {
    pub unsafe fn copy_from(
        this: *mut p6_data_member_of_lvalue_is_lvalue_HasTwoInts,
        _a0: *const p6_data_member_of_lvalue_is_lvalue_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
pub unsafe fn test_8() {
    let mut pm: isize =
        (::std::mem::offset_of!(p6_data_member_of_lvalue_is_lvalue_HasTwoInts, second) as isize);
    let mut obj: p6_data_member_of_lvalue_is_lvalue_HasTwoInts =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut obj;
        (&raw mut (*__d).first).write(3);
        (&raw mut (*__d).second).write(4);
    };
    assert!(
        ((&mut (*(&mut obj as *mut p6_data_member_of_lvalue_is_lvalue_HasTwoInts)
            .wrapping_byte_offset(pm)
            .cast::<i32>()) as *mut i32)
            == (&mut obj.second as *mut i32))
    );
    let ref_: *mut i32 = &mut (*(&mut obj as *mut p6_data_member_of_lvalue_is_lvalue_HasTwoInts)
        .wrapping_byte_offset(pm)
        .cast::<i32>());
    (*ref_) = 5;
    assert!(((obj.second) == (5)));
    assert!(((obj.first) == (3)));
}
pub static mut copyCount_9: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut moveCount_10: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {}
impl p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {
    pub unsafe fn new(this: *mut p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves) {}
    pub unsafe fn copy_from(
        this: *mut p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
        _a0: *const p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
    ) {
        (*(*&raw const copyCount_9).get()).prefix_inc();
    }
    pub unsafe fn move_from(
        this: *mut p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
        _a0: *mut p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
    ) {
        (*(*&raw const moveCount_10).get()).prefix_inc();
    }
}
impl Clone for p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves =
                ::std::mem::MaybeUninit::uninit().assume_init();
            p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::copy_from(
                &raw mut __v,
                self as *const p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
            );
            __v
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct p6_data_member_of_rvalue_is_xvalue_HoldsCounted {
    pub member: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
}
impl p6_data_member_of_rvalue_is_xvalue_HoldsCounted {
    pub unsafe fn copy_from(
        this: *mut p6_data_member_of_rvalue_is_xvalue_HoldsCounted,
        _a0: *const p6_data_member_of_rvalue_is_xvalue_HoldsCounted,
    ) {
        p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::copy_from(
            (&raw mut (*this).member),
            { &(*_a0).member },
        );
    }
    pub unsafe fn move_from(
        this: *mut p6_data_member_of_rvalue_is_xvalue_HoldsCounted,
        _a0: *mut p6_data_member_of_rvalue_is_xvalue_HoldsCounted,
    ) {
        p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::move_from(
            (&raw mut (*this).member),
            { &mut (*_a0).member },
        );
    }
    pub unsafe fn new_1(this: *mut p6_data_member_of_rvalue_is_xvalue_HoldsCounted) {
        p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::new((&raw mut (*this).member));
    }
}
pub unsafe fn test_11() {
    let mut pm: isize =
        (::std::mem::offset_of!(p6_data_member_of_rvalue_is_xvalue_HoldsCounted, member) as isize);
    let mut holder: p6_data_member_of_rvalue_is_xvalue_HoldsCounted =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p6_data_member_of_rvalue_is_xvalue_HoldsCounted::new_1(&raw mut holder);
    let mut from_lvalue: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::copy_from(&raw mut from_lvalue, {
        &(*(&mut holder as *mut p6_data_member_of_rvalue_is_xvalue_HoldsCounted)
            .wrapping_byte_offset(pm)
            .cast::<p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves>())
    });
    assert!(((*(*&raw const copyCount_9).get()) == (1)));
    assert!(((*(*&raw const moveCount_10).get()) == (0)));
    let mut from_xvalue: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::move_from(&raw mut from_xvalue, {
        &mut (*(&mut holder as *mut p6_data_member_of_rvalue_is_xvalue_HoldsCounted)
            .wrapping_byte_offset(pm)
            .cast::<p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves>())
    });
    assert!(((*(*&raw const copyCount_9).get()) == (1)));
    assert!(((*(*&raw const moveCount_10).get()) == (1)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    (unsafe { test_11() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const sequence_1).get();
    (*&raw const object_stamp_2).get();
    (*&raw const member_stamp_3).get();
    (*&raw const object_4).get();
    (*&raw const copyCount_9).get();
    (*&raw const moveCount_10).get();
}
