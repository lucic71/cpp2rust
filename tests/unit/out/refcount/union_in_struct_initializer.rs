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
    pub fn next(&self) -> Ptr<Ptr<Item>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn tag(&self) -> Ptr<i64> {
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
#[derive(Clone)]
pub struct Item {
    pub kind: i32,
    pub handler: FnPtr<fn(i32) -> i32>,
    pub u: anon_0,
}
impl Default for Item {
    fn default() -> Self {
        Item {
            kind: <i32>::default(),
            handler: FnPtr::<fn(i32) -> i32>::null(),
            u: <anon_0>::default(),
        }
    }
}
impl ByteRepr for Item {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.kind.to_bytes(&mut buf[0..4]);
        self.handler.to_bytes(&mut buf[8..16]);
        self.u.to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: <i32>::from_bytes(&buf[0..4]),
            handler: <FnPtr<fn(i32) -> i32>>::from_bytes(&buf[8..16]),
            u: <anon_0>::from_bytes(&buf[16..24]),
        }
    }
}
pub fn double_it_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn negate_2(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return -(*x.borrow());
}
thread_local!(
    pub static items_3: Value<Box<[Item]>> = Rc::new(RefCell::new(Box::new([
        Item {
            kind: 1,
            handler: FnPtr::<fn(i32) -> i32>::new(double_it_1),
            u: Default::default(),
        },
        Item {
            kind: 2,
            handler: FnPtr::<fn(i32) -> i32>::new(negate_2),
            u: Default::default(),
        },
    ])));
);
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({ (*(*items_3.with(Value::clone).borrow())[(0) as usize].handler)(21) }) == 42)
            as i32)
            != 0)
    );
    assert!(
        (((({ (*(*items_3.with(Value::clone).borrow())[(1) as usize].handler)(21) }) == -21_i32)
            as i32)
            != 0)
    );
    assert!(
        ((((((items_3.with(Value::clone).as_pointer() as Ptr<Item>)
            .offset(0)
            .reinterpret_cast::<u8>()
            .offset(16usize)
            .reinterpret_cast::<Ptr<Item>>() as Ptr<Ptr<Item>>)
            .read())
        .is_null()) as i32)
            != 0)
    );
    ((items_3.with(Value::clone).as_pointer() as Ptr<Item>)
        .offset(0)
        .reinterpret_cast::<u8>()
        .offset(16usize)
        .reinterpret_cast::<i64>() as Ptr<i64>)
        .write(7_i64);
    assert!(
        ((((((items_3.with(Value::clone).as_pointer() as Ptr<Item>)
            .offset(0)
            .reinterpret_cast::<u8>()
            .offset(16usize)
            .reinterpret_cast::<i64>() as Ptr<i64>)
            .read())
            == 7_i64) as i32)
            != 0)
    );
    return 0;
}
