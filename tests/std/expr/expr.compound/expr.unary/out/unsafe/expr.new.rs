extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut constructed_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p23_1_new_without_initializer_Counted {
    pub id: i32,
}
impl p23_1_new_without_initializer_Counted {
    pub unsafe fn new(this: *mut p23_1_new_without_initializer_Counted) {
        (&raw mut (*this).id)
            .write((*(*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_0)).prefix_inc()));
    }
}
impl Default for p23_1_new_without_initializer_Counted {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p23_1_new_without_initializer_Counted =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p23_1_new_without_initializer_Counted::new(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn test_1() {
    let mut one: *mut p23_1_new_without_initializer_Counted = {
        let __p = Box::into_raw(Box::<p23_1_new_without_initializer_Counted>::new_uninit())
            as *mut p23_1_new_without_initializer_Counted;
        p23_1_new_without_initializer_Counted::new(__p);
        __p
    };
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_0)) == (1))
            && (((*one).id) == (1))
    );
    {
        let __p = one;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut many: *mut p23_1_new_without_initializer_Counted = Box::leak(
        (0..3_usize)
            .map(|_| <p23_1_new_without_initializer_Counted>::default())
            .collect::<Box<[p23_1_new_without_initializer_Counted]>>(),
    )
    .as_mut_ptr();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_0)) == (4)));
    assert!(
        ((((*many.offset((0) as isize)).id) == (2)) && (((*many.offset((1) as isize)).id) == (3)))
            && (((*many.offset((2) as isize)).id) == (4))
    );
    {
        let __p = many;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<p23_1_new_without_initializer_Counted>(),
            )))
        }
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p23_2_new_with_initializer_Pair {
    pub first: i32,
    pub second: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p23_2_new_with_initializer_Tracker {
    pub self_: *mut p23_2_new_with_initializer_Tracker,
    pub value: i32,
}
impl p23_2_new_with_initializer_Tracker {
    pub unsafe fn new(this: *mut p23_2_new_with_initializer_Tracker, mut value: i32) {
        (&raw mut (*this).self_).write(this);
        (&raw mut (*this).value).write(value);
    }
}
pub unsafe fn test_2() {
    let mut dirty: *mut p23_2_new_with_initializer_Pair =
        (Box::leak(Box::new(<p23_2_new_with_initializer_Pair>::default()))
            as *mut p23_2_new_with_initializer_Pair);
    (*dirty).first = 23130;
    (*dirty).second = 23130;
    {
        let __p = dirty;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut zeroed: *mut p23_2_new_with_initializer_Pair =
        (Box::leak(Box::new(<p23_2_new_with_initializer_Pair>::default()))
            as *mut p23_2_new_with_initializer_Pair);
    assert!((((*zeroed).first) == (0)) && (((*zeroed).second) == (0)));
    {
        let __p = zeroed;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut tracker: *mut p23_2_new_with_initializer_Tracker = {
        let __p = Box::into_raw(Box::<p23_2_new_with_initializer_Tracker>::new_uninit())
            as *mut p23_2_new_with_initializer_Tracker;
        p23_2_new_with_initializer_Tracker::new(__p, { 4 });
        __p
    };
    assert!((((*tracker).self_) == (tracker)) && (((*tracker).value) == (4)));
    {
        let __p = tracker;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut braces: *mut p23_2_new_with_initializer_Pair =
        (Box::leak(Box::new(p23_2_new_with_initializer_Pair {
            first: 1,
            second: 2,
        })) as *mut p23_2_new_with_initializer_Pair);
    assert!((((*braces).first) == (1)) && (((*braces).second) == (2)));
    {
        let __p = braces;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut number: *mut i32 = (Box::leak(Box::new(7)) as *mut i32);
    assert!(((*number) == (7)));
    {
        let __p = number;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut zero: *mut i32 = (Box::leak(Box::new(0_i32)) as *mut i32);
    assert!(((*zero) == (0)));
    {
        let __p = zero;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const constructed_0);
}
