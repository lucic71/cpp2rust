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
    pub hi: u32,
    pub fill: Box<[u8]>,
}
impl Default for shape_b {
    fn default() -> Self {
        shape_b {
            code: <u16>::default(),
            lo: <u16>::default(),
            hi: <u32>::default(),
            fill: (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        }
    }
}
impl ByteRepr for shape_b {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.code.to_bytes(&mut buf[0..2]);
        self.lo.to_bytes(&mut buf[2..4]);
        self.hi.to_bytes(&mut buf[4..8]);
        self.fill.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            code: <u16>::from_bytes(&buf[0..2]),
            lo: <u16>::from_bytes(&buf[2..4]),
            hi: <u32>::from_bytes(&buf[4..8]),
            fill: <Box<[u8]>>::from_bytes(&buf[8..16]),
        }
    }
}
#[repr(C, align(4))]
#[derive(Clone, Default)]
#[bitfields(__bits_0 { f1: u32 @ 0..1 unsigned, f2: u32 @ 1..4 unsigned, f3: u32 @ 4..16 unsigned })]
pub struct shape_c {
    pub code: u16,
    pub __bits_0: [u8; 2],
}
impl ByteRepr for shape_c {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf[2..4].copy_from_slice(&self.__bits_0);
        self.code.to_bytes(&mut buf[0..2]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            __bits_0: buf[2..4].try_into().unwrap(),
            code: <u16>::from_bytes(&buf[0..2]),
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
    pub fn c(&self) -> Ptr<shape_c> {
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
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 256]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        256
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
    pub view: anon_0,
}
impl ByteRepr for Container {
    fn byte_size() -> usize {
        256
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.view.to_bytes(&mut buf[0..256]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            view: <anon_0>::from_bytes(&buf[0..256]),
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
            .memset((0) as u8, 256usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    assert!(
        (((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_a>() as Ptr<shape_a>)
            .with(|__v| (*__v).code) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_b>() as Ptr<shape_b>)
            .with(|__v| (*__v).lo) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((((((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((0) as isize))
            .read()) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((((((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((255) as isize))
            .read()) as i32)
            == 0) as i32)
            != 0)
    );
    let src: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    (*src.borrow_mut())[(0) as usize] = 2_u8;
    (*src.borrow_mut())[(2) as usize] = 80_u8;
    (*src.borrow_mut())[(3) as usize] = 0_u8;
    (*src.borrow_mut())[(4) as usize] = 127_u8;
    (*src.borrow_mut())[(5) as usize] = 0_u8;
    (*src.borrow_mut())[(6) as usize] = 0_u8;
    (*src.borrow_mut())[(7) as usize] = 1_u8;
    let len: Value<usize> = Rc::new(RefCell::new(16_usize));
    assert!(((((*len.borrow()) <= 256usize) as i32) != 0));
    {
        (((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
            as Ptr<u8>)
            .to_any()
            .memcpy(
                &((src.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                (*len.borrow()) as usize,
            );
        (((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
            as Ptr<u8>)
            .to_any()
            .clone()
    };
    assert!(
        (((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_b>() as Ptr<shape_b>)
            .with(|__v| (*__v).code) as i32)
            == 2) as i32)
            != 0)
    );
    assert!(
        ((((((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_b>() as Ptr<shape_b>)
            .field_ptr(
                2,
                |__v: &shape_b| ::std::slice::from_ref(&__v.lo),
                |__v: &mut shape_b| ::std::slice::from_mut(&mut __v.lo)
            ))
        .reinterpret_cast::<u8>())
        .offset(((0) as isize))
        .read()) as i32)
            == 80) as i32)
            != 0)
    );
    {
        ((c.as_pointer()) as Ptr<Container>)
            .to_any()
            .memset((0) as u8, 256usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    assert!(
        (((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_b>() as Ptr<shape_b>)
            .with(|__v| (*__v).code) as i32)
            == 0) as i32)
            != 0)
    );
    assert!((((4usize == 4_usize) as i32) != 0));
    assert!(
        (((((((((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
            .with(|__v| (*__v).f1()) as i32)
            == 0) as i32)
            != 0)
            && (((((c
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
                .with(|__v| (*__v).f2()) as i32)
                == 0) as i32)
                != 0)) as i32)
            != 0)
            && (((((c
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
                .with(|__v| (*__v).f3()) as i32)
                == 0) as i32)
                != 0)) as i32)
            != 0)
    );
    (c.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
        .with_mut(|__v| __v.code = 2_u16);
    (c.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
        .with_mut(|__v| __v.set_f1(1_u32));
    (c.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
        .with_mut(|__v| __v.set_f2(5_u32));
    (c.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
        .with_mut(|__v| __v.set_f3(2748_u32));
    assert!(
        ((((((((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
            .reinterpret_cast::<u8>())
        .offset(((2) as isize))
        .read()) as i32)
            == 203) as i32)
            != 0)
    );
    assert!(
        ((((((((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
            .reinterpret_cast::<u8>())
        .offset(((3) as isize))
        .read()) as i32)
            == 171) as i32)
            != 0)
    );
    {
        ((((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
            .offset(((2) as isize))) as Ptr<u8>)
            .to_any()
            .memset((255) as u8, 2_usize as usize);
        ((((c.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
            .offset(((2) as isize))) as Ptr<u8>)
            .to_any()
            .clone()
    };
    assert!(
        (((((((((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
            .with(|__v| (*__v).f1()) as i32)
            == 1) as i32)
            != 0)
            && (((((c
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
                .with(|__v| (*__v).f2()) as i32)
                == 7) as i32)
                != 0)) as i32)
            != 0)
            && (((((c
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
                .with(|__v| (*__v).f3()) as i32)
                == 4095) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        (((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
            .with(|__v| (*__v).code) as i32)
            == 2) as i32)
            != 0)
    );
    {
        ((c.as_pointer()) as Ptr<Container>)
            .to_any()
            .memset((0) as u8, 256usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    assert!(
        (((((((((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
            .with(|__v| (*__v).f1()) as i32)
            == 0) as i32)
            != 0)
            && (((((c
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
                .with(|__v| (*__v).f2()) as i32)
                == 0) as i32)
                != 0)) as i32)
            != 0)
            && (((((c
                .as_pointer()
                .reinterpret_cast::<u8>()
                .offset(0usize)
                .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
                .with(|__v| (*__v).f3()) as i32)
                == 0) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        (((((c
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<shape_c>() as Ptr<shape_c>)
            .with(|__v| (*__v).code) as i32)
            == 0) as i32)
            != 0)
    );
    return 0;
}
