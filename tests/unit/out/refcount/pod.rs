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
pub struct POD {
    pub x1: i32,
    pub x2: i32,
    pub x3: i32,
}
impl Clone for POD {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: self.x1,
            x2: self.x2,
            x3: self.x3,
        };
        this
    }
}
impl ByteRepr for POD {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..4]);
        self.x2.to_bytes(&mut buf[4..8]);
        self.x3.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <i32>::from_bytes(&buf[0..4]),
            x2: <i32>::from_bytes(&buf[4..8]),
            x3: <i32>::from_bytes(&buf[8..12]),
        }
    }
}
pub fn PODIncrement_0(pod: Ptr<POD>) {
    pod.with_mut(|__v| __v.x1 += 1);
    pod.with_mut(|__v| __v.x2 += 2);
    pod.with_mut(|__v| __v.x3 += 3);
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let p1: Value<POD> = Rc::new(RefCell::new(POD {
        x1: 10,
        x2: 11,
        x3: 12,
    }));
    let p2: Value<POD> = Rc::new(RefCell::new(POD {
        x1: (*p1.borrow()).x1,
        x2: (*p1.borrow()).x2,
        x3: (*p1.borrow()).x3,
    }));
    ({ PODIncrement_0(p2.as_pointer()) });
    return (((*p2.borrow()).x1 + (*p2.borrow()).x2) + (*p2.borrow()).x3);
}
