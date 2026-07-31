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
pub struct Point {
    pub x: u32,
    pub y: u32,
}
impl Clone for Point {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: self.x,
            y: self.y,
        };
        this
    }
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <u32>::from_bytes(&buf[0..4]),
            y: <u32>::from_bytes(&buf[4..8]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Pair {
    pub first: u32,
    pub second: u32,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            first: self.first,
            second: self.second,
        };
        this
    }
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.first.to_bytes(&mut buf[0..4]);
        self.second.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: <u32>::from_bytes(&buf[0..4]),
            second: <u32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let pt: Value<Point> = Rc::new(RefCell::new(Point {
        x: 10_u32,
        y: 20_u32,
    }));
    let pair: Value<Ptr<Pair>> =
        Rc::new(RefCell::new((pt.as_pointer()).reinterpret_cast::<Pair>()));
    assert!(((*pair.borrow()).with(|__v| (*__v).first) == 10_u32));
    assert!(((*pair.borrow()).with(|__v| (*__v).second) == 20_u32));
    (*pair.borrow()).with_mut(|__v| __v.first = 42_u32);
    assert!(((*pt.borrow()).x == 42_u32));
    return 0;
}
