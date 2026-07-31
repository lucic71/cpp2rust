extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Foo {
    pub x1: i32,
    pub x2: i32,
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: self.x1,
            x2: self.x2,
        };
        this
    }
}
impl ByteRepr for Foo {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..4]);
        self.x2.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <i32>::from_bytes(&buf[0..4]),
            x2: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
        Foo { x1: 1, x2: 2 },
        Foo { x1: 3, x2: 4 },
    ])));
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<Foo>).offset(1).field_ptr(
            0,
            |__v: &Foo| ::std::slice::from_ref(&__v.x1),
            |__v: &mut Foo| ::std::slice::from_mut(&mut __v.x1),
        )),
    ));
    let a: Value<i32> = Rc::new(RefCell::new(((*p1.borrow()).read())));
    let p2: Value<Ptr<Foo>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<Foo>).offset(0))));
    return {
        let _lhs = (*a.borrow());
        _lhs + (*(*p2.borrow()).upgrade().deref()).x2
    };
}
