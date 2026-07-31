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
    pub x: i32,
    pub y: i32,
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
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn sum_0(p: Point) -> i32 {
    let p: Value<Point> = Rc::new(RefCell::new(p));
    return ((*p.borrow()).x + (*p.borrow()).y);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Option<Value<Point>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Point {
            x: 3,
            y: 4,
        })))));
    (*(*p.borrow_mut()).as_ref().unwrap().borrow_mut()).x += 10;
    let __rhs = ((*(*p.borrow()).as_ref().unwrap().borrow()).x
        + (*(*p.borrow()).as_ref().unwrap().borrow()).y);
    (*(*p.borrow_mut()).as_ref().unwrap().borrow_mut()).y = __rhs;
    let s: Value<i32> = Rc::new(RefCell::new(
        ({ sum_0((*(*p.borrow()).as_ref().unwrap().borrow()).clone()) }),
    ));
    return (*s.borrow());
}
