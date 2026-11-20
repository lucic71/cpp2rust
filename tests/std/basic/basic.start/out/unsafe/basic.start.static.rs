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
pub struct p2_static_storage_is_zero_initialized_Pair {
    pub first: i32,
    pub second: i32,
}
impl p2_static_storage_is_zero_initialized_Pair {
    pub unsafe fn new_1(this: *mut p2_static_storage_is_zero_initialized_Pair) {}
    pub unsafe fn copy_from(
        this: *mut p2_static_storage_is_zero_initialized_Pair,
        _a0: *const p2_static_storage_is_zero_initialized_Pair,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
    pub unsafe fn move_from(
        this: *mut p2_static_storage_is_zero_initialized_Pair,
        _a0: *mut p2_static_storage_is_zero_initialized_Pair,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p2_static_storage_is_zero_initialized_Partial {
    pub set: i32,
    pub unset: i32,
}
impl p2_static_storage_is_zero_initialized_Partial {
    pub unsafe fn new(this: *mut p2_static_storage_is_zero_initialized_Partial) {
        (&raw mut (*this).set).write(4);
    }
    pub unsafe fn copy_from(
        this: *mut p2_static_storage_is_zero_initialized_Partial,
        _a0: *const p2_static_storage_is_zero_initialized_Partial,
    ) {
        (&raw mut (*this).set).write((*_a0).set);
        (&raw mut (*this).unset).write((*_a0).unset);
    }
    pub unsafe fn move_from(
        this: *mut p2_static_storage_is_zero_initialized_Partial,
        _a0: *mut p2_static_storage_is_zero_initialized_Partial,
    ) {
        (&raw mut (*this).set).write((*_a0).set);
        (&raw mut (*this).unset).write((*_a0).unset);
    }
}
impl Default for p2_static_storage_is_zero_initialized_Partial {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p2_static_storage_is_zero_initialized_Partial =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p2_static_storage_is_zero_initialized_Partial::new(&raw mut __v);
                __v
            }
        }
    }
}
pub static mut global_pair_0: libcc2rs::LazyStorage<p2_static_storage_is_zero_initialized_Pair> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        p2_static_storage_is_zero_initialized_Pair::new_1(__p)
    });
pub static mut global_numbers_1: libcc2rs::LazyStorage<[i32; 3]> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write([0_i32; 3]) });
pub static mut global_pointer_2: libcc2rs::LazyStorage<*mut i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub static mut global_partial_3: libcc2rs::LazyStorage<
    p2_static_storage_is_zero_initialized_Partial,
> = libcc2rs::LazyStorage::new(|__p| unsafe {
    p2_static_storage_is_zero_initialized_Partial::new(__p)
});
pub unsafe fn read_static_local_4() -> i32 {
    return ((((*(*&raw const local_pair_5).get()).first)
        + ((*(*&raw const local_pair_5).get()).second))
        + ((*(*&raw const local_partial_6).get()).unset));
}
pub unsafe fn test_7() {
    assert!(
        (((*(*&raw const global_pair_0).get()).first) == (0))
            && (((*(*&raw const global_pair_0).get()).second) == (0))
    );
    assert!(
        (((*(*&raw const global_numbers_1).get())[(0) as usize]) == (0))
            && (((*(*&raw const global_numbers_1).get())[(2) as usize]) == (0))
    );
    assert!((*(*&raw const global_pointer_2).get()).is_null());
    assert!(
        (((*(*&raw const global_partial_3).get()).set) == (4))
            && (((*(*&raw const global_partial_3).get()).unset) == (0))
    );
    assert!(((unsafe { read_static_local_4() }) == (0)));
}
static mut local_pair_5: libcc2rs::LazyStorage<p2_static_storage_is_zero_initialized_Pair> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        p2_static_storage_is_zero_initialized_Pair::new_1(__p)
    });
static mut local_partial_6: libcc2rs::LazyStorage<p2_static_storage_is_zero_initialized_Partial> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        p2_static_storage_is_zero_initialized_Partial::new(__p)
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_7() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const global_pair_0).get();
    (*&raw const global_numbers_1).get();
    (*&raw const global_pointer_2).get();
    (*&raw const global_partial_3).get();
}
