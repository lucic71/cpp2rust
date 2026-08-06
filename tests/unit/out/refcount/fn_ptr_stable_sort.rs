extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct Item {
    pub key: i32,
    pub value: i32,
}
impl Clone for Item {
    fn clone(&self) -> Self {
        let mut this = Self {
            key: self.key,
            value: self.value,
        };
        this
    }
}
impl ByteRepr for Item {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.key.to_bytes(&mut buf[0..4]);
        self.value.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            key: <i32>::from_bytes(&buf[0..4]),
            value: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn Compare_0(a: Ptr<Item>, b: Ptr<Item>) -> bool {
    return {
        let _lhs = a.with(|__v| (*__v).key);
        _lhs < b.with(|__v| (*__v).key)
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<Item>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(Item { key: 3, value: 30 });
    (*v.borrow_mut()).push(Item { key: 1, value: 10 });
    (*v.borrow_mut()).push(Item { key: 2, value: 20 });
    (v.as_pointer() as Ptr<Item>).sort_with_cmp(
        (v.as_pointer() as Ptr<Item>).to_end().get_offset(),
        *FnPtr::<fn(Ptr<Item>, Ptr<Item>) -> bool>::new(Compare_0),
    );
    assert!(
        ((v.as_pointer() as Ptr<Item>)
            .offset(0_usize)
            .with(|__v| (*__v).key)
            == 1)
    );
    assert!(
        ((v.as_pointer() as Ptr<Item>)
            .offset(1_usize)
            .with(|__v| (*__v).key)
            == 2)
    );
    assert!(
        ((v.as_pointer() as Ptr<Item>)
            .offset(2_usize)
            .with(|__v| (*__v).key)
            == 3)
    );
    return 0;
}
