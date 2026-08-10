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
pub struct big {
    pub a: i64,
    pub b: i64,
}
impl ByteRepr for big {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..8]);
        self.b.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i64>::from_bytes(&buf[0..8]),
            b: <i64>::from_bytes(&buf[8..16]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct outer {
    pub pad: i64,
    pub big: big,
}
impl ByteRepr for outer {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.pad.to_bytes(&mut buf[0..8]);
        self.big.to_bytes(&mut buf[8..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            pad: <i64>::from_bytes(&buf[0..8]),
            big: <big>::from_bytes(&buf[8..24]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct holder {
    pub p: Ptr<big>,
}
impl ByteRepr for holder {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.p.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: <Ptr<big>>::from_bytes(&buf[0..8]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let o: Value<Ptr<outer>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(::std::mem::size_of::<outer>()).reinterpret_cast::<outer>(),
    ));
    (*o.borrow()).with_mut(|__v| __v.pad = 1_i64);
    (*o.borrow()).with_mut(|__v| __v.big.a = 2_i64);
    (*o.borrow()).with_mut(|__v| __v.big.b = 3_i64);
    let h: Value<Ptr<holder>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(8usize).reinterpret_cast::<holder>(),
    ));
    {
        let __rhs = ((*o.borrow()).field_ptr(
            8,
            |__v: &outer| ::std::slice::from_ref(&__v.big),
            |__v: &mut outer| ::std::slice::from_mut(&mut __v.big),
        ));
        (*h.borrow()).with_mut(|__v| __v.p = __rhs)
    };
    assert!(((((*h.borrow()).with(|__v| __v.p.with(|__v| __v.a) == 2_i64)) as i32) != 0));
    {
        let __obj = (*h.borrow()).with(|__v| __v.p.clone());
        __obj.with_mut(|__v| __v.b = 9_i64)
    };
    assert!(((((*o.borrow()).with(|__v| __v.big.b == 9_i64)) as i32) != 0));
    assert!(((((*o.borrow()).with(|__v| __v.pad == 1_i64)) as i32) != 0));
    libcc2rs::free_refcount(((*h.borrow()).clone() as Ptr<holder>).to_any().clone());
    libcc2rs::free_refcount(((*o.borrow()).clone() as Ptr<outer>).to_any().clone());
    return 0;
}
