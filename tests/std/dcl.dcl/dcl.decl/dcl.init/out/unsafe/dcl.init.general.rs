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
pub struct p6_1_zero_initialized_pointer_to_member_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    pub selector: isize,
}
impl Default for p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    fn default() -> Self {
        unsafe { p6_1_zero_initialized_pointer_to_member_HoldsSelector { selector: -1_isize } }
    }
}
pub static mut static_pm_0: std::cell::LazyCell<isize> =
    std::cell::LazyCell::new(|| unsafe { -1_isize });
pub unsafe fn test_1() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut static_pm_0)) == -1_isize));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut static_pm_0))
            != (::std::mem::offset_of!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
    let mut value_initialized: isize = -1_isize;
    assert!(((value_initialized) == -1_isize));
    assert!(
        ((value_initialized)
            != (::std::mem::offset_of!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
    let mut holder: p6_1_zero_initialized_pointer_to_member_HoldsSelector =
        <p6_1_zero_initialized_pointer_to_member_HoldsSelector>::default();
    assert!(((holder.selector) == -1_isize));
    assert!(
        ((holder.selector)
            != (::std::mem::offset_of!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
}
pub static mut constructed_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p7_default_initialization_calls_default_constructor_Counted {
    pub id: i32,
}
impl p7_default_initialization_calls_default_constructor_Counted {
    pub unsafe fn new(this: *mut p7_default_initialization_calls_default_constructor_Counted) {
        (&raw mut (*this).id)
            .write((*(*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_2)).prefix_inc()));
    }
}
impl Default for p7_default_initialization_calls_default_constructor_Counted {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p7_default_initialization_calls_default_constructor_Counted =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p7_default_initialization_calls_default_constructor_Counted::new(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn test_3() {
    let mut one: p7_default_initialization_calls_default_constructor_Counted =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p7_default_initialization_calls_default_constructor_Counted::new(&raw mut one);
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_2)) == (1))
            && ((one.id) == (1))
    );
    let mut many: [p7_default_initialization_calls_default_constructor_Counted; 3] =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d =
            (&raw mut many).cast::<p7_default_initialization_calls_default_constructor_Counted>();
        for __i in 0..3_isize {
            p7_default_initialization_calls_default_constructor_Counted::new(__d.offset(__i));
        }
    };
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_2)) == (4)));
    assert!(
        (((many[(0) as usize].id) == (2)) && ((many[(1) as usize].id) == (3)))
            && ((many[(2) as usize].id) == (4))
    );
}
pub static mut constructed_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_1_1_value_initialization_calls_user_provided_constructor_Seven {
    pub value: i32,
}
impl p9_1_1_value_initialization_calls_user_provided_constructor_Seven {
    pub unsafe fn new(
        this: *mut p9_1_1_value_initialization_calls_user_provided_constructor_Seven,
    ) {
        (&raw mut (*this).value).write(7);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_4)).prefix_inc();
    }
}
impl Default for p9_1_1_value_initialization_calls_user_provided_constructor_Seven {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p9_1_1_value_initialization_calls_user_provided_constructor_Seven =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p9_1_1_value_initialization_calls_user_provided_constructor_Seven::new(
                    &raw mut __v,
                );
                __v
            }
        }
    }
}
pub unsafe fn test_5() {
    let mut braces: p9_1_1_value_initialization_calls_user_provided_constructor_Seven =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_1_1_value_initialization_calls_user_provided_constructor_Seven::new(&raw mut braces);
    assert!(
        ((braces.value) == (7))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_4)) == (1))
    );
    assert!(
        (({
            let mut __v: p9_1_1_value_initialization_calls_user_provided_constructor_Seven =
                ::std::mem::MaybeUninit::uninit().assume_init();
            p9_1_1_value_initialization_calls_user_provided_constructor_Seven::new(&raw mut __v);
            __v
        }
        .value)
            == (7))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_4)) == (2))
    );
    let mut heap: *mut p9_1_1_value_initialization_calls_user_provided_constructor_Seven = {
        let __p = Box::into_raw(Box::<
            p9_1_1_value_initialization_calls_user_provided_constructor_Seven,
        >::new_uninit())
            as *mut p9_1_1_value_initialization_calls_user_provided_constructor_Seven;
        p9_1_1_value_initialization_calls_user_provided_constructor_Seven::new(__p);
        __p
    };
    assert!(
        (((*heap).value) == (7))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_4)) == (3))
    );
    {
        let __p = heap;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub static mut constructed_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p9_1_2_value_initialization_zeroes_then_default_initializes_Pair {
    pub first: i32,
    pub second: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_1_2_value_initialization_zeroes_then_default_initializes_Member {
    pub id: i32,
}
impl p9_1_2_value_initialization_zeroes_then_default_initializes_Member {
    pub unsafe fn new(
        this: *mut p9_1_2_value_initialization_zeroes_then_default_initializes_Member,
    ) {
        (&raw mut (*this).id)
            .write((*(*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)).prefix_inc()));
    }
}
impl Default for p9_1_2_value_initialization_zeroes_then_default_initializes_Member {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p9_1_2_value_initialization_zeroes_then_default_initializes_Member =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p9_1_2_value_initialization_zeroes_then_default_initializes_Member::new(
                    &raw mut __v,
                );
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p9_1_2_value_initialization_zeroes_then_default_initializes_Holder {
    pub number: i32,
    pub member: p9_1_2_value_initialization_zeroes_then_default_initializes_Member,
    pub pointer: *mut i32,
}
impl p9_1_2_value_initialization_zeroes_then_default_initializes_Holder {
    pub unsafe fn new_1(
        this: *mut p9_1_2_value_initialization_zeroes_then_default_initializes_Holder,
    ) {
        p9_1_2_value_initialization_zeroes_then_default_initializes_Member::new(
            (&raw mut (*this).member),
        );
    }
}
pub unsafe fn test_7() {
    let mut braces: p9_1_2_value_initialization_zeroes_then_default_initializes_Pair =
        <p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::default();
    assert!(((braces.first) == (0)) && ((braces.second) == (0)));
    assert!(
        ((<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::default().first)
            == (0))
            && ((<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::default()
                .second)
                == (0))
    );
    let mut dirty: *mut p9_1_2_value_initialization_zeroes_then_default_initializes_Pair =
        (Box::leak(Box::new(
            <p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::default(),
        )) as *mut p9_1_2_value_initialization_zeroes_then_default_initializes_Pair);
    (*dirty).first = 23130;
    (*dirty).second = 23130;
    {
        let __p = dirty;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut heap: *mut p9_1_2_value_initialization_zeroes_then_default_initializes_Pair = (Box::leak(
        Box::new(<p9_1_2_value_initialization_zeroes_then_default_initializes_Pair>::default()),
    )
        as *mut p9_1_2_value_initialization_zeroes_then_default_initializes_Pair);
    assert!((((*heap).first) == (0)) && (((*heap).second) == (0)));
    {
        let __p = heap;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut holder: p9_1_2_value_initialization_zeroes_then_default_initializes_Holder =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut holder;
        (&raw mut (*__d).number).write(0_i32);
        p9_1_2_value_initialization_zeroes_then_default_initializes_Member::new(
            (&raw mut (*__d).member),
        );
        (&raw mut (*__d).pointer).write(std::ptr::null_mut());
    };
    assert!(((holder.number) == (0)) && ((holder.pointer).is_null()));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)) == (1))
            && ((holder.member.id) == (1))
    );
    let mut heap_holder: *mut p9_1_2_value_initialization_zeroes_then_default_initializes_Holder = {
        let __p = Box::into_raw(Box::<
            p9_1_2_value_initialization_zeroes_then_default_initializes_Holder,
        >::new_uninit())
            as *mut p9_1_2_value_initialization_zeroes_then_default_initializes_Holder;
        {
            let __d = __p;
            (__d).write_bytes(0, 1);
            p9_1_2_value_initialization_zeroes_then_default_initializes_Holder::new_1(__d)
        };
        __p
    };
    assert!((((*heap_holder).number) == (0)) && (((*heap_holder).pointer).is_null()));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)) == (2))
            && (((*heap_holder).member.id) == (2))
    );
    {
        let __p = heap_holder;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p9_2_value_initialized_array_and_scalar_Pair {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_8() {
    let mut pairs: [p9_2_value_initialized_array_and_scalar_Pair; 2] =
        [<p9_2_value_initialized_array_and_scalar_Pair>::default(); 2];
    assert!(((pairs[(0) as usize].first) == (0)) && ((pairs[(1) as usize].second) == (0)));
    let mut numbers: [i32; 3] = [0_i32; 3];
    assert!(((numbers[(0) as usize]) == (0)) && ((numbers[(2) as usize]) == (0)));
    let mut number: i32 = 0_i32;
    let mut pointer: *mut i32 = std::ptr::null_mut();
    assert!(((number) == (0)) && ((pointer).is_null()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_3() });
    (unsafe { test_5() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const static_pm_0);
    std::cell::LazyCell::force(&*&raw const constructed_2);
    std::cell::LazyCell::force(&*&raw const constructed_4);
    std::cell::LazyCell::force(&*&raw const constructed_6);
}
