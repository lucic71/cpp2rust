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
pub struct item {
    pub value: i32,
}
impl ByteRepr for item {
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
pub fn read_item_0(it: Ptr<item>) -> i32 {
    let it: Value<Ptr<item>> = Rc::new(RefCell::new(it));
    return ((*it.borrow()).with(|__v| (*__v).value) + 1);
}
#[repr(C)]
#[derive(Clone)]
pub struct holder {
    pub callback: FnPtr<fn(AnyPtr) -> i32>,
}
impl Default for holder {
    fn default() -> Self {
        holder {
            callback: FnPtr::<fn(AnyPtr) -> i32>::null(),
        }
    }
}
impl ByteRepr for holder {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.callback.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            callback: <FnPtr<fn(AnyPtr) -> i32>>::from_bytes(&buf[0..8]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<Ptr<holder>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, 8usize).reinterpret_cast::<holder>(),
    ));
    (*h.borrow()).with_mut(|__v| {
        __v.callback =
            FnPtr::<fn(Ptr<item>) -> i32>::new(read_item_0).cast::<fn(AnyPtr) -> i32>(Some(
                (|a0: AnyPtr| -> i32 { read_item_0(a0.reinterpret_cast::<item>()) })
                    as fn(AnyPtr) -> i32,
            ))
    });
    let it: Value<item> = <Value<item>>::default();
    (*it.borrow_mut()).value = 41;
    assert!(
        (((({
            (*(*h.borrow()).with(|__v| (*__v).callback.clone()))(
                ((it.as_pointer()) as Ptr<item>).to_any(),
            )
        }) == 42) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*h.borrow()).clone() as Ptr<holder>).to_any());
    return 0;
}
