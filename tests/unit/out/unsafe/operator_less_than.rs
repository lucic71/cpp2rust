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
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
impl Pair {
    pub unsafe fn operator_lt(&mut self, other: *const Pair) -> bool {
        return ((self.x) < ((*other).x))
            || (((self.x) == ((*other).x)) && ((self.y) < ((*other).y)));
    }
    pub unsafe fn new_1(this: *mut Pair) {}
    pub unsafe fn copy_from(this: *mut Pair, _a0: *const Pair) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
    pub unsafe fn move_from(this: *mut Pair, _a0: *mut Pair) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
impl std::cmp::Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Pair::operator_lt(&mut *(&raw const *self).cast_mut(), other as *const Pair) {
                std::cmp::Ordering::Less
            } else if Pair::operator_lt(&mut *(&raw const *other).cast_mut(), self as *const Pair) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Pair::operator_lt(&mut *(&raw const *self).cast_mut(), other as *const Pair))
                && !(Pair::operator_lt(&mut *(&raw const *other).cast_mut(), self as *const Pair))
        }
    }
}
impl std::cmp::Eq for Pair {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pair1: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut pair1;
        (&raw mut (*__d).x).write(1);
        (&raw mut (*__d).y).write(2);
    };
    let mut pair2: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut pair2;
        (&raw mut (*__d).x).write(1);
        (&raw mut (*__d).y).write(3);
    };
    assert!((unsafe { Pair::operator_lt(&mut pair1, &pair2,) }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
