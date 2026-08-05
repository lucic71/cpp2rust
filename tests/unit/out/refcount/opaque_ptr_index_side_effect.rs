extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Clone)]
pub struct bin {
    pub idx: i32,
    pub buf: Box<[u8]>,
}
impl Default for bin {
    fn default() -> Self {
        bin {
            idx: <i32>::default(),
            buf: (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        }
    }
}
impl ByteRepr for bin {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.idx.to_bytes(&mut buf[0..4]);
        self.buf.to_bytes(&mut buf[4..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            idx: <i32>::from_bytes(&buf[0..4]),
            buf: <Box<[u8]>>::from_bytes(&buf[4..12]),
        }
    }
}
pub fn store_0(p: AnyPtr, c: u8) {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    let c: Value<u8> = Rc::new(RefCell::new(c));
    let b: Value<Ptr<bin>> = Rc::new(RefCell::new((*p.borrow()).reinterpret_cast::<bin>()));
    (*b.borrow()).with_mut(|__v| __v.buf[(__v.idx.postfix_inc()) as usize] = (*c.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let b: Value<Ptr<bin>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, 12usize).reinterpret_cast::<bin>(),
    ));
    ({
        store_0(
            ((*b.borrow()).clone() as Ptr<bin>).to_any(),
            (('a' as i32) as u8),
        )
    });
    ({
        store_0(
            ((*b.borrow()).clone() as Ptr<bin>).to_any(),
            (('b' as i32) as u8),
        )
    });
    assert!(((((*b.borrow()).with(|__v| (*__v).idx) == 2) as i32) != 0));
    assert!(
        (((((*b.borrow()).with(|__v| (*__v).buf[(0) as usize]) as i32) == ('a' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*b.borrow()).with(|__v| (*__v).buf[(1) as usize]) as i32) == ('b' as i32)) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*b.borrow()).clone() as Ptr<bin>).to_any().clone());
    return 0;
}
