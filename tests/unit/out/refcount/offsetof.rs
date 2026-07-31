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
pub struct Layout {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}
impl Clone for Layout {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: self.a,
            b: self.b,
            c: self.c,
        };
        this
    }
}
impl ByteRepr for Layout {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..1]);
        self.b.to_bytes(&mut buf[4..8]);
        self.c.to_bytes(&mut buf[8..10]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <u8>::from_bytes(&buf[0..1]),
            b: <u32>::from_bytes(&buf[4..8]),
            c: <u16>::from_bytes(&buf[8..10]),
        }
    }
}
#[repr(C)]
#[derive()]
pub struct Frame {
    pub tag: u16,
    pub body: Box<[u8]>,
}
impl Clone for Frame {
    fn clone(&self) -> Self {
        let mut this = Self {
            tag: self.tag,
            body: (self.body).clone(),
        };
        this
    }
}
impl Default for Frame {
    fn default() -> Self {
        Frame {
            tag: <u16>::default(),
            body: (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        }
    }
}
impl ByteRepr for Frame {
    fn byte_size() -> usize {
        66
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.tag.to_bytes(&mut buf[0..2]);
        self.body.to_bytes(&mut buf[2..66]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: <u16>::from_bytes(&buf[0..2]),
            body: <Box<[u8]>>::from_bytes(&buf[2..66]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((0_usize == 0_usize));
    assert!((4_usize == 4_usize));
    assert!((8_usize == 8_usize));
    let v: Value<Layout> = Rc::new(RefCell::new(Layout {
        a: 0_u8,
        b: <u32>::default(),
        c: <u16>::default(),
    }));
    (*v.borrow_mut()).b = 3735928559_u32;
    let base: Value<Ptr<u8>> = Rc::new(RefCell::new((v.as_pointer()).reinterpret_cast::<u8>()));
    let bp: Value<Ptr<u32>> = Rc::new(RefCell::new(
        ((*base.borrow()).offset(((4_usize) as isize))).reinterpret_cast::<u32>(),
    ));
    assert!((((*bp.borrow()).read()) == 3735928559_u32));
    ((*base.borrow()).offset(((4_usize) as isize)))
        .reinterpret_cast::<u32>()
        .write(305419896_u32);
    assert!(((*v.borrow()).b == 305419896_u32));
    let text: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"example-body")));
    let len: Value<usize> = Rc::new(RefCell::new(
        ((*text.borrow()).to_c_string_iterator().count()).wrapping_add(1_usize),
    ));
    let total: Value<usize> = Rc::new(RefCell::new(
        ((2_usize as u64).wrapping_add(((*len.borrow()) as u64)) as usize),
    ));
    assert!(((*total.borrow()) == (2_usize).wrapping_add((*len.borrow()))));
    return 0;
}
