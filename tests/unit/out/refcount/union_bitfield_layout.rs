extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct packed_flags {
    pub a: Value<u32>,
    pub b: Value<u32>,
    pub wide: Value<u32>,
    pub sgn: Value<i32>,
    pub tail: Value<u32>,
}
impl Clone for packed_flags {
    fn clone(&self) -> Self {
        Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()).clone())),
            wide: Rc::new(RefCell::new((*self.wide.borrow()).clone())),
            sgn: Rc::new(RefCell::new((*self.sgn.borrow()).clone())),
            tail: Rc::new(RefCell::new((*self.tail.borrow()).clone())),
        }
    }
}
impl ByteRepr for packed_flags {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        {
            let __v = (*self.a.borrow()) as u64;
            buf[0] = (buf[0] & !0x01u8) | ((((__v >> 0) as u8) << 0) & 0x01u8);
        }
        {
            let __v = (*self.b.borrow()) as u64;
            buf[0] = (buf[0] & !0x0eu8) | ((((__v >> 0) as u8) << 1) & 0x0eu8);
        }
        {
            let __v = (*self.wide.borrow()) as u64;
            buf[0] = (buf[0] & !0xf0u8) | ((((__v >> 0) as u8) << 4) & 0xf0u8);
            buf[1] = (buf[1] & !0xffu8) | ((((__v >> 4) as u8) << 0) & 0xffu8);
            buf[2] = (buf[2] & !0xffu8) | ((((__v >> 12) as u8) << 0) & 0xffu8);
        }
        {
            let __v = (*self.sgn.borrow()) as u64;
            buf[3] = (buf[3] & !0x0fu8) | ((((__v >> 0) as u8) << 0) & 0x0fu8);
        }
        (*self.tail.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new((((buf[0] as u64 >> 0) & 0x1) << 0) as u32)),
            b: Rc::new(RefCell::new((((buf[0] as u64 >> 1) & 0x7) << 0) as u32)),
            wide: Rc::new(RefCell::new(
                ((((buf[0] as u64 >> 4) & 0xf) << 0)
                    | (((buf[1] as u64 >> 0) & 0xff) << 4)
                    | (((buf[2] as u64 >> 0) & 0xff) << 12)) as u32,
            )),
            sgn: Rc::new(RefCell::new(
                ((((((buf[3] as u64 >> 0) & 0xf) << 0) << 60) as i64) >> 60) as i32,
            )),
            tail: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
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
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<view> = <Value<view>>::default();
    {
        ((v.as_pointer()) as Ptr<view>)
            .to_any()
            .memset((0) as u8, 8usize as usize);
        ((v.as_pointer()) as Ptr<view>).to_any().clone()
    };
    (*v.borrow())
        .f()
        .with_mut(|__v| (*__v.a.borrow_mut()) = 1_u32);
    (*v.borrow())
        .f()
        .with_mut(|__v| (*__v.b.borrow_mut()) = 5_u32);
    (*v.borrow())
        .f()
        .with_mut(|__v| (*__v.wide.borrow_mut()) = 703710_u32);
    (*v.borrow())
        .f()
        .with_mut(|__v| (*__v.sgn.borrow_mut()) = -3_i32);
    (*v.borrow())
        .f()
        .with_mut(|__v| (*__v.tail.borrow_mut()) = 287454020_u32);
    assert!(
        (((((((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((0) as isize)
            .read()) as i32)
            == 235) as i32)
            != 0)
    );
    assert!(
        (((((((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((1) as isize)
            .read()) as i32)
            == 205) as i32)
            != 0)
    );
    assert!(
        (((((((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((2) as isize)
            .read()) as i32)
            == 171) as i32)
            != 0)
    );
    assert!(
        (((((((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((3) as isize)
            .read()) as i32)
            == 13) as i32)
            != 0)
    );
    assert!(
        (((((((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((4) as isize)
            .read()) as i32)
            == 68) as i32)
            != 0)
    );
    assert!(
        (((((((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((5) as isize)
            .read()) as i32)
            == 51) as i32)
            != 0)
    );
    assert!(
        (((((((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((6) as isize)
            .read()) as i32)
            == 34) as i32)
            != 0)
    );
    assert!(
        (((((((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((7) as isize)
            .read()) as i32)
            == 17) as i32)
            != 0)
    );
    (*v.borrow())
        .f()
        .with_mut(|__v| (*__v.b.borrow_mut()) = 2_u32);
    assert!(
        (((((((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((0) as isize)
            .read()) as i32)
            == 229) as i32)
            != 0)
    );
    assert!((((((*(*(*v.borrow()).f().upgrade().deref()).a.borrow()) as i32) == 1) as i32) != 0));
    assert!(
        (((((*(*(*v.borrow()).f().upgrade().deref()).wide.borrow()) as i32) == 703710) as i32)
            != 0)
    );
    assert!(((((*(*(*v.borrow()).f().upgrade().deref()).sgn.borrow()) == -3_i32) as i32) != 0));
    assert!(
        ((((*(*(*v.borrow()).f().upgrade().deref()).tail.borrow()) == 287454020_u32) as i32) != 0)
    );
    {
        ((v.as_pointer()) as Ptr<view>)
            .to_any()
            .memset((0) as u8, 8usize as usize);
        ((v.as_pointer()) as Ptr<view>).to_any().clone()
    };
    ((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr<u8>)
        .offset((0) as isize)
        .write(60_u8);
    ((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr<u8>)
        .offset((1) as isize)
        .write(18_u8);
    ((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr<u8>)
        .offset((2) as isize)
        .write(0_u8);
    ((*v.borrow()).raw_().reinterpret_cast::<u8>() as Ptr<u8>)
        .offset((3) as isize)
        .write(15_u8);
    assert!((((((*(*(*v.borrow()).f().upgrade().deref()).a.borrow()) as i32) == 0) as i32) != 0));
    assert!((((((*(*(*v.borrow()).f().upgrade().deref()).b.borrow()) as i32) == 6) as i32) != 0));
    assert!(
        (((((*(*(*v.borrow()).f().upgrade().deref()).wide.borrow()) as i32) == 291) as i32) != 0)
    );
    assert!(((((*(*(*v.borrow()).f().upgrade().deref()).sgn.borrow()) == -1_i32) as i32) != 0));
    assert!(((((*(*(*v.borrow()).f().upgrade().deref()).tail.borrow()) == 0_u32) as i32) != 0));
    return 0;
}
