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
pub struct shape_a {
    pub code: u16,
    pub pad: Box<[u8]>,
}
impl Default for shape_a {
    fn default() -> Self {
        shape_a {
            code: <u16>::default(),
            pad: (0..14).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        }
    }
}
impl ByteRepr for shape_a {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.code.to_bytes(&mut buf[0..2]);
        self.pad.to_bytes(&mut buf[2..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            code: <u16>::from_bytes(&buf[0..2]),
            pad: <Box<[u8]>>::from_bytes(&buf[2..16]),
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct shape_b {
    pub code: u16,
    pub lo: u16,
    pub mid: u32,
    pub fill: Box<[u8]>,
    pub tail: u32,
}
impl Default for shape_b {
    fn default() -> Self {
        shape_b {
            code: <u16>::default(),
            lo: <u16>::default(),
            mid: <u32>::default(),
            fill: (0..16).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            tail: <u32>::default(),
        }
    }
}
impl ByteRepr for shape_b {
    fn byte_size() -> usize {
        28
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.code.to_bytes(&mut buf[0..2]);
        self.lo.to_bytes(&mut buf[2..4]);
        self.mid.to_bytes(&mut buf[4..8]);
        self.fill.to_bytes(&mut buf[8..24]);
        self.tail.to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            code: <u16>::from_bytes(&buf[0..2]),
            lo: <u16>::from_bytes(&buf[2..4]),
            mid: <u32>::from_bytes(&buf[4..8]),
            fill: <Box<[u8]>>::from_bytes(&buf[8..24]),
            tail: <u32>::from_bytes(&buf[24..28]),
        }
    }
}
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn a(&self) -> Ptr<shape_a> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn b(&self) -> Ptr<shape_b> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn raw_(&self) -> Ptr<u8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 64]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        64
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Container {
    pub len: u32,
    pub u: anon_0,
}
impl ByteRepr for Container {
    fn byte_size() -> usize {
        68
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.len.to_bytes(&mut buf[0..4]);
        self.u.to_bytes(&mut buf[4..68]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            len: <u32>::from_bytes(&buf[0..4]),
            u: <anon_0>::from_bytes(&buf[4..68]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let c: Value<Container> = <Value<Container>>::default();
    {
        ((c.as_pointer()) as Ptr<Container>)
            .to_any()
            .memset((0) as u8, 68usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    (c.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(4usize)
        .reinterpret_cast::<shape_a>() as Ptr<shape_a>)
        .with_mut(|__v| __v.code = 10_u16);
    (*c.borrow_mut()).len = (28usize as u32);
    ((c.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(4usize)
        .reinterpret_cast::<shape_a>() as Ptr<shape_a>)
        .to_any()
        .reinterpret_cast::<shape_b>())
    .with_mut(|__v| __v.tail = 3735928559_u32);
    assert!(
        ((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(4usize)
            .reinterpret_cast::<shape_b>() as Ptr<shape_b>)
            .with(|__v| (*__v).tail)
            == 3735928559_u32) as i32)
            != 0)
    );
    assert!(
        (((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(4usize)
            .reinterpret_cast::<shape_b>() as Ptr<shape_b>)
            .with(|__v| (*__v).code) as i32)
            == 10) as i32)
            != 0)
    );
    (c.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(4usize)
        .reinterpret_cast::<shape_b>() as Ptr<shape_b>)
        .with_mut(|__v| __v.lo = 8080_u16);
    assert!(
        ((((((((c.as_pointer().reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>) as Ptr<u8>)
            .reinterpret_cast::<u8>())
        .offset(((2) as isize))
        .read()) as i32)
            == 144) as i32)
            != 0)
    );
    assert!(
        ((((((((c.as_pointer().reinterpret_cast::<u8>().offset(4usize) as Ptr<u8>) as Ptr<u8>)
            .reinterpret_cast::<u8>())
        .offset(((3) as isize))
        .read()) as i32)
            == 31) as i32)
            != 0)
    );
    return 0;
}
