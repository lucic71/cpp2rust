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
pub struct pair {
    pub a: Ptr<CFile>,
    pub b: Ptr<CFile>,
}
impl ByteRepr for pair {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..8]);
        self.b.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <Ptr<CFile>>::from_bytes(&buf[0..8]),
            b: <Ptr<CFile>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<pair>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, 16usize).reinterpret_cast::<pair>(),
    ));
    (*p.borrow()).with_mut(|__v| {
        __v.a = match CFile::open(
            &Ptr::from_string_literal(b"/dev/null").to_rust_string(),
            &Ptr::from_string_literal(b"w").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        }
    });
    (*p.borrow()).with_mut(|__v| {
        __v.b = match CFile::open(
            &Ptr::from_string_literal(b"/dev/null").to_rust_string(),
            &Ptr::from_string_literal(b"w").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        }
    });
    assert!(
        ((({
            let _lhs = ((*p.borrow()).with(|__v| (*__v).a.clone())).clone();
            _lhs != ((*p.borrow()).with(|__v| (*__v).b.clone())).clone()
        }) as i32)
            != 0)
    );
    libcc2rs::fclose_refcount((*p.borrow()).with(|__v| (*__v).a.clone()).clone());
    libcc2rs::fclose_refcount((*p.borrow()).with(|__v| (*__v).b.clone()).clone());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 64) as i32) != 0) {
        let q: Value<Ptr<u8>> = Rc::new(RefCell::new(
            libcc2rs::malloc_refcount(16_usize).reinterpret_cast::<u8>(),
        ));
        {
            let __rhs = ((*i.borrow()) as u8);
            (*q.borrow()).offset(((0) as isize)).write(__rhs)
        };
        libcc2rs::free_refcount(((*q.borrow()).clone() as Ptr<u8>).to_any().clone());
        (*i.borrow_mut()).postfix_inc();
    }
    assert!(
        ((({
            let _lhs = ((*p.borrow()).with(|__v| (*__v).a.clone())).clone();
            _lhs != ((*p.borrow()).with(|__v| (*__v).b.clone())).clone()
        }) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*p.borrow()).clone() as Ptr<pair>).to_any().clone());
    return 0;
}
