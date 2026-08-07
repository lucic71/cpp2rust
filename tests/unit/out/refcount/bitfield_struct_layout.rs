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
pub struct flags {
    pub tag: u8,
    pub __bits_0: [u8; 1],
    pub x: i32,
    pub __bits_1: [u8; 1],
}
impl flags {
    #[inline]
    pub const fn a(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 0) & 0x1) << 0) as u32
    }
    #[inline]
    pub const fn set_a(&mut self, v: u32) {
        assert!(v <= 1, "bitfield a: value does not fit in 1 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0x01u8) | ((((__v >> 0) as u8) << 0) & 0x01u8);
    }
    #[inline]
    pub const fn with_a(mut self, v: u32) -> Self {
        self.set_a(v);
        self
    }
    #[inline]
    pub const fn b(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 1) & 0x7) << 0) as u32
    }
    #[inline]
    pub const fn set_b(&mut self, v: u32) {
        assert!(v <= 7, "bitfield b: value does not fit in 3 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0x0eu8) | ((((__v >> 0) as u8) << 1) & 0x0eu8);
    }
    #[inline]
    pub const fn with_b(mut self, v: u32) -> Self {
        self.set_b(v);
        self
    }
    #[inline]
    pub const fn c(&self) -> u32 {
        ((((self.__bits_1[0] as u64) >> 0) & 0x1) << 0) as u32
    }
    #[inline]
    pub const fn set_c(&mut self, v: u32) {
        assert!(v <= 1, "bitfield c: value does not fit in 1 bits");
        let __v = v as u64;
        self.__bits_1[0] = (self.__bits_1[0] & !0x01u8) | ((((__v >> 0) as u8) << 0) & 0x01u8);
    }
    #[inline]
    pub const fn with_c(mut self, v: u32) -> Self {
        self.set_c(v);
        self
    }
}
impl ByteRepr for flags {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf[1..2].copy_from_slice(&self.__bits_0);
        buf[8..9].copy_from_slice(&self.__bits_1);
        self.tag.to_bytes(&mut buf[0..1]);
        self.x.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            __bits_0: buf[1..2].try_into().unwrap(),
            __bits_1: buf[8..9].try_into().unwrap(),
            tag: <u8>::from_bytes(&buf[0..1]),
            x: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct outer {
    pub lead: u8,
    pub f: flags,
}
impl ByteRepr for outer {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.lead.to_bytes(&mut buf[0..1]);
        self.f.to_bytes(&mut buf[4..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            lead: <u8>::from_bytes(&buf[0..1]),
            f: <flags>::from_bytes(&buf[4..16]),
        }
    }
}
#[repr(C, align(4))]
#[derive(Clone, Default)]
pub struct mixed_sign {
    pub __bits_0: [u8; 3],
}
impl mixed_sign {
    #[inline]
    pub const fn s(&self) -> i32 {
        (((((((self.__bits_0[0] as u64) >> 0) & 0x7) << 0) << 61) as i64) >> 61) as i32
    }
    #[inline]
    pub const fn set_s(&mut self, v: i32) {
        assert!(v >= -4 && v <= 3, "bitfield s: value out of range");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0x07u8) | ((((__v >> 0) as u8) << 0) & 0x07u8);
    }
    #[inline]
    pub const fn with_s(mut self, v: i32) -> Self {
        self.set_s(v);
        self
    }
    #[inline]
    pub const fn u(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 3) & 0x1f) << 0) as u32
    }
    #[inline]
    pub const fn set_u(&mut self, v: u32) {
        assert!(v <= 31, "bitfield u: value does not fit in 5 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0xf8u8) | ((((__v >> 0) as u8) << 3) & 0xf8u8);
    }
    #[inline]
    pub const fn with_u(mut self, v: u32) -> Self {
        self.set_u(v);
        self
    }
    #[inline]
    pub const fn wide(&self) -> u32 {
        ((((self.__bits_0[1] as u64) >> 0) & 0xff) << 0
            | (((self.__bits_0[2] as u64) >> 0) & 0xf) << 8) as u32
    }
    #[inline]
    pub const fn set_wide(&mut self, v: u32) {
        assert!(v <= 4095, "bitfield wide: value does not fit in 12 bits");
        let __v = v as u64;
        self.__bits_0[1] = (self.__bits_0[1] & !0xffu8) | ((((__v >> 0) as u8) << 0) & 0xffu8);
        self.__bits_0[2] = (self.__bits_0[2] & !0x0fu8) | ((((__v >> 8) as u8) << 0) & 0x0fu8);
    }
    #[inline]
    pub const fn with_wide(mut self, v: u32) -> Self {
        self.set_wide(v);
        self
    }
}
impl ByteRepr for mixed_sign {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf[0..3].copy_from_slice(&self.__bits_0);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            __bits_0: buf[0..3].try_into().unwrap(),
        }
    }
}
thread_local!(
    pub static g_0: Value<flags> = Rc::new(RefCell::new(
        flags {
            tag: 2_u8,
            x: 7,
            __bits_0: [0u8; 1],
            __bits_1: [0u8; 1],
        }
        .with_a(1_u32)
        .with_b(5_u32)
        .with_c(0_u32),
    ));
);
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((12usize == 12_usize) as i32) != 0));
    assert!((((0_usize == 0_usize) as i32) != 0));
    assert!((((4_usize == 4_usize) as i32) != 0));
    assert!((((4_usize == 4_usize) as i32) != 0));
    assert!((((4usize == 4_usize) as i32) != 0));
    assert!(
        (((((((((((((((((*g_0.with(Value::clone).borrow()).tag as i32) == 2) as i32) != 0)
            && (((((*g_0.with(Value::clone).borrow_mut()).a() as i32) == 1) as i32) != 0))
            as i32)
            != 0)
            && (((((*g_0.with(Value::clone).borrow_mut()).b() as i32) == 5) as i32) != 0))
            as i32)
            != 0)
            && ((((*g_0.with(Value::clone).borrow()).x == 7) as i32) != 0)) as i32)
            != 0)
            && (((((*g_0.with(Value::clone).borrow_mut()).c() as i32) == 0) as i32) != 0))
            as i32)
            != 0)
    );
    let f: Value<flags> = <Value<flags>>::default();
    {
        ((f.as_pointer()) as Ptr<flags>)
            .to_any()
            .memset((0) as u8, 12usize as usize);
        ((f.as_pointer()) as Ptr<flags>).to_any().clone()
    };
    assert!(
        (((((((((((((((((*f.borrow()).tag as i32) == 0) as i32) != 0)
            && (((((*f.borrow_mut()).a() as i32) == 0) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow_mut()).b() as i32) == 0) as i32) != 0)) as i32)
            != 0)
            && ((((*f.borrow()).x == 0) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow_mut()).c() as i32) == 0) as i32) != 0)) as i32)
            != 0)
    );
    (*f.borrow_mut()).set_a(1_u32);
    assert!(
        (((((((((((((((((*f.borrow_mut()).a() as i32) == 1) as i32) != 0)
            && (((((*f.borrow_mut()).b() as i32) == 0) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow_mut()).c() as i32) == 0) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow()).tag as i32) == 0) as i32) != 0)) as i32)
            != 0)
            && ((((*f.borrow()).x == 0) as i32) != 0)) as i32)
            != 0)
    );
    (*f.borrow_mut()).set_b(5_u32);
    assert!(
        ((((((((*f.borrow_mut()).b() as i32) == 5) as i32) != 0)
            && (((((*f.borrow_mut()).a() as i32) == 1) as i32) != 0)) as i32)
            != 0)
    );
    (*f.borrow_mut()).tag = 255_u8;
    assert!(
        (((((((((((*f.borrow()).tag as i32) == 255) as i32) != 0)
            && (((((*f.borrow_mut()).a() as i32) == 1) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow_mut()).b() as i32) == 5) as i32) != 0)) as i32)
            != 0)
    );
    {
        let __bf_old = (*f.borrow_mut()).b();
        let __bf_v = (__bf_old + 1);
        (*f.borrow_mut()).set_b(__bf_v)
    };
    assert!(
        ((((((((*f.borrow_mut()).b() as i32) == 6) as i32) != 0)
            && (((((*f.borrow_mut()).a() as i32) == 1) as i32) != 0)) as i32)
            != 0)
    );
    {
        let __bf_old = (*f.borrow_mut()).b();
        let __bf_v = (__bf_old + (1));
        (*f.borrow_mut()).set_b(__bf_v)
    };
    assert!(
        ((((((((*f.borrow_mut()).b() as i32) == 7) as i32) != 0)
            && (((((*f.borrow_mut()).a() as i32) == 1) as i32) != 0)) as i32)
            != 0)
    );
    (*f.borrow_mut()).set_c(1_u32);
    assert!(
        (((((((((((*f.borrow_mut()).c() as i32) == 1) as i32) != 0)
            && (((((*f.borrow_mut()).a() as i32) == 1) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow_mut()).b() as i32) == 7) as i32) != 0)) as i32)
            != 0)
    );
    (*f.borrow_mut()).x = -3_i32;
    assert!(
        ((((((((((((((((*f.borrow()).x == -3_i32) as i32) != 0)
            && (((((*f.borrow_mut()).a() as i32) == 1) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow_mut()).b() as i32) == 7) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow_mut()).c() as i32) == 1) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow()).tag as i32) == 255) as i32) != 0)) as i32)
            != 0)
    );
    let px: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (f.as_pointer().field_ptr(
            4,
            |__v: &flags| ::std::slice::from_ref(&__v.x),
            |__v: &mut flags| ::std::slice::from_mut(&mut __v.x),
        )),
    ));
    (*px.borrow()).write(42);
    assert!(
        ((((((((((*f.borrow()).x == 42) as i32) != 0)
            && (((((*f.borrow_mut()).b() as i32) == 7) as i32) != 0)) as i32)
            != 0)
            && (((((*f.borrow_mut()).c() as i32) == 1) as i32) != 0)) as i32)
            != 0)
    );
    let raw_: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..12).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    {
        ((f.as_pointer()) as Ptr<flags>)
            .to_any()
            .memset((0) as u8, 12usize as usize);
        ((f.as_pointer()) as Ptr<flags>).to_any().clone()
    };
    (*f.borrow_mut()).set_b(7_u32);
    {
        ((raw_.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memcpy(&((f.as_pointer()) as Ptr<flags>).to_any(), 12usize as usize);
        ((raw_.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    };
    assert!((((((*raw_.borrow())[(0) as usize] as i32) == 0) as i32) != 0));
    assert!((((((*raw_.borrow())[(1) as usize] as i32) == 14) as i32) != 0));
    let copy: Value<flags> = Rc::new(RefCell::new((*f.borrow()).clone()));
    assert!(
        (((((((((((*copy.borrow_mut()).b() as i32) == 7) as i32) != 0)
            && (((((*copy.borrow_mut()).a() as i32) == 0) as i32) != 0)) as i32)
            != 0)
            && (((((*copy.borrow()).tag as i32) == 0) as i32) != 0)) as i32)
            != 0)
    );
    let dup: Value<flags> = <Value<flags>>::default();
    {
        ((dup.as_pointer()) as Ptr<flags>)
            .to_any()
            .memcpy(&((f.as_pointer()) as Ptr<flags>).to_any(), 12usize as usize);
        ((dup.as_pointer()) as Ptr<flags>).to_any().clone()
    };
    assert!(
        (((((((((((*dup.borrow_mut()).b() as i32) == 7) as i32) != 0)
            && (((((*dup.borrow_mut()).a() as i32) == 0) as i32) != 0)) as i32)
            != 0)
            && (((((*dup.borrow()).tag as i32) == 0) as i32) != 0)) as i32)
            != 0)
    );
    let m: Value<mixed_sign> = <Value<mixed_sign>>::default();
    {
        ((m.as_pointer()) as Ptr<mixed_sign>)
            .to_any()
            .memset((0) as u8, 4usize as usize);
        ((m.as_pointer()) as Ptr<mixed_sign>).to_any().clone()
    };
    (*m.borrow_mut()).set_s(-4_i32);
    assert!(((((*m.borrow_mut()).s() == -4_i32) as i32) != 0));
    (*m.borrow_mut()).set_s(3);
    assert!(((((*m.borrow_mut()).s() == 3) as i32) != 0));
    (*m.borrow_mut()).set_u(31_u32);
    assert!(
        ((((((((*m.borrow_mut()).u() as i32) == 31) as i32) != 0)
            && ((((*m.borrow_mut()).s() == 3) as i32) != 0)) as i32)
            != 0)
    );
    (*m.borrow_mut()).set_wide(2748_u32);
    assert!(
        (((((((((((*m.borrow_mut()).wide() as i32) == 2748) as i32) != 0)
            && (((((*m.borrow_mut()).u() as i32) == 31) as i32) != 0)) as i32)
            != 0)
            && ((((*m.borrow_mut()).s() == 3) as i32) != 0)) as i32)
            != 0)
    );
    (*m.borrow_mut()).set_s(-1_i32);
    assert!(
        ((((((((((*m.borrow_mut()).s() == -1_i32) as i32) != 0)
            && (((((*m.borrow_mut()).u() as i32) == 31) as i32) != 0)) as i32)
            != 0)
            && (((((*m.borrow_mut()).wide() as i32) == 2748) as i32) != 0)) as i32)
            != 0)
    );
    return 0;
}
