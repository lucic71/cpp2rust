extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Clone)]
pub struct Counter {
    pub bits: i32,
}
impl Counter {
    pub unsafe fn units(&self) -> i32 {
        return ((self.bits) / (8));
    }
}
impl Drop for Counter {
    fn drop(&mut self) {
        unsafe {
            total_0 += (unsafe { self.units() });
        }
    }
}
impl Default for Counter {
    fn default() -> Self {
        Counter { bits: 16 }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct Watcher {
    pub target: *mut i32,
}
impl Drop for Watcher {
    fn drop(&mut self) {
        unsafe {
            if !((self.target).is_null()) {
                total_0 += (*self.target);
                self.target = std::ptr::null_mut();
            }
        }
    }
}
impl Default for Watcher {
    fn default() -> Self {
        Watcher {
            target: std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Owner {
    pub watcher: Watcher,
}
#[repr(C)]
#[derive(Clone)]
pub struct Tracker {
    pub step: i32,
}
impl Drop for Tracker {
    fn drop(&mut self) {
        unsafe {
            total_0 += self.step;
        }
    }
}
impl Default for Tracker {
    fn default() -> Self {
        Tracker { step: 4 }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut c: Counter = <Counter>::default();
    }
    assert!(((total_0) == (2)));
    let mut value: i32 = 40;
    {
        let mut o: Owner = <Owner>::default();
        o.watcher.target = (&raw mut value as *mut i32);
    }
    assert!(((total_0) == (42)));
    {
        let mut t: Tracker = <Tracker>::default();
    }
    assert!(((total_0) == (46)));
    return 0;
}
