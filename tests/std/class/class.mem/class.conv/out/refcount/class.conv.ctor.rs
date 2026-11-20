extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
impl p1_template_constructor_from_own_type_Probe {
    pub fn new_1(
        this: Ptr<p1_template_constructor_from_own_type_Probe>,
        u: Ptr<p1_template_constructor_from_own_type_Probe>,
    ) {
        field!(this, value).write((u.with(|__s| __s.value) + 1));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new_2, value)]
#[byte_size(4)]
pub struct p1_template_constructor_from_own_type_Probe {
    #[offset(0)]
    pub value: i32,
}
impl p1_template_constructor_from_own_type_Probe {
    pub fn new_2(this: Ptr<p1_template_constructor_from_own_type_Probe>, mut v: i32) {
        field!(this, value).write(v);
    }
    pub fn copy_from(
        this: Ptr<p1_template_constructor_from_own_type_Probe>,
        _a0: Ptr<p1_template_constructor_from_own_type_Probe>,
    ) {
        field!(this, value).write(_a0.with(|__s| __s.value));
    }
}
pub fn test_0() {
    let probe: Value<p1_template_constructor_from_own_type_Probe> = Rc::new(RefCell::new(
        <p1_template_constructor_from_own_type_Probe>::uninit(),
    ));
    p1_template_constructor_from_own_type_Probe::new_2(probe.as_pointer(), { 22 });
    let converted: Value<p1_template_constructor_from_own_type_Probe> = Rc::new(RefCell::new(
        <p1_template_constructor_from_own_type_Probe>::uninit(),
    ));
    p1_template_constructor_from_own_type_Probe::new_1(converted.as_pointer(), {
        probe.as_pointer()
    });
    assert!(({ ({ (*converted.borrow()).value } == 23) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
