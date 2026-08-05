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
pub struct hdr {
    pub n: i32,
    pub name: Box<[u8]>,
}
impl Default for hdr {
    fn default() -> Self {
        hdr {
            n: <i32>::default(),
            name: (0..1).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        }
    }
}
impl ByteRepr for hdr {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.n.to_bytes(&mut buf[0..4]);
        self.name.to_bytes(&mut buf[4..5]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: <i32>::from_bytes(&buf[0..4]),
            name: <Box<[u8]>>::from_bytes(&buf[4..5]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<Ptr<hdr>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, (8usize as usize).wrapping_add(8_usize))
            .reinterpret_cast::<hdr>(),
    ));
    {
        ((((*h.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>) as Ptr<u8>)
            as Ptr<u8>)
            .to_any()
            .memcpy(
                &Ptr::from_string_literal(b"abcdefg").to_any(),
                8_usize as usize,
            );
        ((((*h.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>) as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .clone()
    };
    (*h.borrow()).with_mut(|__v| __v.n = 5);
    assert!(((((*h.borrow()).with(|__v| (*__v).n) == 5) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (((*h.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>)
                as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"abcdefg").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*h.borrow()).clone() as Ptr<hdr>).to_any().clone());
    return 0;
}
