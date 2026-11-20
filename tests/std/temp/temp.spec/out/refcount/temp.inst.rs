extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[placement]
#[byte_size(4)]
pub struct p2_reference_does_not_instantiate_Member {
    #[offset(0)]
    pub value: i32,
}
impl p2_reference_does_not_instantiate_Member {
    pub fn new_1(this: Ptr<p2_reference_does_not_instantiate_Member>) {
        field!(this, value).write(1);
    }
    pub fn new_2(
        this: Ptr<p2_reference_does_not_instantiate_Member>,
        _a0: Ptr<p2_reference_does_not_instantiate_Wrapper_int_>,
    ) {
        field!(this, value).write(2);
    }
    pub fn copy_from(
        this: Ptr<p2_reference_does_not_instantiate_Member>,
        _a0: Ptr<p2_reference_does_not_instantiate_Member>,
    ) {
        field!(this, value).write(_a0.with(|__s| __s.value));
    }
    pub fn move_from(
        this: Ptr<p2_reference_does_not_instantiate_Member>,
        _a0: Ptr<p2_reference_does_not_instantiate_Member>,
    ) {
        field!(this, value).write({ (*_a0.upgrade().deref()).value });
    }
}
impl Default for p2_reference_does_not_instantiate_Member {
    fn default() -> Self {
        { p2_reference_does_not_instantiate_Member { value: 1 } }
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, Default)]
#[placement]
#[byte_size(4)]
pub struct p2_reference_does_not_instantiate_Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub member: p2_reference_does_not_instantiate_Member,
}
impl p2_reference_does_not_instantiate_Holder {
    pub fn new_1(this: Ptr<p2_reference_does_not_instantiate_Holder>) {
        p2_reference_does_not_instantiate_Member::new_1(field_ptr!(this, member));
    }
    pub fn move_from(
        this: Ptr<p2_reference_does_not_instantiate_Holder>,
        _a0: Ptr<p2_reference_does_not_instantiate_Holder>,
    ) {
        p2_reference_does_not_instantiate_Member::move_from(field_ptr!(this, member), {
            field_ptr!(_a0, member)
        });
    }
}
pub fn test_0() {
    let holder: Value<p2_reference_does_not_instantiate_Holder> = Rc::new(RefCell::new(
        <p2_reference_does_not_instantiate_Holder>::uninit(),
    ));
    p2_reference_does_not_instantiate_Holder::new_1(holder.as_pointer());
    assert!(({ ({ (*holder.borrow()).member.value } == 1) }));
    (*holder.borrow_mut()).member.value = 3;
    assert!(({ ({ (*holder.borrow()).member.value } == 3) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct p2_reference_does_not_instantiate_Wrapper_int_;
pub fn __cpp2rust_init_globals() {}
