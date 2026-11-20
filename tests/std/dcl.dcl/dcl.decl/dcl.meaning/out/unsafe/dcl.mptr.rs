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
pub struct p2_pointer_to_array_member_HasIntArray {
    pub before: i32,
    pub elements: [i32; 3],
}
impl p2_pointer_to_array_member_HasIntArray {
    pub unsafe fn new_1(this: *mut p2_pointer_to_array_member_HasIntArray) {}
    pub unsafe fn copy_from(
        this: *mut p2_pointer_to_array_member_HasIntArray,
        _a0: *const p2_pointer_to_array_member_HasIntArray,
    ) {
        (&raw mut (*this).before).write((*_a0).before);
        (&raw mut (*this).elements).write(std::array::from_fn::<_, 3, _>(|__i: usize| {
            (*_a0).elements[(__i)]
        }));
    }
    pub unsafe fn move_from(
        this: *mut p2_pointer_to_array_member_HasIntArray,
        _a0: *mut p2_pointer_to_array_member_HasIntArray,
    ) {
        (&raw mut (*this).before).write((*_a0).before);
        (&raw mut (*this).elements).write(std::array::from_fn::<_, 3, _>(|__i: usize| {
            (*_a0).elements[(__i)]
        }));
    }
}
pub unsafe fn test_0() {
    let mut pm: isize =
        (::std::mem::offset_of!(p2_pointer_to_array_member_HasIntArray, elements) as isize);
    let mut obj: p2_pointer_to_array_member_HasIntArray =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut obj;
        (&raw mut (*__d).before).write(1);
        (&raw mut (*__d).elements).write([2, 3, 4]);
    };
    assert!(
        (((*(&mut obj as *mut p2_pointer_to_array_member_HasIntArray)
            .wrapping_byte_offset(pm)
            .cast::<[i32; 3]>())[(0) as usize])
            == (2))
    );
    assert!(
        (((*(&mut obj as *mut p2_pointer_to_array_member_HasIntArray)
            .wrapping_byte_offset(pm)
            .cast::<[i32; 3]>())[(2) as usize])
            == (4))
    );
    (*(&mut obj as *mut p2_pointer_to_array_member_HasIntArray)
        .wrapping_byte_offset(pm)
        .cast::<[i32; 3]>())[(1) as usize] = 7;
    assert!(((obj.elements[(1) as usize]) == (7)));
    assert!(((obj.before) == (1)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_pointer_to_data_member_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
impl p2_pointer_to_data_member_HasTwoInts {
    pub unsafe fn new_1(this: *mut p2_pointer_to_data_member_HasTwoInts) {}
    pub unsafe fn copy_from(
        this: *mut p2_pointer_to_data_member_HasTwoInts,
        _a0: *const p2_pointer_to_data_member_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
    pub unsafe fn move_from(
        this: *mut p2_pointer_to_data_member_HasTwoInts,
        _a0: *mut p2_pointer_to_data_member_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
pub unsafe fn test_1() {
    let mut pm: isize =
        (::std::mem::offset_of!(p2_pointer_to_data_member_HasTwoInts, first) as isize);
    let mut obj: p2_pointer_to_data_member_HasTwoInts =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut obj;
        (&raw mut (*__d).first).write(3);
        (&raw mut (*__d).second).write(4);
    };
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_data_member_HasTwoInts)
            .wrapping_byte_offset(pm)
            .cast::<i32>())
            == (3))
    );
    (*(&mut obj as *mut p2_pointer_to_data_member_HasTwoInts)
        .wrapping_byte_offset(pm)
        .cast::<i32>()) = 5;
    assert!(((obj.first) == (5)));
    assert!(((obj.second) == (4)));
    pm = (::std::mem::offset_of!(p2_pointer_to_data_member_HasTwoInts, second) as isize);
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_data_member_HasTwoInts)
            .wrapping_byte_offset(pm)
            .cast::<i32>())
            == (4))
    );
    (*(&mut obj as *mut p2_pointer_to_data_member_HasTwoInts)
        .wrapping_byte_offset(pm)
        .cast::<i32>()) = 6;
    assert!(((obj.first) == (5)));
    assert!(((obj.second) == (6)));
}
pub static mut pm_2: libcc2rs::LazyStorage<isize> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(-1_isize) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_pointer_to_member_of_incomplete_class_CompletedLater {
    pub first: i32,
    pub second: i32,
}
impl p2_pointer_to_member_of_incomplete_class_CompletedLater {
    pub unsafe fn new_1(this: *mut p2_pointer_to_member_of_incomplete_class_CompletedLater) {}
    pub unsafe fn copy_from(
        this: *mut p2_pointer_to_member_of_incomplete_class_CompletedLater,
        _a0: *const p2_pointer_to_member_of_incomplete_class_CompletedLater,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
    pub unsafe fn move_from(
        this: *mut p2_pointer_to_member_of_incomplete_class_CompletedLater,
        _a0: *mut p2_pointer_to_member_of_incomplete_class_CompletedLater,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
pub unsafe fn test_3() {
    assert!(((*(*&raw const pm_2).get()) == -1_isize));
    (*(*&raw const pm_2).get()) = (::std::mem::offset_of!(
        p2_pointer_to_member_of_incomplete_class_CompletedLater,
        second
    ) as isize);
    let mut obj: p2_pointer_to_member_of_incomplete_class_CompletedLater =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut obj;
        (&raw mut (*__d).first).write(3);
        (&raw mut (*__d).second).write(4);
    };
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_member_of_incomplete_class_CompletedLater)
            .wrapping_byte_offset((*(*&raw const pm_2).get()))
            .cast::<i32>())
            == (4))
    );
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
impl p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts {
    pub unsafe fn new_1(this: *mut p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts) {}
    pub unsafe fn copy_from(
        this: *mut p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
        _a0: *const p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
    pub unsafe fn move_from(
        this: *mut p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
        _a0: *mut p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector {
    pub selector: isize,
}
impl p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector {
    pub unsafe fn new_1(this: *mut p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector) {}
    pub unsafe fn copy_from(
        this: *mut p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector,
        _a0: *const p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector,
    ) {
        (&raw mut (*this).selector).write((*_a0).selector);
    }
    pub unsafe fn move_from(
        this: *mut p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector,
        _a0: *mut p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector,
    ) {
        (&raw mut (*this).selector).write((*_a0).selector);
    }
}
impl Default for p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector {
    fn default() -> Self {
        unsafe {
            p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector { selector: -1_isize }
        }
    }
}
pub unsafe fn test_4() {
    let mut pm: isize = (::std::mem::offset_of!(
        p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector,
        selector
    ) as isize);
    let mut holder: p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut holder;
        (&raw mut (*__d).selector).write(
            (::std::mem::offset_of!(
                p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
                second
            ) as isize),
        );
    };
    let mut obj: p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut obj;
        (&raw mut (*__d).first).write(3);
        (&raw mut (*__d).second).write(4);
    };
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts)
            .wrapping_byte_offset(
                (*(&mut holder
                    as *mut p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector)
                    .wrapping_byte_offset(pm)
                    .cast::<isize>())
            )
            .cast::<i32>())
            == (4))
    );
    (*(&mut holder as *mut p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector)
        .wrapping_byte_offset(pm)
        .cast::<isize>()) = (::std::mem::offset_of!(
        p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
        first
    ) as isize);
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts)
            .wrapping_byte_offset((holder.selector))
            .cast::<i32>())
            == (3))
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const pm_2).get();
}
