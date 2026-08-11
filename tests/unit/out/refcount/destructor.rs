extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[repr(C)]
#[derive()]
pub struct Counter {
    pub bits: i32,
}
pub trait CounterMethods {
    fn units(&self) -> i32;
    fn __dtor(&self);
}
impl Clone for Counter {
    fn clone(&self) -> Self {
        let mut this = Self { bits: self.bits };
        this
    }
}
impl Default for Counter {
    fn default() -> Self {
        Counter { bits: 16 }
    }
}
impl ByteRepr for Counter {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.bits.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            bits: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[repr(C)]
#[derive()]
pub struct Watcher {
    pub target: Ptr<i32>,
}
pub trait WatcherMethods {
    fn __dtor(&self);
}
impl Clone for Watcher {
    fn clone(&self) -> Self {
        let mut this = Self {
            target: (self.target).clone(),
        };
        this
    }
}
impl Default for Watcher {
    fn default() -> Self {
        Watcher {
            target: Ptr::<i32>::null(),
        }
    }
}
impl ByteRepr for Watcher {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.target.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            target: <Ptr<i32>>::from_bytes(&buf[0..8]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Owner {
    pub watcher: Watcher,
}
pub trait OwnerMethods {
    fn __dtor(&self);
}
impl Clone for Owner {
    fn clone(&self) -> Self {
        let mut this = Self {
            watcher: (self.watcher).clone(),
        };
        this
    }
}
impl ByteRepr for Owner {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.watcher.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            watcher: <Watcher>::from_bytes(&buf[0..8]),
        }
    }
}
#[repr(C)]
#[derive()]
pub struct Tracker {
    pub step: i32,
}
pub trait TrackerMethods {
    fn __dtor(&self);
}
impl Clone for Tracker {
    fn clone(&self) -> Self {
        let mut this = Self { step: self.step };
        this
    }
}
impl Default for Tracker {
    fn default() -> Self {
        Tracker { step: 4 }
    }
}
impl ByteRepr for Tracker {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.step.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            step: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    {
        let c: Value<Counter> = Rc::new(RefCell::new(<Counter>::default()));
        let _dtor_c = ScopedDestructor::new(&c, |__p| __p.__dtor());
    }
    assert!(((*total_0.with(Value::clone).borrow()) == 2));
    let value: Value<i32> = Rc::new(RefCell::new(40));
    {
        let o: Value<Owner> = Rc::new(RefCell::new(<Owner>::default()));
        let _dtor_o = ScopedDestructor::new(&o, |__p| __p.__dtor());
        (*o.borrow_mut()).watcher.target = (value.as_pointer());
    }
    assert!(((*total_0.with(Value::clone).borrow()) == 42));
    {
        let t: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::default()));
        let _dtor_t = ScopedDestructor::new(&t, |__p| __p.__dtor());
    }
    assert!(((*total_0.with(Value::clone).borrow()) == 46));
    return 0;
}
impl CounterMethods for Ptr<Counter> {
    fn units(&self) -> i32 {
        return (self.with(|__v| __v.bits / 8));
    }
    fn __dtor(&self) {
        (*total_0.with(Value::clone).borrow_mut()) += ({ self.units() });
    }
}
impl OwnerMethods for Ptr<Owner> {
    fn __dtor(&self) {
        self.field_ptr(
            0,
            |__v: &Owner| ::std::slice::from_ref(&__v.watcher),
            |__v: &mut Owner| ::std::slice::from_mut(&mut __v.watcher),
        )
        .__dtor();
    }
}
impl TrackerMethods for Ptr<Tracker> {
    fn __dtor(&self) {
        (*total_0.with(Value::clone).borrow_mut()) += self.with(|__v| __v.step);
    }
}
impl WatcherMethods for Ptr<Watcher> {
    fn __dtor(&self) {
        if !((self.with(|__v| __v.target.clone())).is_null()) {
            {
                let __rhs = (self.with(|__v| __v.target.clone()).read());
                (*total_0.with(Value::clone).borrow_mut()) += __rhs
            };
            self.with_mut(|__v| __v.target = Ptr::<i32>::null());
        }
    }
}
