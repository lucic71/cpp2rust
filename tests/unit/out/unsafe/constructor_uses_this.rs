extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn append_0(mut out: *mut Vec<i32>, mut v: i32) {
    {
        let a0_clone = v.clone();
        (*out).push(a0_clone)
    };
}
#[repr(C)]
#[derive(Clone)]
pub struct Setup {
    pub size: i32,
    pub values: Vec<i32>,
}
impl Setup {
    pub unsafe fn Setup() -> Self {
        let mut this = Self {
            size: 0,
            values: Vec::new(),
        };
        (unsafe { this.init() });
        (unsafe { append_0((&raw mut this.values as *mut Vec<i32>), 7) });
        this
    }
    pub unsafe fn init(&mut self) {
        self.size = 3;
    }
}
impl Default for Setup {
    fn default() -> Self {
        unsafe { Setup::Setup() }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: Setup = Setup::Setup();
    assert!(((s.size) == (3)));
    assert!(((s.values.len()) == (1_usize)));
    assert!(((s.values[(0_usize)]) == (7)));
    return 0;
}
