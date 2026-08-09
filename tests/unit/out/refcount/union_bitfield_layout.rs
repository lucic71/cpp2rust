extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C, align(4))]
#[derive(Clone, Default)]
#[bitfields(__bits_0 { a: u32 @ 0..1 unsigned, b: u32 @ 1..4 unsigned, wide: u32 @ 4..24 unsigned, sgn: i32 @ 24..28 signed })]
pub struct packed_flags {
    pub __bits_0: [u8; 4],
    pub tail: u32,
}
impl ByteRepr for packed_flags {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf[0..4].copy_from_slice(&self.__bits_0);
        self.tail.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            __bits_0: buf[0..4].try_into().unwrap(),
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
        .with_mut(|__v| __v.set_a(1_u32));
    (v.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
        .with_mut(|__v| __v.set_b(5_u32));
    (v.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(0usize)
        .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
        .with_mut(|__v| __v.set_wide(703710_u32));
    {
        let __bf_v = -3_i32;
        (v.as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with_mut(|__v| __v.set_sgn(__bf_v))
    };
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
        .with_mut(|__v| __v.set_b(2_u32));
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
            .with(|__v| __v.a()) as i32)
            == 1) as i32)
            != 0)
    );
    assert!(
        (((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| __v.wide()) as i32)
            == 703710) as i32)
            != 0)
    );
    assert!(
        ((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| __v.sgn())
            == -3_i32) as i32)
            != 0)
    );
    assert!(
        ((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| __v.tail)
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
            .with(|__v| __v.a()) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| __v.b()) as i32)
            == 6) as i32)
            != 0)
    );
    assert!(
        (((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| __v.wide()) as i32)
            == 291) as i32)
            != 0)
    );
    assert!(
        ((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| __v.sgn())
            == -1_i32) as i32)
            != 0)
    );
    assert!(
        ((((v
            .as_pointer()
            .reinterpret_cast::<u8>()
            .offset(0usize)
            .reinterpret_cast::<packed_flags>() as Ptr<packed_flags>)
            .with(|__v| __v.tail)
            == 0_u32) as i32)
            != 0)
    );
    return 0;
}
