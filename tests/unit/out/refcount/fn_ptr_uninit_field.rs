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
pub struct Handler {
    pub tag: i32,
    pub fn_: FnPtr<fn()>,
}
impl Default for Handler {
    fn default() -> Self {
        Handler {
            tag: <i32>::default(),
            fn_: FnPtr::<fn()>::null(),
        }
    }
}
impl ByteRepr for Handler {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.tag.to_bytes(&mut buf[0..4]);
        self.fn_.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: <i32>::from_bytes(&buf[0..4]),
            fn_: <FnPtr<fn()>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let data: Value<i32> = Rc::new(RefCell::new(42));
    let p: Value<AnyPtr> = Rc::new(RefCell::new(((data.as_pointer()) as Ptr<i32>).to_any()));
    let a: Value<Handler> = <Value<Handler>>::default();
    {
        ((a.as_pointer()) as Ptr<Handler>)
            .to_any()
            .memset((0) as u8, 16usize as usize);
        ((a.as_pointer()) as Ptr<Handler>).to_any().clone()
    };
    {
        ((a.as_pointer().field_ptr(
            8,
            |__v: &Handler| ::std::slice::from_ref(&__v.fn_),
            |__v: &mut Handler| ::std::slice::from_mut(&mut __v.fn_),
        )) as Ptr<FnPtr<fn()>>)
            .to_any()
            .memcpy(&((p.as_pointer()) as Ptr<AnyPtr>).to_any(), 8usize as usize);
        ((a.as_pointer().field_ptr(
            8,
            |__v: &Handler| ::std::slice::from_ref(&__v.fn_),
            |__v: &mut Handler| ::std::slice::from_mut(&mut __v.fn_),
        )) as Ptr<FnPtr<fn()>>)
            .to_any()
            .clone()
    };
    let b: Value<Handler> = <Value<Handler>>::default();
    {
        ((b.as_pointer()) as Ptr<Handler>).to_any().memcpy(
            &((a.as_pointer()) as Ptr<Handler>).to_any(),
            16usize as usize,
        );
        ((b.as_pointer()) as Ptr<Handler>).to_any().clone()
    };
    assert!(((((*b.borrow()).tag == 0) as i32) != 0));
    assert!((((!(((*b.borrow()).fn_).is_null())) as i32) != 0));
    assert!(((((*data.borrow()) == 42) as i32) != 0));
    return 0;
}
