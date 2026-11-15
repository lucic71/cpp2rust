extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static int_ctors_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct VirtualBase {
    #[offset(0)]
    pub value: i32,
}
impl VirtualBase {
    pub fn new(mut v: i32) -> Self {
        let __this: VirtualBase = Self { value: v };
        (*int_ctors_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct DeletesDefaultConstructor {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct InheritsOnce {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct InheritsThroughTwoLevels {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let twoLevels: Value<InheritsThroughTwoLevels> =
        Rc::new(RefCell::new(InheritsThroughTwoLevels::new_1({ 0 })));
    assert!((int_ctors_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ ((*twoLevels.borrow()) as VirtualBase).value } == 0) }));
    let once: Value<InheritsOnce> = Rc::new(RefCell::new(InheritsOnce::new_1({ 4 })));
    assert!((int_ctors_0.with(|rc| *rc.borrow()) == 2));
    assert!(({ ({ ((*once.borrow()) as VirtualBase).value } == 4) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = int_ctors_0.with(|_| ());
}
