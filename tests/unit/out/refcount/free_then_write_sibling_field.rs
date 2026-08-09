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
pub struct payload {
    pub value: i32,
}
impl ByteRepr for payload {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.value.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct holder {
    pub first: Ptr<payload>,
    pub second: Ptr<payload>,
    pub count: i32,
}
impl ByteRepr for holder {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.first.to_bytes(&mut buf[0..8]);
        self.second.to_bytes(&mut buf[8..16]);
        self.count.to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: <Ptr<payload>>::from_bytes(&buf[0..8]),
            second: <Ptr<payload>>::from_bytes(&buf[8..16]),
            count: <i32>::from_bytes(&buf[16..20]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let h: Value<Ptr<holder>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(24usize).reinterpret_cast::<holder>(),
    ));
    assert!((((!((*h.borrow()).is_null())) as i32) != 0));
    (*h.borrow()).with_mut(|__v| {
        __v.first = libcc2rs::malloc_refcount(::std::mem::size_of::<payload>())
            .reinterpret_cast::<payload>()
    });
    (*h.borrow()).with_mut(|__v| {
        __v.second = libcc2rs::malloc_refcount(::std::mem::size_of::<payload>())
            .reinterpret_cast::<payload>()
    });
    assert!((((!(((*h.borrow()).with(|__v| __v.first.clone())).is_null())) as i32) != 0));
    assert!((((!(((*h.borrow()).with(|__v| __v.second.clone())).is_null())) as i32) != 0));
    {
        let __obj = (*h.borrow()).with(|__v| __v.first.clone());
        __obj.with_mut(|__v| __v.value = 11)
    };
    {
        let __obj = (*h.borrow()).with(|__v| __v.second.clone());
        __obj.with_mut(|__v| __v.value = 22)
    };
    (*h.borrow()).with_mut(|__v| __v.count = 2);
    libcc2rs::free_refcount(
        (((*h.borrow()).with(|__v| __v.first.clone())) as Ptr<payload>)
            .to_any()
            .clone(),
    );
    (*h.borrow()).with_mut(|__v| __v.count = 1);
    assert!(((((*h.borrow()).with(|__v| __v.count == 1)) as i32) != 0));
    assert!(
        ((((*h.borrow()).with(|__v| __v.second.clone().with(|__v| __v.value) == 22)) as i32) != 0)
    );
    (*h.borrow()).with_mut(|__v| __v.first = Ptr::<payload>::null());
    assert!((((((*h.borrow()).with(|__v| __v.first.clone())).is_null()) as i32) != 0));
    assert!(((((*h.borrow()).with(|__v| __v.count == 1)) as i32) != 0));
    libcc2rs::free_refcount(
        (((*h.borrow()).with(|__v| __v.second.clone())) as Ptr<payload>)
            .to_any()
            .clone(),
    );
    (*h.borrow()).with_mut(|__v| __v.second = Ptr::<payload>::null());
    (*h.borrow()).with_mut(|__v| __v.count = 0);
    assert!(((((*h.borrow()).with(|__v| __v.count == 0)) as i32) != 0));
    assert!((((((*h.borrow()).with(|__v| __v.first.clone())).is_null()) as i32) != 0));
    libcc2rs::free_refcount(((*h.borrow()).clone() as Ptr<holder>).to_any().clone());
    return 0;
}
