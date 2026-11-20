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
pub static mut global_pair_0: std::cell::LazyCell<p2_static_storage_is_zero_initialized_Pair> =
    std::cell::LazyCell::new(|| unsafe { <p2_static_storage_is_zero_initialized_Pair>::default() });
pub static mut global_numbers_1: std::cell::LazyCell<[i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [0_i32; 3] });
pub static mut global_pointer_2: std::cell::LazyCell<*mut i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut global_partial_3: std::cell::LazyCell<
    p2_static_storage_is_zero_initialized_Partial,
> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __v: p2_static_storage_is_zero_initialized_Partial =
            ::std::mem::MaybeUninit::zeroed().assume_init();
        p2_static_storage_is_zero_initialized_Partial::new(&raw mut __v);
        __v
    }
});
pub unsafe fn read_static_local_4() -> i32 {
    return ((((*std::cell::LazyCell::force_mut(&mut *&raw mut local_pair_5)).first)
        + ((*std::cell::LazyCell::force_mut(&mut *&raw mut local_pair_5)).second))
        + ((*std::cell::LazyCell::force_mut(&mut *&raw mut local_partial_6)).unset));
}
pub unsafe fn test_7() {
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_pair_0)).first) == (0))
            && (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_pair_0)).second) == (0))
    );
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_numbers_1))[(0) as usize]) == (0))
            && (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_numbers_1))[(2) as usize])
                == (0))
    );
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut global_pointer_2)).is_null());
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_partial_3)).set) == (4))
            && (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_partial_3)).unset) == (0))
    );
    assert!(((unsafe { read_static_local_4() }) == (0)));
}
static mut local_pair_5: std::cell::LazyCell<p2_static_storage_is_zero_initialized_Pair> =
    std::cell::LazyCell::new(|| unsafe { <p2_static_storage_is_zero_initialized_Pair>::default() });
static mut local_partial_6: std::cell::LazyCell<p2_static_storage_is_zero_initialized_Partial> =
    std::cell::LazyCell::new(|| unsafe {
        {
            let mut __v: p2_static_storage_is_zero_initialized_Partial =
                ::std::mem::MaybeUninit::zeroed().assume_init();
            p2_static_storage_is_zero_initialized_Partial::new(&raw mut __v);
            __v
        }
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
    std::cell::LazyCell::force(&*&raw const global_pair_0);
    std::cell::LazyCell::force(&*&raw const global_numbers_1);
    std::cell::LazyCell::force(&*&raw const global_pointer_2);
    std::cell::LazyCell::force(&*&raw const global_partial_3);
}
