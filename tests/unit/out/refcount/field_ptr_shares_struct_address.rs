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
pub struct conn {
    pub first: i32,
    pub port: i32,
}
impl ByteRepr for conn {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.first.to_bytes(&mut buf[0..4]);
        self.port.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: <i32>::from_bytes(&buf[0..4]),
            port: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let c: Value<Ptr<conn>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(::std::mem::size_of::<conn>()).reinterpret_cast::<conn>(),
    ));
    (*c.borrow()).with_mut(|__v| __v.port = 443);
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((*c.borrow()).field_ptr(
            0,
            |__v: &conn| ::std::slice::from_ref(&__v.first),
            |__v: &mut conn| ::std::slice::from_mut(&mut __v.first),
        )),
    ));
    (*p.borrow()).write(1);
    assert!(((((*c.borrow()).with(|__v| __v.first) == 1) as i32) != 0));
    assert!(((((*c.borrow()).with(|__v| __v.port) == 443) as i32) != 0));
    libcc2rs::free_refcount(((*c.borrow()).clone() as Ptr<conn>).to_any().clone());
    return 0;
}
