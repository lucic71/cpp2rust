extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait Animal {
    unsafe fn bark(&self) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Dog {}
impl Dog {
    pub unsafe fn new_1(this: *mut Dog) {}
    pub unsafe fn copy_from(this: *mut Dog, _a0: *const Dog) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cat {}
impl Cat {
    unsafe fn meow(&self) -> bool {
        return true;
    }
    pub unsafe fn new_1(this: *mut Cat) {}
    pub unsafe fn copy_from(this: *mut Cat, _a0: *const Cat) {}
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut dog: Dog = ::std::mem::MaybeUninit::uninit().assume_init();
    Dog::new_1(&raw mut dog);
    let mut animal: *mut dyn Animal = (&mut dog as *mut Dog);
    let mut eat1: bool = (unsafe { (*(animal).cast_const()).bark() });
    let mut cat: Cat = ::std::mem::MaybeUninit::uninit().assume_init();
    Cat::new_1(&raw mut cat);
    animal = (&mut cat as *mut Cat);
    let mut eat2: bool = (unsafe { (*(animal).cast_const()).bark() });
    assert!((eat1) && (!(eat2)));
    return 0;
}
unsafe impl Animal for Cat {
    unsafe fn bark(&self) -> bool {
        return false;
    }
}
unsafe impl Animal for Dog {
    unsafe fn bark(&self) -> bool {
        return true;
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
