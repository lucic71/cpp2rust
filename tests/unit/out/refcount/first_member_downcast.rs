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
pub struct base {
    pub next: Ptr<base>,
}
impl ByteRepr for base {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.next.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            next: <Ptr<base>>::from_bytes(&buf[0..8]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct derived {
    pub head: base,
    pub value: usize,
}
impl ByteRepr for derived {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.head.to_bytes(&mut buf[0..8]);
        self.value.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            head: <base>::from_bytes(&buf[0..8]),
            value: <usize>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let d: Value<Ptr<derived>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(16usize).reinterpret_cast::<derived>(),
    ));
    assert!((((!((*d.borrow()).is_null())) as i32) != 0));
    (*d.borrow()).with_mut(|__v| __v.head.next = Ptr::<base>::null());
    (*d.borrow()).with_mut(|__v| __v.value = 7_usize);
    let b: Value<Ptr<base>> = Rc::new(RefCell::new(
        ((*d.borrow()).field_ptr(
            0,
            |__v: &derived| ::std::slice::from_ref(&__v.head),
            |__v: &mut derived| ::std::slice::from_mut(&mut __v.head),
        )),
    ));
    let back: Value<Ptr<derived>> =
        Rc::new(RefCell::new((*b.borrow()).reinterpret_cast::<derived>()));
    assert!(
        ((({
            let _lhs = (*back.borrow()).clone();
            _lhs == (*d.borrow()).clone()
        }) as i32)
            != 0)
    );
    assert!(((((*back.borrow()).with(|__v| (*__v).value) == 7_usize) as i32) != 0));
    libcc2rs::free_refcount(((*back.borrow()).clone() as Ptr<derived>).to_any().clone());
    return 0;
}
