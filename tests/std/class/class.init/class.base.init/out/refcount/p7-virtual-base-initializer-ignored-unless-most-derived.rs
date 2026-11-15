extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static default_ctors_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static int_ctors_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct VirtualBase {
    #[offset(0)]
    pub value: i32,
}
impl VirtualBase {
    pub fn new_2() -> Self {
        let __this: VirtualBase = Self { value: -1_i32 };
        (*default_ctors_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_1(mut i: i32) -> Self {
        let __this: VirtualBase = Self { value: i };
        (*int_ctors_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for VirtualBase {
    fn default() -> Self {
        { VirtualBase::new_2() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(16)]
pub struct PassesArgumentToVirtualBase {}
impl PassesArgumentToVirtualBase {
    pub fn new(mut i: i32) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(16)]
pub struct HasNoInitializerForVirtualBase {}
impl HasNoInitializerForVirtualBase {
    pub fn new(mut _a0: i32) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(24)]
pub struct JoinsBoth {}
impl JoinsBoth {
    pub fn new(mut i: i32) -> Self {
        Self {}
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut base: VirtualBase = VirtualBase::new_1({ 1 });
    assert!(
        (default_ctors_0.with(|rc| *rc.borrow()) == 0)
            && (int_ctors_1.with(|rc| *rc.borrow()) == 1)
    );
    assert!((base.value == 1));
    let passes: Value<PassesArgumentToVirtualBase> =
        Rc::new(RefCell::new(PassesArgumentToVirtualBase::new({ 2 })));
    assert!(
        (default_ctors_0.with(|rc| *rc.borrow()) == 0)
            && (int_ctors_1.with(|rc| *rc.borrow()) == 2)
    );
    assert!(({ ({ ((*passes.borrow()) as VirtualBase).value } == 2) }));
    let hasNone: Value<HasNoInitializerForVirtualBase> =
        Rc::new(RefCell::new(HasNoInitializerForVirtualBase::new({ 3 })));
    assert!(
        (default_ctors_0.with(|rc| *rc.borrow()) == 1)
            && (int_ctors_1.with(|rc| *rc.borrow()) == 2)
    );
    assert!(({ ({ ((*hasNone.borrow()) as VirtualBase).value } == -1_i32) }));
    let joined: Value<JoinsBoth> = Rc::new(RefCell::new(JoinsBoth::new({ 4 })));
    assert!(
        (default_ctors_0.with(|rc| *rc.borrow()) == 2)
            && (int_ctors_1.with(|rc| *rc.borrow()) == 2)
    );
    assert!(({ ({ ((*joined.borrow()) as VirtualBase).value } == -1_i32) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = default_ctors_0.with(|_| ());
    let _ = int_ctors_1.with(|_| ());
}
