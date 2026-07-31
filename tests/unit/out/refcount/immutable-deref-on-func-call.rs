extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Item {
    pub value: i32,
}
pub trait ItemMethods {
    fn foo(&self, other: Ptr<Item>);
}
impl ItemMethods for Ptr<Item> {
    fn foo(&self, other: Ptr<Item>) {
        let other: Value<Ptr<Item>> = Rc::new(RefCell::new(other));
        (*other.borrow()).with_mut(|__v| __v.value = 10);
    }
}
impl Clone for Item {
    fn clone(&self) -> Self {
        let mut this = Self { value: self.value };
        this
    }
}
impl ByteRepr for Item {
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
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Ptr<Item>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..2_usize)
            .map(|_| <Item>::default())
            .collect::<Box<[Item]>>(),
    )));
    (*arr.borrow())
        .offset(((0) as isize))
        .with_mut(|__v| __v.value = 1);
    (*arr.borrow())
        .offset(((1) as isize))
        .with_mut(|__v| __v.value = 2);
    ({
        let _other: Ptr<Item> = ((*arr.borrow()).offset(((1) as isize)));
        (*arr.borrow()).offset(((0) as isize)).foo(_other)
    });
    let result: Value<i32> = Rc::new(RefCell::new(
        ((*arr.borrow())
            .offset(((0) as isize))
            .with(|__v| (*__v).value)
            + (*arr.borrow())
                .offset(((1) as isize))
                .with(|__v| (*__v).value)),
    ));
    (*arr.borrow()).delete_array();
    return (*result.borrow());
}
