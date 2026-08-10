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
pub struct Wrapper {
    pub items: Vec<Item>,
    pub queue: Vec<Item>,
}
impl ByteRepr for Wrapper {
    fn byte_size() -> usize {
        104
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.items.to_bytes(&mut buf[0..24]);
        self.queue.to_bytes(&mut buf[24..104]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            items: <Vec<Item>>::from_bytes(&buf[0..24]),
            queue: <Vec<Item>>::from_bytes(&buf[24..104]),
        }
    }
}
pub fn count_0(w: Ptr<Wrapper>, q: Ptr<Vec<Item>>) -> i32 {
    let w: Value<Ptr<Wrapper>> = Rc::new(RefCell::new(w));
    let q: Value<Ptr<Vec<Item>>> = Rc::new(RefCell::new(q));
    return {
        let _lhs = {
            let _lhs = ((*w.borrow()).with(|__v| __v.items.len()) as i32);
            _lhs + (if (*w.borrow()).with(|__v| __v.queue.is_empty()) {
                1
            } else {
                0
            })
        };
        _lhs + (if (*q.borrow()).with(|__v| __v.is_empty()) {
            2
        } else {
            0
        })
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let w: Value<Wrapper> = Rc::new(RefCell::new(<Wrapper>::default()));
    assert!(
        (({
            let _w: Ptr<Wrapper> = (w.as_pointer());
            let _q: Ptr<Vec<Item>> = (w.as_pointer().field_ptr(
                24,
                |__v: &Wrapper| ::std::slice::from_ref(&__v.queue),
                |__v: &mut Wrapper| ::std::slice::from_mut(&mut __v.queue),
            ));
            count_0(_w, _q)
        }) == 3)
    );
    return 0;
}
