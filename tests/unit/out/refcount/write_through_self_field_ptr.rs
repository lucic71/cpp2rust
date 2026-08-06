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
pub struct S {
    pub x: i32,
    pub p: Ptr<i32>,
    pub self_: Ptr<S>,
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.p.to_bytes(&mut buf[8..16]);
        self.self_.to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            p: <Ptr<i32>>::from_bytes(&buf[8..16]),
            self_: <Ptr<S>>::from_bytes(&buf[16..24]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = <Value<S>>::default();
    (*s.borrow_mut()).x = 1;
    {
        let __rhs = (s.as_pointer().field_ptr(
            0,
            |__v: &S| ::std::slice::from_ref(&__v.x),
            |__v: &mut S| ::std::slice::from_mut(&mut __v.x),
        ));
        (*s.borrow_mut()).p = __rhs
    };
    {
        let __ptr = (*s.borrow()).p.clone();
        __ptr.write(5)
    };
    assert!(((((*s.borrow()).x == 5) as i32) != 0));
    {
        let __rhs = ((*s.borrow()).x + 1);
        {
            let __ptr = (*s.borrow()).p.clone();
            __ptr.write(__rhs)
        }
    };
    assert!(((((*s.borrow()).x == 6) as i32) != 0));
    {
        let __rhs = (s.as_pointer());
        (*s.borrow_mut()).self_ = __rhs
    };
    {
        let __ptr = (*s.borrow()).self_.clone();
        __ptr.with_mut(|__v| __v.x = 7)
    };
    assert!(((((*s.borrow()).x == 7) as i32) != 0));
    return 0;
}
