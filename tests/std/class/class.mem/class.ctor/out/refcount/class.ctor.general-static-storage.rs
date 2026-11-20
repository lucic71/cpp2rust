extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom)]
#[placement]
#[cpp_from(i32, new_1, value)]
#[byte_size(16)]
pub struct Tracker {
    #[offset(0)]
    #[byte_size(8)]
    pub self_: Ptr<Tracker>,
    #[offset(8)]
    pub value: i32,
}
impl Tracker {
    pub fn new_2(this: Ptr<Tracker>) {
        field!(this, self_).write((this).clone());
        field!(this, value).write(-1_i32);
    }
    pub fn new_1(this: Ptr<Tracker>, mut value: i32) {
        field!(this, self_).write((this).clone());
        field!(this, value).write(value);
    }
    pub fn new_3(this: Ptr<Tracker>, mut a: i32, mut b: i32) {
        Tracker::new_1(this.clone(), { (a + b) });
    }
    pub fn copy_from(this: Ptr<Tracker>, other: Ptr<Tracker>) {
        field!(this, self_).write((this).clone());
        field!(this, value).write(other.with(|__s| __s.value));
    }
    pub fn move_from(this: Ptr<Tracker>, other: Ptr<Tracker>) {
        field!(this, self_).write((this).clone());
        field!(this, value).write(other.with(|__s| __s.value));
        field!(other, value).write(-2_i32);
    }
}
impl Clone for Tracker {
    fn clone(&self) -> Self {
        let __src: Value<Tracker> = Rc::new(RefCell::new(Tracker {
            self_: self.self_.clone(),
            value: self.value.clone(),
        }));
        let __dst: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
        Tracker::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl Default for Tracker {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
            Tracker::new_2(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn get_0() -> Ptr<Tracker> {
    return (once_1.with(|v| v.as_pointer()));
}
pub fn test_2() {
    let mut p: Ptr<Tracker> = ({ get_0() });
    assert!(
        ({ p.with(|__s| __s.self_.clone()) } == { (p).clone() }) && (p.with(|__s| __s.value) == 2)
    );
    assert!(({ ({ get_0() }) } == { (p).clone() }));
}
thread_local!(
    static once_1: Value<Tracker> = {
        let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
        Tracker::new_1(__v.as_pointer(), { 2 });
        __v
    };
);
thread_local!(
    pub static global_3: Value<Tracker> = {
        let __v: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::uninit()));
        Tracker::new_1(__v.as_pointer(), { 3 });
        __v
    };
);
pub fn test_4() {
    assert!(
        ({
            ({ { (*global_3.with(Value::clone).borrow()).self_.clone() } } == {
                (global_3.with(|v| v.as_pointer()))
            }) && ({ (*global_3.with(Value::clone).borrow()).value } == 3)
        })
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_2() });
    ({ test_4() });
    return 0;
}
pub trait TrackerImpl {
    fn is_self(&self) -> bool;
}
impl TrackerImpl for Ptr<Tracker> {
    fn is_self(&self) -> bool {
        return ((*self).with(|__s| __s.self_.clone()) == (*self));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = global_3.with(|_| ());
}
