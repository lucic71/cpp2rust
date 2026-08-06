extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn i(&self) -> Ptr<i32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn z(&self) -> Ptr<Ptr<u8>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn pBig(&self) -> Ptr<Ptr<i64>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Rec {
    pub kind: i32,
    pub u: anon_0,
}
impl ByteRepr for Rec {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.kind.to_bytes(&mut buf[0..4]);
        self.u.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: <i32>::from_bytes(&buf[0..4]),
            u: <anon_0>::from_bytes(&buf[8..16]),
        }
    }
}
thread_local!(
    pub static zeroRec_1: Value<Rec> = Rc::new(RefCell::new(Rec {
        kind: 0,
        u: <anon_0>::default(),
    }));
);
pub fn get_rec_2() -> Ptr<Rec> {
    thread_local!(
        static dummy_3: Value<Rec> = <Value<Rec>>::default();
    );
    return (dummy_3.with(Value::clone).as_pointer());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((((*zeroRec_1.with(Value::clone).borrow()).kind == 0) as i32) != 0));
    assert!(
        (((((zeroRec_1
            .with(Value::clone)
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<Ptr::<u8>>() as Ptr<Ptr::<u8>>)
            .read())
        .is_null()) as i32)
            != 0)
    );
    let p: Value<Ptr<Rec>> = Rc::new(RefCell::new(({ get_rec_2() })));
    assert!(
        ((((((*p.borrow())
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<Ptr::<i64>>() as Ptr<Ptr::<i64>>)
            .read())
        .is_null()) as i32)
            != 0)
    );
    ((*p.borrow())
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<i32>() as Ptr<i32>)
        .write(5);
    assert!(
        ((((((*p.borrow())
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<i32>() as Ptr<i32>)
            .read())
            == 5) as i32)
            != 0)
    );
    let r: Value<Rec> = <Value<Rec>>::default();
    (*r.borrow_mut()).kind = 3;
    (r.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<i32>() as Ptr<i32>)
        .write(9);
    (*r.borrow_mut()) = (*zeroRec_1.with(Value::clone).borrow()).clone();
    assert!(((((*r.borrow()).kind == 0) as i32) != 0));
    assert!(
        (((((r
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<i32>() as Ptr<i32>)
            .read())
            == 0) as i32)
            != 0)
    );
    return 0;
}
