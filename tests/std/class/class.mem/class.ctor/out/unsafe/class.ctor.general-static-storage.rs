extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[placement]
#[cpp_from(i32, new_1, value)]
pub struct Tracker {
    pub self_: *mut Tracker,
    pub value: i32,
}
impl Tracker {
    pub unsafe fn new_2(this: *mut Tracker) {
        (&raw mut (*this).self_).write(this);
        (&raw mut (*this).value).write(-1_i32);
    }
    pub unsafe fn new_1(this: *mut Tracker, mut value: i32) {
        (&raw mut (*this).self_).write(this);
        (&raw mut (*this).value).write(value);
    }
    pub unsafe fn new_3(this: *mut Tracker, mut a: i32, mut b: i32) {
        Tracker::new_1(this.clone(), { ((a) + (b)) });
    }
    pub unsafe fn copy_from(this: *mut Tracker, other: *const Tracker) {
        (&raw mut (*this).self_).write(this);
        (&raw mut (*this).value).write((*other).value);
    }
    pub unsafe fn move_from(this: *mut Tracker, other: *mut Tracker) {
        (&raw mut (*this).self_).write(this);
        (&raw mut (*this).value).write((*other).value);
        (*other).value = -2_i32;
    }
    pub unsafe fn is_self(&self) -> bool {
        return (((self.self_).cast_const()) == (self as *const Tracker));
    }
}
impl Clone for Tracker {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracker::copy_from(&raw mut __v, self as *const Tracker);
            __v
        }
    }
}
impl Default for Tracker {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
                Tracker::new_2(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn get_0() -> *mut Tracker {
    return (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut once_1)) as *mut Tracker);
}
pub unsafe fn test_2() {
    let mut p: *mut Tracker = (unsafe { get_0() });
    assert!((((*p).self_) == (p)) && (((*p).value) == (2)));
    assert!(((unsafe { get_0() }) == (p)));
}
static mut once_1: std::cell::LazyCell<Tracker> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __v: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracker::new_1(&raw mut __v, { 2 });
        __v
    }
});
pub static mut global_3: std::cell::LazyCell<Tracker> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __v: Tracker = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracker::new_1(&raw mut __v, { 3 });
        __v
    }
});
pub unsafe fn test_4() {
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_3)).self_)
            == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_3))
                as *mut Tracker))
            && (((*std::cell::LazyCell::force_mut(&mut *&raw mut global_3)).value) == (3))
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_2() });
    (unsafe { test_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_3);
}
