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
pub struct inner {
    pub a: i64,
    pub b: i64,
}
impl ByteRepr for inner {
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
    pub in_: inner,
    pub tag: i64,
}
impl ByteRepr for outer {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.in_.to_bytes(&mut buf[0..16]);
        self.tag.to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            in_: <inner>::from_bytes(&buf[0..16]),
            tag: <i64>::from_bytes(&buf[16..24]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct holder {
    pub words: Ptr<i64>,
    pub field: Ptr<inner>,
}
impl ByteRepr for holder {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.words.to_bytes(&mut buf[0..8]);
        self.field.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            words: <Ptr<i64>>::from_bytes(&buf[0..8]),
            field: <Ptr<inner>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Ptr<outer>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(::std::mem::size_of::<outer>()).reinterpret_cast::<outer>(),
    ));
    (*o.borrow()).with_mut(|__v| __v.tag = 7_i64);
    let h: Value<Ptr<holder>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(16usize).reinterpret_cast::<holder>(),
    ));
    {
        let __rhs = libcc2rs::malloc_refcount(
            (2_usize).wrapping_mul((::std::mem::size_of::<i64>() as usize)),
        )
        .reinterpret_cast::<i64>();
        (*h.borrow()).with_mut(|__v| __v.words = __rhs)
    };
    (*h.borrow()).with_mut(|__v| {
        __v.field = ((*o.borrow()).field_ptr(
            0,
            |__v: &outer| ::std::slice::from_ref(&__v.in_),
            |__v: &mut outer| ::std::slice::from_mut(&mut __v.in_),
        ))
    });
    (*h.borrow())
        .with(|__v| (*__v).words.offset(((0) as isize)).clone())
        .write(11_i64);
    (*h.borrow())
        .with(|__v| (*__v).words.offset(((1) as isize)).clone())
        .write(22_i64);
    (*h.borrow()).with(|__v| (*__v).field.clone().with_mut(|__v| __v.a = 33_i64));
    assert!(
        (((((*h.borrow())
            .with(|__v| (*__v).words.offset(((0) as isize)).clone())
            .read())
            == 11_i64) as i32)
            != 0)
    );
    assert!(
        (((((*h.borrow())
            .with(|__v| (*__v).words.offset(((1) as isize)).clone())
            .read())
            == 22_i64) as i32)
            != 0)
    );
    assert!(
        ((((*h.borrow()).with(|__v| (*__v).field.clone().with(|__v| (*__v).a)) == 33_i64) as i32)
            != 0)
    );
    assert!(((((*o.borrow()).with(|__v| (*__v).tag) == 7_i64) as i32) != 0));
    libcc2rs::free_refcount(
        (((*h.borrow()).with(|__v| (*__v).words.clone())).clone() as Ptr<i64>).to_any(),
    );
    libcc2rs::free_refcount(((*h.borrow()).clone() as Ptr<holder>).to_any());
    libcc2rs::free_refcount(((*o.borrow()).clone() as Ptr<outer>).to_any());
    return 0;
}
