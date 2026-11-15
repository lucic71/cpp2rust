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
    pub static base_stamp_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static first_stamp_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static second_stamp_3: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static base_dtors_4: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static first_dtors_5: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static second_dtors_6: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct HasVirtualDestructor {}
impl HasVirtualDestructor {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct OverridesDestructor {}
impl OverridesDestructor {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct OverridesDestructorAgain {}
impl OverridesDestructorAgain {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Ptr<HasVirtualDestructor> = Ptr::alloc(<OverridesDestructorAgain>::default());
    {
        let __p = p;
        __p.destructor();
        __p.delete();
    };
    assert!(
        ((second_dtors_6.with(|rc| *rc.borrow()) == 1)
            && (first_dtors_5.with(|rc| *rc.borrow()) == 1))
            && (base_dtors_4.with(|rc| *rc.borrow()) == 1)
    );
    assert!((second_stamp_3.with(|rc| *rc.borrow()) == 1));
    assert!((first_stamp_2.with(|rc| *rc.borrow()) == 2));
    assert!((base_stamp_1.with(|rc| *rc.borrow()) == 3));
    p = Ptr::alloc(<OverridesDestructor>::default());
    {
        let __p = p;
        __p.destructor();
        __p.delete();
    };
    assert!(
        ((second_dtors_6.with(|rc| *rc.borrow()) == 1)
            && (first_dtors_5.with(|rc| *rc.borrow()) == 2))
            && (base_dtors_4.with(|rc| *rc.borrow()) == 2)
    );
    assert!((first_stamp_2.with(|rc| *rc.borrow()) == 4));
    assert!((base_stamp_1.with(|rc| *rc.borrow()) == 5));
    return 0;
}
impl HasVirtualDestructor for OverridesDestructor {}
impl OverridesDestructor for OverridesDestructorAgain {}
pub fn __cpp2rust_init_globals() {
    let _ = sequence_0.with(|_| ());
    let _ = base_stamp_1.with(|_| ());
    let _ = first_stamp_2.with(|_| ());
    let _ = second_stamp_3.with(|_| ());
    let _ = base_dtors_4.with(|_| ());
    let _ = first_dtors_5.with(|_| ());
    let _ = second_dtors_6.with(|_| ());
}
