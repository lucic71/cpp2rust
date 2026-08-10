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
    pub value: Option<Value<i32>>,
}
impl ByteRepr for Item {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.value.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: <Option<Value<i32>>>::from_bytes(&buf[0..8]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct Holder {
    pub items: Vec<Item>,
    pub pending: Item,
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        88
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.items.to_bytes(&mut buf[0..80]);
        self.pending.to_bytes(&mut buf[80..88]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            items: <Vec<Item>>::from_bytes(&buf[0..80]),
            pending: <Item>::from_bytes(&buf[80..88]),
        }
    }
}
pub fn store_0(h: Ptr<Holder>) {
    let h: Value<Ptr<Holder>> = Rc::new(RefCell::new(h));
    {
        let __val = (*h.borrow()).with_mut(|__v| std::mem::take(&mut __v.pending));
        (*h.borrow())
            .field_ptr(
                0,
                |__v: &Holder| ::std::slice::from_ref(&__v.items),
                |__v: &mut Holder| ::std::slice::from_mut(&mut __v.items),
            )
            .with_mut(|__v: &mut Vec<Item>| __v.push(__val))
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    {
        let _p: Ptr<_> = Ptr::alloc(7);
        (*h.borrow_mut()).pending.value = _p.to_owned_opt()
    };
    ({ store_0((h.as_pointer())) });
    assert!(((*h.borrow()).pending.value.as_pointer()).is_null());
    assert!(
        ((*(h.as_pointer().field_ptr(
            0,
            |__v: &Holder| &__v.items[..],
            |__v: &mut Holder| &mut __v.items[..]
        ) as Ptr<Item>)
            .with(|__v| __v.value.clone())
            .as_ref()
            .unwrap()
            .borrow())
            == 7)
    );
    return 0;
}
