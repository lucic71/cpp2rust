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
pub struct p7_4_pointer_to_member_and_null_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
impl p7_4_pointer_to_member_and_null_HasTwoInts {
    pub unsafe fn copy_from(
        this: *mut p7_4_pointer_to_member_and_null_HasTwoInts,
        _a0: *const p7_4_pointer_to_member_and_null_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
pub unsafe fn second_or_null_0(mut choose_second: bool) -> isize {
    return if choose_second {
        (::std::mem::offset_of!(p7_4_pointer_to_member_and_null_HasTwoInts, second) as isize)
    } else {
        -1_isize
    };
}
pub unsafe fn test_1() {
    let mut obj: p7_4_pointer_to_member_and_null_HasTwoInts =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut obj;
        (&raw mut (*__d).first).write(3);
        (&raw mut (*__d).second).write(4);
    };
    assert!(((unsafe { second_or_null_0(false,) }) == -1_isize));
    assert!(
        ((unsafe { second_or_null_0(true,) })
            == (::std::mem::offset_of!(p7_4_pointer_to_member_and_null_HasTwoInts, second)
                as isize))
    );
    assert!(
        ((*(&mut obj as *mut p7_4_pointer_to_member_and_null_HasTwoInts)
            .wrapping_byte_offset((unsafe { second_or_null_0(true,) }))
            .cast::<i32>())
            == (4))
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
