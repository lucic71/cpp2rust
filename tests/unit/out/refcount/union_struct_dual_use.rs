extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Inner {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn sum_inner_0(i: Ptr<Inner>) -> i32 {
    let i: Value<Ptr<Inner>> = Rc::new(RefCell::new(i));
    return {
        let _lhs = (*(*(*i.borrow()).upgrade().deref()).a.borrow());
        _lhs + (*(*(*i.borrow()).upgrade().deref()).b.borrow())
    };
}
#[derive(Clone)]
pub struct anon_1 {
    __store: libcc2rs::UnionStorage,
}
impl anon_1 {
    pub fn inner(&self) -> Ptr<Inner> {
        self.__store.reinterpret(0)
    }
    pub fn raw_(&self) -> Ptr<Box<[u8]>> {
        self.__store.reinterpret(0)
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __store: libcc2rs::UnionStorage::new(16),
        }
    }
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.__store.to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_1 {
            __store: libcc2rs::UnionStorage::from_bytes(buf),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub u: Value<anon_1>,
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.u.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            u: Rc::new(RefCell::new(<anon_1>::from_bytes(&buf[0..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let standalone: Value<Inner> = <Value<Inner>>::default();
    (*(*standalone.borrow()).a.borrow_mut()) = 3;
    (*(*standalone.borrow()).b.borrow_mut()) = 4;
    assert!(
        (((({
            let _i: Ptr<Inner> = (standalone.as_pointer());
            sum_inner_0(_i)
        }) == 7) as i32)
            != 0)
    );
    let outer: Value<Outer> = <Value<Outer>>::default();
    {
        ((outer.as_pointer()) as Ptr<Outer>)
            .to_any()
            .memset((0) as u8, 16usize as usize);
        ((outer.as_pointer()) as Ptr<Outer>).to_any().clone()
    };
    (*(*(*(*outer.borrow()).u.borrow()).inner().upgrade().deref())
        .a
        .borrow_mut()) = 3;
    (*(*(*(*outer.borrow()).u.borrow()).inner().upgrade().deref())
        .b
        .borrow_mut()) = 4;
    assert!(
        (((({
            let _i: Ptr<Inner> = ((*(*outer.borrow()).u.borrow()).inner()).clone();
            sum_inner_0(_i)
        }) == 7) as i32)
            != 0)
    );
    assert!(
        (((((((*(*outer.borrow()).u.borrow()).raw_().read())[(0) as usize] as u8) as i32) == 3)
            as i32)
            != 0)
    );
    assert!(
        (((((((*(*outer.borrow()).u.borrow()).raw_().read())[(4) as usize] as u8) as i32) == 4)
            as i32)
            != 0)
    );
    return 0;
}
