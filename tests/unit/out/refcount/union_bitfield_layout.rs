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
pub struct packed_flags {
    pub a: u32,
    pub b: u32,
    pub wide: u32,
    pub sgn: i32,
    pub tail: u32,
}
impl ByteRepr for packed_flags {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        {
            let __v = self.a as u64;
            buf[0] = (buf[0] & !0x01u8) | ((((__v >> 0) as u8) << 0) & 0x01u8);
        }
        {
            let __v = self.b as u64;
            buf[0] = (buf[0] & !0x0eu8) | ((((__v >> 0) as u8) << 1) & 0x0eu8);
        }
        {
            let __v = self.wide as u64;
            buf[0] = (buf[0] & !0xf0u8) | ((((__v >> 0) as u8) << 4) & 0xf0u8);
            buf[1] = (buf[1] & !0xffu8) | ((((__v >> 4) as u8) << 0) & 0xffu8);
            buf[2] = (buf[2] & !0xffu8) | ((((__v >> 12) as u8) << 0) & 0xffu8);
        }
        {
            let __v = self.sgn as u64;
            buf[3] = (buf[3] & !0x0fu8) | ((((__v >> 0) as u8) << 0) & 0x0fu8);
        }
        self.tail.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: (((buf[0] as u64 >> 0) & 0x1) << 0) as u32,
            b: (((buf[0] as u64 >> 1) & 0x7) << 0) as u32,
            wide: ((((buf[0] as u64 >> 4) & 0xf) << 0)
                | (((buf[1] as u64 >> 0) & 0xff) << 4)
                | (((buf[2] as u64 >> 0) & 0xff) << 12)) as u32,
            sgn: ((((((buf[3] as u64 >> 0) & 0xf) << 0) << 60) as i64) >> 60) as i32,
            tail: <u32>::from_bytes(&buf[4..8]),
        }
    }
}
pub struct view {
    __bytes: Value<Box<[u8]>>,
}
impl view {
    pub fn f(&self) -> Ptr<packed_flags> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn raw_(&self) -> Ptr<u8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for view {
    fn clone(&self) -> Self {
        view {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for view {
    fn default() -> Self {
        view {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
impl ByteRepr for view {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        view {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let v: Value<view> = <Value<view>>::default();
    {
        ((v.as_pointer()) as Ptr<view>)
            .to_any()
            .memset((0) as u8, 8usize as usize);
        ((v.as_pointer()) as Ptr<view>).to_any().clone()
    };
    (v.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
        .with_mut(|__v| __v.a = 1_u32);
    (v.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
        .with_mut(|__v| __v.b = 5_u32);
    (v.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
        .with_mut(|__v| __v.wide = 703710_u32);
    (v.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
        .with_mut(|__v| __v.sgn = -3_i32);
    (v.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
        .with_mut(|__v| __v.tail = 287454020_u32);
    assert!(
        (((((((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((0) as isize))
            .read()) as i32)
            == 235) as i32)
            != 0)
    );
    assert!(
        (((((((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((1) as isize))
            .read()) as i32)
            == 205) as i32)
            != 0)
    );
    assert!(
        (((((((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((2) as isize))
            .read()) as i32)
            == 171) as i32)
            != 0)
    );
    assert!(
        (((((((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((3) as isize))
            .read()) as i32)
            == 13) as i32)
            != 0)
    );
    assert!(
        (((((((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((4) as isize))
            .read()) as i32)
            == 68) as i32)
            != 0)
    );
    assert!(
        (((((((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((5) as isize))
            .read()) as i32)
            == 51) as i32)
            != 0)
    );
    assert!(
        (((((((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((6) as isize))
            .read()) as i32)
            == 34) as i32)
            != 0)
    );
    assert!(
        (((((((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((7) as isize))
            .read()) as i32)
            == 17) as i32)
            != 0)
    );
    (v.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
        .with_mut(|__v| __v.b = 2_u32);
    assert!(
        (((((((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((0) as isize))
            .read()) as i32)
            == 229) as i32)
            != 0)
    );
    assert!(
        (((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| (*__v).a) as i32)
            == 1) as i32)
            != 0)
    );
    assert!(
        (((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| (*__v).wide) as i32)
            == 703710) as i32)
            != 0)
    );
    assert!(
        ((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| (*__v).sgn)
            == -3_i32) as i32)
            != 0)
    );
    assert!(
        ((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| (*__v).tail)
            == 287454020_u32) as i32)
            != 0)
    );
    {
        ((v.as_pointer()) as Ptr<view>)
            .to_any()
            .memset((0) as u8, 8usize as usize);
        ((v.as_pointer()) as Ptr<view>).to_any().clone()
    };
    ((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
        .offset(((0) as isize))
        .write(60_u8);
    ((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
        .offset(((1) as isize))
        .write(18_u8);
    ((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
        .offset(((2) as isize))
        .write(0_u8);
    ((v.as_pointer().reinterpret_cast::<u8>().offset(0usize) as Ptr<u8>) as Ptr<u8>)
        .offset(((3) as isize))
        .write(15_u8);
    assert!(
        (((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| (*__v).a) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| (*__v).b) as i32)
            == 6) as i32)
            != 0)
    );
    assert!(
        (((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| (*__v).wide) as i32)
            == 291) as i32)
            != 0)
    );
    assert!(
        ((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| (*__v).sgn)
            == -1_i32) as i32)
            != 0)
    );
    assert!(
        ((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| (*__v).tail)
            == 0_u32) as i32)
            != 0)
    );
    return 0;
}
