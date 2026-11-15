extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut sequence_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn nextStamp_1() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HasIntConstructor {
    pub value: i32,
    pub stamp: i32,
}
impl HasIntConstructor {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            value: v,
            stamp: (unsafe { nextStamp_1() }),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasDefaultConstructor {
    pub stamp: i32,
}
impl HasDefaultConstructor {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            stamp: (unsafe { nextStamp_1() }),
        };
        this
    }
}
impl Default for HasDefaultConstructor {
    fn default() -> Self {
        unsafe { HasDefaultConstructor::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct InheritsConstructor {
    pub memberStamp: i32,
}
impl Default for InheritsConstructor {
    fn default() -> Self {
        unsafe {
            InheritsConstructor {
                memberStamp: (unsafe { nextStamp_1() }),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct InheritsConstructorWithOtherBase {
    pub memberStamp: i32,
}
impl Default for InheritsConstructorWithOtherBase {
    fn default() -> Self {
        unsafe {
            InheritsConstructorWithOtherBase {
                memberStamp: (unsafe { nextStamp_1() }),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: InheritsConstructor = InheritsConstructor::new_1({ 2 });
    assert!((((d as HasIntConstructor).value) == (2)));
    assert!((((d as HasIntConstructor).stamp) == (1)));
    assert!(((d.memberStamp) == (2)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)) = 0;
    let mut e: InheritsConstructorWithOtherBase = InheritsConstructorWithOtherBase::new_1({ 3 });
    assert!((((e as HasIntConstructor).value) == (3)));
    assert!((((e as HasDefaultConstructor).stamp) == (1)));
    assert!((((e as HasIntConstructor).stamp) == (2)));
    assert!(((e.memberStamp) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sequence_0);
}
