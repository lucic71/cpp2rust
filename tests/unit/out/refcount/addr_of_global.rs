extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct Inner {
    pub value: i32,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let mut this = Self { value: self.value };
        this
    }
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.value.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Outer {
    pub p: Ptr<Inner>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let mut this = Self {
            p: (self.p).clone(),
        };
        this
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.p.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: <Ptr<Inner>>::from_bytes(&buf[0..8]),
        }
    }
}
thread_local!(
    pub static alpha_0: Value<Inner> = Rc::new(RefCell::new(Inner { value: 1 }));
);
thread_local!(
    pub static beta_1: Value<Inner> = Rc::new(RefCell::new(Inner { value: 2 }));
);
thread_local!(
    pub static shared_2: Value<Inner> = Rc::new(RefCell::new(Inner { value: 42 }));
);
thread_local!(
    pub static items_3: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
        (alpha_0.with(Value::clone).as_pointer()),
        (beta_1.with(Value::clone).as_pointer()),
    ])));
);
thread_local!(
    pub static obj_4: Value<Outer> = Rc::new(RefCell::new(Outer {
        p: (shared_2.with(Value::clone).as_pointer()),
    }));
);
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!(((*items_3.with(Value::clone).borrow())[(0) as usize].with(|__v| (*__v).value) == 1));
    assert!(((*items_3.with(Value::clone).borrow())[(1) as usize].with(|__v| (*__v).value) == 2));
    assert!(
        ((*obj_4.with(Value::clone).borrow())
            .p
            .with(|__v| (*__v).value)
            == 42)
    );
    thread_local!(
        static cache_5: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
            (alpha_0.with(Value::clone).as_pointer()),
            (beta_1.with(Value::clone).as_pointer()),
        ])));
    );
    assert!(((*cache_5.with(Value::clone).borrow())[(0) as usize].with(|__v| (*__v).value) == 1));
    assert!(((*cache_5.with(Value::clone).borrow())[(1) as usize].with(|__v| (*__v).value) == 2));
    return 0;
}
