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
pub struct bits {
    pub __bits_0: [u8; 3],
}
impl bits {
    #[inline]
    pub const fn b(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 0) & 0x7) << 0) as u32
    }
    #[inline]
    pub const fn set_b(&mut self, v: u32) {
        assert!(v <= 7, "bitfield b: value does not fit in 3 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0x07u8) | ((((__v >> 0) as u8) << 0) & 0x07u8);
    }
    #[inline]
    pub const fn with_b(mut self, v: u32) -> Self {
        self.set_b(v);
        self
    }
    #[inline]
    pub const fn w(&self) -> u32 {
        ((((self.__bits_0[0] as u64) >> 3) & 0x1f) << 0
            | (((self.__bits_0[1] as u64) >> 0) & 0x7f) << 5) as u32
    }
    #[inline]
    pub const fn set_w(&mut self, v: u32) {
        assert!(v <= 4095, "bitfield w: value does not fit in 12 bits");
        let __v = v as u64;
        self.__bits_0[0] = (self.__bits_0[0] & !0xf8u8) | ((((__v >> 0) as u8) << 3) & 0xf8u8);
        self.__bits_0[1] = (self.__bits_0[1] & !0x7fu8) | ((((__v >> 5) as u8) << 0) & 0x7fu8);
    }
    #[inline]
    pub const fn with_w(mut self, v: u32) -> Self {
        self.set_w(v);
        self
    }
    #[inline]
    pub const fn s(&self) -> i32 {
        (((((((self.__bits_0[1] as u64) >> 7) & 0x1) << 0
            | (((self.__bits_0[2] as u64) >> 0) & 0x3) << 1)
            << 61) as i64)
            >> 61) as i32
    }
    #[inline]
    pub const fn set_s(&mut self, v: i32) {
        assert!(v >= -4 && v <= 3, "bitfield s: value out of range");
        let __v = v as u64;
        self.__bits_0[1] = (self.__bits_0[1] & !0x80u8) | ((((__v >> 0) as u8) << 7) & 0x80u8);
        self.__bits_0[2] = (self.__bits_0[2] & !0x03u8) | ((((__v >> 1) as u8) << 0) & 0x03u8);
    }
    #[inline]
    pub const fn with_s(mut self, v: i32) -> Self {
        self.set_s(v);
        self
    }
}
impl ByteRepr for bits {
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
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let nine: Value<i32> = Rc::new(RefCell::new(9));
    let big: Value<i32> = Rc::new(RefCell::new(4660));
    let seven: Value<i32> = Rc::new(RefCell::new(7));
    let v: Value<bits> = <Value<bits>>::default();
    (*v.borrow_mut()).set_b(0_u32);
    (*v.borrow_mut()).set_w(0_u32);
    (*v.borrow_mut()).set_s(0);
    (*v.borrow_mut()).set_b(((*nine.borrow()) as u32));
    assert!((((((*v.borrow_mut()).b() as i32) == 1) as i32) != 0));
    (*v.borrow_mut()).set_b(7_u32);
    {
        let __bf_old = (*v.borrow_mut()).b();
        let __bf_v = (__bf_old + 1);
        (*v.borrow_mut()).set_b(__bf_v)
    };
    assert!((((((*v.borrow_mut()).b() as i32) == 0) as i32) != 0));
    (*v.borrow_mut()).set_b(0_u32);
    {
        let __bf_old = (*v.borrow_mut()).b();
        let __bf_v = (__bf_old - 1);
        (*v.borrow_mut()).set_b(__bf_v)
    };
    assert!((((((*v.borrow_mut()).b() as i32) == 7) as i32) != 0));
    (*v.borrow_mut()).set_w(((*big.borrow()) as u32));
    assert!((((((*v.borrow_mut()).w() as i32) == 564) as i32) != 0));
    (*v.borrow_mut()).set_s((*seven.borrow()));
    assert!(((((*v.borrow_mut()).s() == -1_i32) as i32) != 0));
    (*v.borrow_mut()).set_s(3);
    {
        let __bf_old = (*v.borrow_mut()).s();
        let __bf_v = (__bf_old + 1);
        (*v.borrow_mut()).set_s(__bf_v)
    };
    assert!(((((*v.borrow_mut()).s() == -4_i32) as i32) != 0));
    (*v.borrow_mut()).set_s(-4_i32);
    {
        let __bf_old = (*v.borrow_mut()).s();
        let __bf_v = (__bf_old - 1);
        (*v.borrow_mut()).set_s(__bf_v)
    };
    assert!(((((*v.borrow_mut()).s() == 3) as i32) != 0));
    return 0;
}
