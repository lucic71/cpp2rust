extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Handler {
    pub tag: i32,
    pub cb: FnPtr<fn(i32) -> i32>,
}
impl Clone for Handler {
    fn clone(&self) -> Self {
        let mut this = Self {
            tag: self.tag,
            cb: (self.cb).clone(),
        };
        this
    }
}
impl Default for Handler {
    fn default() -> Self {
        Handler {
            tag: <i32>::default(),
            cb: FnPtr::<fn(i32) -> i32>::null(),
        }
    }
}
impl ByteRepr for Handler {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.tag.to_bytes(&mut buf[0..4]);
        self.cb.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: <i32>::from_bytes(&buf[0..4]),
            cb: <FnPtr<fn(i32) -> i32>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn double_it_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn negate_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return -(*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h1: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: 1,
        cb: FnPtr::<fn(i32) -> i32>::new(double_it_0),
    }));
    let h2: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: 2,
        cb: FnPtr::<fn(i32) -> i32>::new(negate_1),
    }));
    assert!(!(((*h1.borrow()).cb).is_null()));
    assert!((({ (*(*h1.borrow()).cb)(5,) }) == 10));
    assert!((({ (*(*h2.borrow()).cb)(7,) }) == -7_i32));
    (*h1.borrow_mut()).cb = FnPtr::<fn(i32) -> i32>::new(negate_1);
    assert!((({ (*(*h1.borrow()).cb)(3,) }) == -3_i32));
    assert!({
        let _lhs = ((*h1.borrow()).cb).clone();
        _lhs == ((*h2.borrow()).cb).clone()
    });
    return 0;
}
