extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Probe {}
impl Probe {
    pub fn new_1(this: Ptr<Probe>) {}
    pub fn copy_from(this: Ptr<Probe>, _a0: Ptr<Probe>) {}
    pub fn move_from(this: Ptr<Probe>, _a0: Ptr<Probe>) {}
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Wrapper_Probe_ {
    #[offset(0)]
    #[byte_size(1)]
    pub base_: Probe,
    #[offset(4)]
    pub tag: i32,
}
impl Wrapper_Probe_ {
    pub fn new_1(this: Ptr<Wrapper_Probe_>) {
        Probe::new_1(field_ptr!(this, base_));
    }
    pub fn copy_from(this: Ptr<Wrapper_Probe_>, _a0: Ptr<Wrapper_Probe_>) {
        Probe::copy_from(field_ptr!(this, base_), { field_ptr!(_a0, base_) });
        field!(this, tag).write(_a0.with(|__s| __s.tag));
    }
    pub fn move_from(this: Ptr<Wrapper_Probe_>, _a0: Ptr<Wrapper_Probe_>) {
        Probe::move_from(field_ptr!(this, base_), { field_ptr!(_a0, base_) });
        field!(this, tag).write({ (*_a0.upgrade().deref()).tag });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Wrapper_Probe_> = Rc::new(RefCell::new(<Wrapper_Probe_>::uninit()));
    {
        let __d = a.as_pointer();
        {
            let __d = field_ptr!(__d, base_);
        };
        ();
    };
    (*a.borrow_mut()).tag = 3;
    let b: Value<Wrapper_Probe_> = Rc::new(RefCell::new(<Wrapper_Probe_>::uninit()));
    Wrapper_Probe_::copy_from(b.as_pointer(), { a.as_pointer() });
    assert!(({ ({ (*b.borrow()).tag } == 3) }));
    return 0;
}
pub trait ProbeImpl {
    fn operator_inc(&self) -> Ptr<Probe> {
        unimplemented!()
    }
}
impl ProbeImpl for Ptr<Probe> {}
pub fn __cpp2rust_init_globals() {}
