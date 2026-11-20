extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Tracker {
    #[offset(0)]
    pub tag: i32,
}
impl Tracker {
    pub fn new(this: Ptr<Tracker>, mut tag: i32) {
        field!(this, tag).write(tag);
        printf!(b"construct %d\n", tag);
    }
}
thread_local!(
    pub static global_0: Value<Tracker> = {
        let __static = {
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new(__v.as_pointer(), { 1 });
            __v
        };
        libcc2rs::register_static_destructor(|| {
            global_0.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn exit_from_nested_call_1() {
    printf!(b"local static %d\n", {
        (*local_static_2.with(Value::clone).borrow()).tag
    });
    let automatic: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
    Tracker::new(automatic.as_pointer(), { 3 });
    let _dtor_automatic =
        std::mem::DropGuard::new(automatic.clone(), |__p| __p.as_pointer().destructor());
    libcc2rs::cpp2rust_exit(0);
}
thread_local!(
    static local_static_2: Value<Tracker> = {
        let __static = {
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new(__v.as_pointer(), { 2 });
            __v
        };
        libcc2rs::register_static_destructor(|| {
            local_static_2.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ exit_from_nested_call_1() });
    printf!(b"not reached\n");
    return 0;
}
pub trait TrackerImpl {
    fn destructor(&self);
}
impl TrackerImpl for Ptr<Tracker> {
    fn destructor(&self) {
        printf!(b"destroy %d\n", (*self).with(|__s| __s.tag));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = global_0.with(|_| ());
}
