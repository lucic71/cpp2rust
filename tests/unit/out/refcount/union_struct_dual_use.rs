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
pub struct Inner {
    pub a: i32,
    pub b: i32,
}
impl ByteRepr for Inner {
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
pub fn sum_inner_0(i: Ptr<Inner>) -> i32 {
    let i: Value<Ptr<Inner>> = Rc::new(RefCell::new(i));
    return {
        let _lhs = (*i.borrow()).with(|__v| __v.a);
        (*i.borrow()).with(|__v| _lhs + __v.b)
    };
}
pub struct anon_1 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_1 {
    pub fn inner(&self) -> Ptr<Inner> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn raw_(&self) -> Ptr<u8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_1 {
    fn clone(&self) -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 16]))),
        }
    }
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Outer {
    pub u: anon_1,
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.u.to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            u: <anon_1>::from_bytes(&buf[0..16]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let standalone: Value<Inner> = <Value<Inner>>::default();
    (*standalone.borrow_mut()).a = 3;
    (*standalone.borrow_mut()).b = 4;
    assert!((((({ sum_inner_0((standalone.as_pointer())) }) == 7) as i32) != 0));
    let outer: Value<Outer> = <Value<Outer>>::default();
    {
        ((outer.as_pointer()) as Ptr<Outer>)
            .to_any()
            .memset((0) as u8, 16usize as usize);
        ((outer.as_pointer()) as Ptr<Outer>).to_any().clone()
    };
    (outer
        .as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<Inner>() as Ptr<Inner>)
        .with_mut(|__v| __v.a = 3);
    (outer
        .as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<Inner>() as Ptr<Inner>)
        .with_mut(|__v| __v.b = 4);
    assert!(
        (((({
            sum_inner_0(
                (outer
                    .as_pointer()
                    .reinterpret_cast::<u8>()
                    .offset(0usize)
                    .reinterpret_cast::<Inner>() as Ptr<Inner>),
            )
        }) == 7) as i32)
            != 0)
    );
    assert!(
        ((((((((outer.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>)
            as Ptr::<u8>)
            .offset(((0) as isize))
            .read()) as u8) as i32)
            == 3) as i32)
            != 0)
    );
    assert!(
        ((((((((outer.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>)
            as Ptr::<u8>)
            .offset(((4) as isize))
            .read()) as u8) as i32)
            == 4) as i32)
            != 0)
    );
    return 0;
}
