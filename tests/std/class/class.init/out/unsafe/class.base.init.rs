extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Logged {
    pub id: i32,
}
impl Logged {
    pub unsafe fn new(this: *mut Logged, mut id: i32) {
        (&raw mut (*this).id).write(id);
        (unsafe { libc::printf(c"member %d\n".as_ptr() as *const libc::c_char, (id)) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct p6_delegating_constructor_runs_target_first_Delegating {
    pub value: i32,
}
impl p6_delegating_constructor_runs_target_first_Delegating {
    pub unsafe fn new_1(
        this: *mut p6_delegating_constructor_runs_target_first_Delegating,
        mut value: i32,
    ) {
        (&raw mut (*this).value).write(value);
        (unsafe { libc::printf(c"target %d\n".as_ptr() as *const libc::c_char, (value)) });
    }
    pub unsafe fn new_2(this: *mut p6_delegating_constructor_runs_target_first_Delegating) {
        p6_delegating_constructor_runs_target_first_Delegating::new_1(this.clone(), { 42 });
        (unsafe {
            libc::printf(
                c"delegating %d\n".as_ptr() as *const libc::c_char,
                ((*this).value),
            )
        });
        (*this).value += 1;
    }
}
impl Default for p6_delegating_constructor_runs_target_first_Delegating {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p6_delegating_constructor_runs_target_first_Delegating =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p6_delegating_constructor_runs_target_first_Delegating::new_2(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn test_0() {
    let mut object: p6_delegating_constructor_runs_target_first_Delegating =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p6_delegating_constructor_runs_target_first_Delegating::new_2(&raw mut object);
    assert!(((object.value) == (43)));
}
pub static mut evaluated_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn next_2() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut evaluated_1)).prefix_inc());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct p9_1_default_member_initializer_WithDefaults {
    pub fixed: i32,
    pub counted: i32,
}
impl p9_1_default_member_initializer_WithDefaults {
    pub unsafe fn new_2(this: *mut p9_1_default_member_initializer_WithDefaults) {
        (&raw mut (*this).fixed).write(5);
        (&raw mut (*this).counted).write((unsafe { next_2() }));
    }
    pub unsafe fn new_1(this: *mut p9_1_default_member_initializer_WithDefaults, mut counted: i32) {
        (&raw mut (*this).fixed).write(5);
        (&raw mut (*this).counted).write(counted);
    }
}
impl Default for p9_1_default_member_initializer_WithDefaults {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p9_1_default_member_initializer_WithDefaults =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p9_1_default_member_initializer_WithDefaults::new_2(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn test_3() {
    let mut defaults: p9_1_default_member_initializer_WithDefaults =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_1_default_member_initializer_WithDefaults::new_2(&raw mut defaults);
    assert!(
        (((defaults.fixed) == (5)) && ((defaults.counted) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut evaluated_1)) == (1))
    );
    let mut given: p9_1_default_member_initializer_WithDefaults =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_1_default_member_initializer_WithDefaults::new_1(&raw mut given, { 9 });
    assert!(
        (((given.fixed) == (5)) && ((given.counted) == (9)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut evaluated_1)) == (1))
    );
}
pub static mut constructed_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_3_member_is_default_initialized_Member {
    pub id: i32,
}
impl p9_3_member_is_default_initialized_Member {
    pub unsafe fn new(this: *mut p9_3_member_is_default_initialized_Member) {
        (&raw mut (*this).id)
            .write((*(*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_4)).prefix_inc()));
    }
}
impl Default for p9_3_member_is_default_initialized_Member {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p9_3_member_is_default_initialized_Member =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p9_3_member_is_default_initialized_Member::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_3_member_is_default_initialized_Holder {
    pub first: p9_3_member_is_default_initialized_Member,
    pub second: p9_3_member_is_default_initialized_Member,
    pub number: i32,
}
impl p9_3_member_is_default_initialized_Holder {
    pub unsafe fn new(this: *mut p9_3_member_is_default_initialized_Holder) {
        p9_3_member_is_default_initialized_Member::new((&raw mut (*this).first));
        p9_3_member_is_default_initialized_Member::new((&raw mut (*this).second));
        (&raw mut (*this).number).write(3);
    }
}
impl Default for p9_3_member_is_default_initialized_Holder {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p9_3_member_is_default_initialized_Holder =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p9_3_member_is_default_initialized_Holder::new(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn test_5() {
    let mut holder: p9_3_member_is_default_initialized_Holder =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_3_member_is_default_initialized_Holder::new(&raw mut holder);
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_4)) == (2))
            && ((holder.first.id) == (1)))
            && ((holder.second.id) == (2))
    );
    assert!(((holder.number) == (3)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p13_members_initialized_in_declaration_order_Ordered {
    pub first: Logged,
    pub second: Logged,
    pub sum: i32,
}
impl p13_members_initialized_in_declaration_order_Ordered {
    pub unsafe fn new(this: *mut p13_members_initialized_in_declaration_order_Ordered) {
        Logged::new((&raw mut (*this).first), { 1 });
        Logged::new((&raw mut (*this).second), { 2 });
        (&raw mut (*this).sum).write((((*this).first.id) + ((*this).second.id)));
        (unsafe { libc::printf(c"body %d\n".as_ptr() as *const libc::c_char, ((*this).sum)) });
    }
}
impl Default for p13_members_initialized_in_declaration_order_Ordered {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p13_members_initialized_in_declaration_order_Ordered =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p13_members_initialized_in_declaration_order_Ordered::new(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn test_6() {
    let mut ordered: p13_members_initialized_in_declaration_order_Ordered =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p13_members_initialized_in_declaration_order_Ordered::new(&raw mut ordered);
    assert!(((ordered.sum) == (3)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_3() });
    (unsafe { test_5() });
    (unsafe { test_6() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const evaluated_1);
    std::cell::LazyCell::force(&*&raw const constructed_4);
}
