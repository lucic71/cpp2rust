extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static shared_base_ctors_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct SharedBase {
    #[offset(0)]
    pub value: i32,
}
impl SharedBase {
    pub fn new() -> Self {
        let __this: SharedBase = Self { value: 0 };
        (*shared_base_ctors_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for SharedBase {
    fn default() -> Self {
        { SharedBase::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct WritesSharedBase {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ReadsSharedBase {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct JoinsWriterAndReader {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<JoinsWriterAndReader> = Rc::new(RefCell::new(<JoinsWriterAndReader>::default()));
    assert!((shared_base_ctors_0.with(|rc| *rc.borrow()) == 1));
    ({ WritesSharedBaseImpl::write(&(d.as_pointer() as WritesSharedBase), 4) });
    assert!((({ ReadsSharedBaseImpl::read(&(d.as_pointer() as ReadsSharedBase),) }) == 4));
    assert!(({ ({ ((*d.borrow()) as SharedBase).value } == 4) }));
    let mut writer: Ptr<WritesSharedBase> = (d.as_pointer());
    let mut reader: Ptr<ReadsSharedBase> = (d.as_pointer());
    let mut throughWriter: Ptr<SharedBase> = (writer).clone();
    let mut throughReader: Ptr<SharedBase> = (reader).clone();
    let mut direct: Ptr<SharedBase> = (d.as_pointer());
    assert!(({ (throughWriter).clone() } == { (throughReader).clone() }));
    assert!(({ (throughWriter).clone() } == { (direct).clone() }));
    field!(throughReader, value).write(5);
    assert!(((writer as *mut SharedBase).with(|__s| __s.value) == 5));
    assert!(({ ({ (((*d.borrow()) as WritesSharedBase) as SharedBase).value } == 5) }));
    assert!(({ ({ (((*d.borrow()) as ReadsSharedBase) as SharedBase).value } == 5) }));
    let onlyWriter: Value<WritesSharedBase> = Rc::new(RefCell::new(<WritesSharedBase>::default()));
    let onlyReader: Value<ReadsSharedBase> = Rc::new(RefCell::new(<ReadsSharedBase>::default()));
    assert!((shared_base_ctors_0.with(|rc| *rc.borrow()) == 3));
    ({ WritesSharedBaseImpl::write(&onlyWriter.as_pointer(), 6) });
    assert!((({ ReadsSharedBaseImpl::read(&onlyReader.as_pointer(),) }) == 0));
    assert!(({ ({ ((*d.borrow()) as SharedBase).value } == 5) }));
    return 0;
}
pub trait ReadsSharedBaseImpl {
    fn read(&self) -> i32;
}
impl ReadsSharedBaseImpl for Ptr<ReadsSharedBase> {
    fn read(&self) -> i32 {
        return ((*self) as *mut SharedBase).with(|__s| __s.value);
    }
}
pub trait WritesSharedBaseImpl {
    fn write(&self, x: i32);
}
impl WritesSharedBaseImpl for Ptr<WritesSharedBase> {
    fn write(&self, mut x: i32) {
        field!(((*self) as *mut SharedBase), value).write(x);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = shared_base_ctors_0.with(|_| ());
}
