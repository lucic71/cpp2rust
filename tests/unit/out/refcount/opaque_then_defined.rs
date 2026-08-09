extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Clone, Default)]
pub struct list {
    pub head: Ptr<node>,
    pub size: i32,
}
impl ByteRepr for list {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.head.to_bytes(&mut buf[0..8]);
        self.size.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            head: <Ptr<node>>::from_bytes(&buf[0..8]),
            size: <i32>::from_bytes(&buf[8..12]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct node {
    pub value: i32,
    pub next: Ptr<node>,
}
impl ByteRepr for node {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.value.to_bytes(&mut buf[0..4]);
        self.next.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: <i32>::from_bytes(&buf[0..4]),
            next: <Ptr<node>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let n: Value<node> = Rc::new(RefCell::new(node {
        value: 42,
        next: Ptr::<node>::null(),
    }));
    let l: Value<list> = Rc::new(RefCell::new(list {
        head: (n.as_pointer()),
        size: 1,
    }));
    assert!(((((*l.borrow()).head.with(|__v| __v.value == 42)) as i32) != 0));
    assert!(((((*l.borrow()).size == 1) as i32) != 0));
    return 0;
}
