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
pub struct Ctx {
    pub mark: Ptr<u8>,
}
impl ByteRepr for Ctx {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.mark.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mark: <Ptr<u8>>::from_bytes(&buf[0..8]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    thread_local!(
        static text_0: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hello world\0")));
    );
    let c: Value<Ctx> = <Value<Ctx>>::default();
    (*c.borrow_mut()).mark = ((text_0.with(Value::clone).as_pointer() as Ptr<u8>).offset(0));
    let tmp: Value<Ptr<u8>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(8_usize).reinterpret_cast::<u8>(),
    ));
    {
        ((*tmp.borrow()).clone() as Ptr<u8>).to_any().memcpy(
            &Ptr::from_string_literal(b"abcdefg\0").to_any(),
            8_usize as usize,
        );
        ((*tmp.borrow()).clone() as Ptr<u8>).to_any().clone()
    };
    (*c.borrow_mut()).mark = (*tmp.borrow()).offset(((2) as isize));
    libcc2rs::free_refcount(((*tmp.borrow()).clone() as Ptr<u8>).to_any().clone());
    return (((((text_0.with(Value::clone).as_pointer() as Ptr<u8>).offset(6))
        - ((*c.borrow()).mark).clone()) as i64) as i32);
}
