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
pub struct record {
    pub code: u16,
    pub pad: Box<[u8]>,
}
impl Default for record {
    fn default() -> Self {
        record {
            code: <u16>::default(),
            pad: (0..14).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
        }
    }
}
impl ByteRepr for record {
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
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn h(&self) -> Ptr<record> {
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
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 128]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        128
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
pub struct inner {
    pub view: anon_0,
}
impl ByteRepr for inner {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.view.to_bytes(&mut buf[0..128]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            view: <anon_0>::from_bytes(&buf[0..128]),
        }
    }
}
pub struct anon_1 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_1 {
    pub fn h(&self) -> Ptr<record> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn nested(&self) -> Ptr<inner> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_1 {
    fn clone(&self) -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 128]))),
        }
    }
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Outer {
    pub kind: i32,
    pub level: i32,
    pub variant: i32,
    pub len: u32,
    pub body: anon_1,
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        144
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.kind.to_bytes(&mut buf[0..4]);
        self.level.to_bytes(&mut buf[4..8]);
        self.variant.to_bytes(&mut buf[8..12]);
        self.len.to_bytes(&mut buf[12..16]);
        self.body.to_bytes(&mut buf[16..144]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: <i32>::from_bytes(&buf[0..4]),
            level: <i32>::from_bytes(&buf[4..8]),
            variant: <i32>::from_bytes(&buf[8..12]),
            len: <u32>::from_bytes(&buf[12..16]),
            body: <anon_1>::from_bytes(&buf[16..144]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let ex: Value<Outer> = <Value<Outer>>::default();
    {
        ((ex.as_pointer()) as Ptr<Outer>)
            .to_any()
            .memset((0) as u8, 144usize as usize);
        ((ex.as_pointer()) as Ptr<Outer>).to_any().clone()
    };
    (*ex.borrow_mut()).kind = 2;
    (*ex.borrow_mut()).level = 1;
    (*ex.borrow_mut()).variant = 6;
    (*ex.borrow_mut()).len = (16usize as u32);
    (ex.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(16usize)
        .reinterpret_cast::<record>() as Ptr<record>)
        .with_mut(|__v| __v.code = 2_u16);
    (ex.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(16usize)
        .reinterpret_cast::<record>() as Ptr<record>)
        .with_mut(|__v| __v.pad[(0) as usize] = (('X' as i32) as u8));
    assert!(
        (((((ex
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(16usize)
            .reinterpret_cast::<record>() as Ptr<record>)
            .with(|__v| __v.code) as i32)
            == 2) as i32)
            != 0)
    );
    assert!(
        (((((ex
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(16usize)
            .reinterpret_cast::<record>() as Ptr<record>)
            .with(|__v| __v.pad[(0) as usize]) as i32)
            == ('X' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((ex
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(16usize)
            .reinterpret_cast::<record>() as Ptr<record>)
            .with(|__v| __v.code) as i32)
            == 2) as i32)
            != 0)
    );
    return 0;
}
