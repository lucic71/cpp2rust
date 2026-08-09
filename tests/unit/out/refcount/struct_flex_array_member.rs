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
pub struct peer {
    pub port: i32,
    pub hostname: Box<[u8]>,
}
impl Default for peer {
    fn default() -> Self {
        peer {
            port: <i32>::default(),
            hostname: (0..1).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        }
    }
}
impl ByteRepr for peer {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.port.to_bytes(&mut buf[0..4]);
        self.hostname.to_bytes(&mut buf[4..5]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            port: <i32>::from_bytes(&buf[0..4]),
            hostname: <Box<[u8]>>::from_bytes(&buf[4..5]),
        }
    }
}
pub fn peer_create_0(host: Ptr<u8>) -> Ptr<peer> {
    let host: Value<Ptr<u8>> = Rc::new(RefCell::new(host));
    let p: Value<Ptr<peer>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(
            (8usize as usize).wrapping_add((*host.borrow()).to_c_string_iterator().count()),
        )
        .reinterpret_cast::<peer>(),
    ));
    (*p.borrow()).with_mut(|__v| __v.port = 443);
    {
        ((((*p.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>) as Ptr<u8>)
            as Ptr<u8>)
            .to_any()
            .memcpy(
                &((*host.borrow()).clone() as Ptr<u8>).to_any(),
                ((*host.borrow()).to_c_string_iterator().count()).wrapping_add(1_usize) as usize,
            );
        ((((*p.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>) as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .clone()
    };
    return (*p.borrow()).clone();
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<peer>> = Rc::new(RefCell::new(
        ({ peer_create_0(Ptr::from_string_literal(b"example.com\0")) }),
    ));
    assert!(((((*p.borrow()).with(|__v| __v.port == 443)) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (((*p.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>)
                as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"example.com\0").to_c_string_iterator();
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
    assert!(
        (((((((*p.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>)
            .offset(((0) as isize))
            .read()) as i32)
            == ('e' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((((*p.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>)
            .offset(((7) as isize))
            .read()) as i32)
            == ('.' as i32)) as i32)
            != 0)
    );
    ((*p.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>)
        .offset(((0) as isize))
        .write((('E' as i32) as u8));
    assert!(
        ((({
            let mut __it1 = (((*p.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>)
                as Ptr<u8>)
                .to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"Example.com\0").to_c_string_iterator();
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
    assert!(((((*p.borrow()).with(|__v| __v.port == 443)) as i32) != 0));
    let h: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (((*p.borrow()).reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>).offset(((8) as isize))),
    ));
    assert!(
        ((({
            let mut __it1 = (*h.borrow()).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"com\0").to_c_string_iterator();
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
    libcc2rs::free_refcount(((*p.borrow()).clone() as Ptr<peer>).to_any().clone());
    return 0;
}
