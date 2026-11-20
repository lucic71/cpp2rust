extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p2_6_pointer_to_member_argument_HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl p2_6_pointer_to_member_argument_HasTwoInts {
    pub fn new_1(this: Ptr<p2_6_pointer_to_member_argument_HasTwoInts>) {}
    pub fn copy_from(
        this: Ptr<p2_6_pointer_to_member_argument_HasTwoInts>,
        _a0: Ptr<p2_6_pointer_to_member_argument_HasTwoInts>,
    ) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
    pub fn move_from(
        this: Ptr<p2_6_pointer_to_member_argument_HasTwoInts>,
        _a0: Ptr<p2_6_pointer_to_member_argument_HasTwoInts>,
    ) {
        field!(this, first).write({ (*_a0.upgrade().deref()).first });
        field!(this, second).write({ (*_a0.upgrade().deref()).second });
    }
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_
{}
impl p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_ { pub fn new_1 ( this: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_>, ) { } pub fn copy_from ( this: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_>, _a0 : Ptr< p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_  > , ) { } pub fn move_from ( this: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_>, _a0 : Ptr< p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_  > , ) { } }
thread_local!(
    pub static count_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_
{}
impl p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_ { pub fn new_1 ( this: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_>, ) { } pub fn copy_from ( this: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_>, _a0 : Ptr< p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_  > , ) { } pub fn move_from ( this: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_>, _a0 : Ptr< p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_  > , ) { } }
thread_local!(
    pub static count_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p2_6_pointer_to_member_argument_CountsPerMember_nullptr_ {}
impl p2_6_pointer_to_member_argument_CountsPerMember_nullptr_ {
    pub fn new_1(this: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_nullptr_>) {}
    pub fn copy_from(
        this: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_nullptr_>,
        _a0: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_nullptr_>,
    ) {
    }
    pub fn move_from(
        this: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_nullptr_>,
        _a0: Ptr<p2_6_pointer_to_member_argument_CountsPerMember_nullptr_>,
    ) {
    }
}
pub fn test_3() {
    (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
    assert!((count_0.with(|rc| *rc.borrow()) == 1));
    assert!((count_1.with(|rc| *rc.borrow()) == 0));
    assert!((count_2.with(|rc| *rc.borrow()) == 0));
    count_1.with(|rc| *rc.borrow_mut() += 2);
    assert!((count_0.with(|rc| *rc.borrow()) == 1));
    assert!((count_1.with(|rc| *rc.borrow()) == 2));
    assert!((count_2.with(|rc| *rc.borrow()) == 0));
    count_2.with(|rc| *rc.borrow_mut() += 3);
    assert!((count_0.with(|rc| *rc.borrow()) == 1));
    assert!((count_1.with(|rc| *rc.borrow()) == 2));
    assert!((count_2.with(|rc| *rc.borrow()) == 3));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_3() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = count_1.with(|_| ());
    let _ = count_2.with(|_| ());
}
