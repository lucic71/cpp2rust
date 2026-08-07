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
    pub fn p(&self) -> Ptr<Ptr<u8>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn n(&self) -> Ptr<i32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn c(&self) -> Ptr<u8> {
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
pub struct item_struct {
    pub tag: i32,
    pub u: anon_0,
}
impl ByteRepr for item_struct {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.tag.to_bytes(&mut buf[0..4]);
        self.u.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: <i32>::from_bytes(&buf[0..4]),
            u: <anon_0>::from_bytes(&buf[8..16]),
        }
    }
}
thread_local!(
    pub static items_1: Value<Box<[item_struct]>> = Rc::new(RefCell::new(Box::new([
        item_struct {
            tag: 0,
            u: {
                let __u: anon_0 = Default::default();
                __u.p().write(Ptr::from_string_literal(b"xy\0"));
                __u
            },
        },
        item_struct {
            tag: 1,
            u: anon_0 {
                __bytes: Rc::new(RefCell::new(Box::from([42, 0, 0, 0, 0, 0, 0, 0]))),
            },
        },
        item_struct {
            tag: 2,
            u: anon_0 {
                __bytes: Rc::new(RefCell::new(Box::from([97, 98, 0, 0, 0, 0, 0, 0]))),
            },
        },
    ])));
);
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let it: Value<Ptr<item_struct>> = Rc::new(RefCell::new(
        ((items_1.with(Value::clone).as_pointer() as Ptr<item_struct>).offset(0)),
    ));
    assert!(
        ((({
            let mut __it1 = (((*it.borrow())
                .reinterpret_cast::<u8>()
                .offset(8usize)
                .reinterpret_cast::<Ptr<u8>>() as Ptr<Ptr<u8>>)
                .read())
            .to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"xy\0").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((((((items_1.with(Value::clone).as_pointer() as Ptr<item_struct>)
            .offset(1)
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<i32>() as Ptr<i32>)
            .read())
            == 42) as i32)
            != 0)
    );
    assert!(
        ((((((((items_1.with(Value::clone).as_pointer() as Ptr<item_struct>)
            .offset(2)
            .reinterpret_cast::<u8>()
            .offset(8usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((1) as isize))
            .read()) as i32)
            == ('b' as i32)) as i32)
            != 0)
    );
    return 0;
}
