extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut constructed_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p23_1_new_without_initializer_Counted {
    pub id: i32,
}
impl p23_1_new_without_initializer_Counted {
    pub unsafe fn new(this: *mut p23_1_new_without_initializer_Counted) {
        (&raw mut (*this).id).write((*(*(*&raw const constructed_0).get()).prefix_inc()));
    }
    pub unsafe fn copy_from(
        this: *mut p23_1_new_without_initializer_Counted,
        _a0: *const p23_1_new_without_initializer_Counted,
    ) {
        (&raw mut (*this).id).write((*_a0).id);
    }
    pub unsafe fn move_from(
        this: *mut p23_1_new_without_initializer_Counted,
        _a0: *mut p23_1_new_without_initializer_Counted,
    ) {
        (&raw mut (*this).id).write((*_a0).id);
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
    assert!(((*(*&raw const constructed_0).get()) == (1)) && (((*one).id) == (1)));
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
    assert!(((*(*&raw const constructed_0).get()) == (4)));
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
impl p23_2_new_with_initializer_Pair {
    pub unsafe fn new_1(this: *mut p23_2_new_with_initializer_Pair) {}
    pub unsafe fn copy_from(
        this: *mut p23_2_new_with_initializer_Pair,
        _a0: *const p23_2_new_with_initializer_Pair,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
    pub unsafe fn move_from(
        this: *mut p23_2_new_with_initializer_Pair,
        _a0: *mut p23_2_new_with_initializer_Pair,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
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
    pub unsafe fn copy_from(
        this: *mut p23_2_new_with_initializer_Tracker,
        _a0: *const p23_2_new_with_initializer_Tracker,
    ) {
        (&raw mut (*this).self_).write((*_a0).self_);
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(
        this: *mut p23_2_new_with_initializer_Tracker,
        _a0: *mut p23_2_new_with_initializer_Tracker,
    ) {
        (&raw mut (*this).self_).write((*_a0).self_);
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub unsafe fn test_2() {
    let mut dirty: *mut p23_2_new_with_initializer_Pair = {
        let __p = Box::into_raw(Box::<p23_2_new_with_initializer_Pair>::new_uninit())
            as *mut p23_2_new_with_initializer_Pair;
        p23_2_new_with_initializer_Pair::new_1(__p);
        __p
    };
    (*dirty).first = 23130;
    (*dirty).second = 23130;
    {
        let __p = dirty;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut zeroed: *mut p23_2_new_with_initializer_Pair = {
        let __p = Box::into_raw(Box::<p23_2_new_with_initializer_Pair>::new_uninit())
            as *mut p23_2_new_with_initializer_Pair;
        {
            let __d = __p;
            __d.write_bytes(0, 1);
            p23_2_new_with_initializer_Pair::new_1(__d)
        };
        __p
    };
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
    let mut braces: *mut p23_2_new_with_initializer_Pair = {
        let __p = Box::into_raw(Box::<p23_2_new_with_initializer_Pair>::new_uninit())
            as *mut p23_2_new_with_initializer_Pair;
        {
            let __d = __p;
            (&raw mut (*__d).first).write(1);
            (&raw mut (*__d).second).write(2);
        };
        __p
    };
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
    (*&raw const constructed_0).get();
}
