extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static sequence_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static indirect_base_stamp_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static first_base_stamp_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static second_base_stamp_3: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static first_member_stamp_4: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static second_member_stamp_5: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static body_stamp_6: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(Ptr< i32  >  , new, value)]
#[byte_size(8)]
pub struct StampsDestruction {
    #[offset(0)]
    #[byte_size(8)]
    pub stamp: Ptr<i32>,
}
impl StampsDestruction {
    pub fn new(mut s: Ptr<i32>) -> Self {
        Self { stamp: (s).clone() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct IndirectBase {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct FirstBase {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct SecondBase {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct HasBasesAndMembers {
    #[offset(0)]
    #[byte_size(8)]
    pub firstMember: StampsDestruction,
    #[offset(8)]
    #[byte_size(8)]
    pub secondMember: StampsDestruction,
}
impl HasBasesAndMembers {
    pub fn new() -> Self {
        Self {
            firstMember: StampsDestruction::new({
                (first_member_stamp_4.with(|v| v.as_pointer()))
            }),
            secondMember: StampsDestruction::new({
                (second_member_stamp_5.with(|v| v.as_pointer()))
            }),
        }
    }
}
impl Default for HasBasesAndMembers {
    fn default() -> Self {
        { HasBasesAndMembers::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let d: Value<HasBasesAndMembers> = Rc::new(RefCell::new(HasBasesAndMembers::new()));
        let _dtor_d = std::mem::DropGuard::new(d.clone(), |__p| __p.as_pointer().destructor());
        assert!((sequence_0.with(|rc| *rc.borrow()) == 0));
    }
    assert!((body_stamp_6.with(|rc| *rc.borrow()) == 1));
    assert!((second_member_stamp_5.with(|rc| *rc.borrow()) == 2));
    assert!((first_member_stamp_4.with(|rc| *rc.borrow()) == 3));
    assert!((second_base_stamp_3.with(|rc| *rc.borrow()) == 4));
    assert!((first_base_stamp_2.with(|rc| *rc.borrow()) == 5));
    assert!((indirect_base_stamp_1.with(|rc| *rc.borrow()) == 6));
    assert!((sequence_0.with(|rc| *rc.borrow()) == 6));
    return 0;
}
pub trait FirstBaseImpl {
    fn destructor(&self);
}
impl FirstBaseImpl for Ptr<FirstBase> {
    fn destructor(&self) {
        first_base_stamp_2.with(|rc| {
            *rc.borrow_mut() = (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc()
        });
    }
}
pub trait HasBasesAndMembersImpl {
    fn destructor(&self);
}
impl HasBasesAndMembersImpl for Ptr<HasBasesAndMembers> {
    fn destructor(&self) {
        body_stamp_6.with(|rc| {
            *rc.borrow_mut() = (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc()
        });
        field_ptr!(self, secondMember).destructor();
        field_ptr!(self, firstMember).destructor();
    }
}
pub trait IndirectBaseImpl {
    fn destructor(&self);
}
impl IndirectBaseImpl for Ptr<IndirectBase> {
    fn destructor(&self) {
        indirect_base_stamp_1.with(|rc| {
            *rc.borrow_mut() = (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc()
        });
    }
}
pub trait SecondBaseImpl {
    fn destructor(&self);
}
impl SecondBaseImpl for Ptr<SecondBase> {
    fn destructor(&self) {
        second_base_stamp_3.with(|rc| {
            *rc.borrow_mut() = (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc()
        });
    }
}
pub trait StampsDestructionImpl {
    fn destructor(&self);
}
impl StampsDestructionImpl for Ptr<StampsDestruction> {
    fn destructor(&self) {
        let __rhs = (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc();
        (*self).with(|__s| __s.stamp.clone()).write(__rhs);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = sequence_0.with(|_| ());
    let _ = indirect_base_stamp_1.with(|_| ());
    let _ = first_base_stamp_2.with(|_| ());
    let _ = second_base_stamp_3.with(|_| ());
    let _ = first_member_stamp_4.with(|_| ());
    let _ = second_member_stamp_5.with(|_| ());
    let _ = body_stamp_6.with(|_| ());
}
