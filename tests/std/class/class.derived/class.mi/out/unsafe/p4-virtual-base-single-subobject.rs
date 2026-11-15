extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut shared_base_ctors_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SharedBase {
    pub value: i32,
}
impl SharedBase {
    pub unsafe fn new() -> Self {
        let mut this = Self { value: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut shared_base_ctors_0)).prefix_inc();
        this
    }
}
impl Default for SharedBase {
    fn default() -> Self {
        unsafe { SharedBase::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WritesSharedBase {}
impl WritesSharedBase {
    pub unsafe fn write(&mut self, mut x: i32) {
        self.value = x;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReadsSharedBase {}
impl ReadsSharedBase {
    pub unsafe fn read(&mut self) -> i32 {
        return self.value;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct JoinsWriterAndReader {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: JoinsWriterAndReader = <JoinsWriterAndReader>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut shared_base_ctors_0)) == (1)));
    (unsafe { WritesSharedBase::write(&mut (d as WritesSharedBase), 4) });
    assert!(((unsafe { ReadsSharedBase::read(&mut (d as ReadsSharedBase),) }) == (4)));
    assert!((((d as SharedBase).value) == (4)));
    let mut writer: *mut WritesSharedBase = (&mut d as *mut JoinsWriterAndReader);
    let mut reader: *mut ReadsSharedBase = (&mut d as *mut JoinsWriterAndReader);
    let mut throughWriter: *mut SharedBase = writer;
    let mut throughReader: *mut SharedBase = reader;
    let mut direct: *mut SharedBase = (&mut d as *mut JoinsWriterAndReader);
    assert!(((throughWriter) == (throughReader)));
    assert!(((throughWriter) == (direct)));
    (*throughReader).value = 5;
    assert!((((*(writer as *mut SharedBase)).value) == (5)));
    assert!(((((d as WritesSharedBase) as SharedBase).value) == (5)));
    assert!(((((d as ReadsSharedBase) as SharedBase).value) == (5)));
    let mut onlyWriter: WritesSharedBase = <WritesSharedBase>::default();
    let mut onlyReader: ReadsSharedBase = <ReadsSharedBase>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut shared_base_ctors_0)) == (3)));
    (unsafe { WritesSharedBase::write(&mut onlyWriter, 6) });
    assert!(((unsafe { ReadsSharedBase::read(&mut onlyReader,) }) == (0)));
    assert!((((d as SharedBase).value) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const shared_base_ctors_0);
}
