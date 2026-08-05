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
pub struct pair {
    pub a: i32,
    pub b: i32,
}
impl ByteRepr for pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
        self.b.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
            b: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn bump_0(s: Ptr<pair>) -> i32 {
    let s: Value<Ptr<pair>> = Rc::new(RefCell::new(s));
    (*s.borrow()).with_mut(|__v| __v.b += 10);
    return (*s.borrow()).with(|__v| (*__v).b);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<pair>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, ::std::mem::size_of::<pair>())
            .reinterpret_cast::<pair>(),
    ));
    (*s.borrow()).with_mut(|__v| __v.b = 1);
    {
        let __rhs = ({ bump_0((*s.borrow()).clone()) });
        (*s.borrow()).with_mut(|__v| __v.a = __rhs)
    };
    assert!(((((*s.borrow()).with(|__v| (*__v).a) == 11) as i32) != 0));
    assert!(((((*s.borrow()).with(|__v| (*__v).b) == 11) as i32) != 0));
    libcc2rs::free_refcount(((*s.borrow()).clone() as Ptr<pair>).to_any().clone());
    return 0;
}
