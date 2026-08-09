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
pub struct XX {
    pub x: i32,
}
impl Clone for XX {
    fn clone(&self) -> Self {
        let mut this = Self { x: self.x };
        this
    }
}
impl ByteRepr for XX {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let obj: Value<XX> = Rc::new(RefCell::new(<XX>::default()));
    let ptr: Value<Ptr<XX>> = Rc::new(RefCell::new((obj.as_pointer())));
    (*ptr.borrow()).with_mut(|__v| __v.x = 2);
    let c: Value<bool> = Rc::new(RefCell::new(false));
    let r: Value<i32> = Rc::new(RefCell::new(if (*c.borrow()) {
        (*obj.borrow()).x
    } else {
        (*ptr.borrow()).with(|__v| __v.x)
    }));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (obj.as_pointer().field_ptr(
            0,
            |__v: &XX| ::std::slice::from_ref(&__v.x),
            |__v: &mut XX| ::std::slice::from_mut(&mut __v.x),
        )),
    ));
    return {
        let _lhs = ((*p.borrow()).read());
        _lhs + (*r.borrow())
    };
}
