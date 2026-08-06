extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(u32)]
pub enum Width_enum {
    #[default]
    W_64 = 0,
    W_32 = 1,
    W_16 = 2,
}
impl From<i32> for Width_enum {
    fn from(n: i32) -> Width_enum {
        match n {
            0 => Width_enum::W_64,
            1 => Width_enum::W_32,
            2 => Width_enum::W_16,
            _ => panic!("invalid Width_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Width_enum);
impl ByteRepr for Width_enum {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <Width_enum>::from(i32::from_bytes(buf))
    }
}
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn text(&self) -> Ptr<Ptr<u8>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn handle(&self) -> Ptr<AnyPtr> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn signed_n(&self) -> Ptr<i64> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn f(&self) -> Ptr<f64> {
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
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        8
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
pub struct Sink {
    pub width: Width_enum,
    pub out: anon_0,
}
impl ByteRepr for Sink {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.width.to_bytes(&mut buf[0..4]);
        self.out.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            width: <Width_enum>::from_bytes(&buf[0..4]),
            out: <anon_0>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn write_count_1(s: Ptr<Sink>, count: i64) {
    let s: Value<Ptr<Sink>> = Rc::new(RefCell::new(s));
    let count: Value<i64> = Rc::new(RefCell::new(count));
    'switch: {
        let __match_cond = ((*s.borrow()).with(|__v| (*__v).width) as u32);
        match __match_cond {
            __v if __v == ((Width_enum::W_64 as i32) as u32) => {
                {
                    let __rhs = (*count.borrow());
                    (((*s.borrow())
                        .reinterpret_cast::<u8>()
                        .offset(8usize)
                        .reinterpret_cast::<AnyPtr>() as Ptr<AnyPtr>)
                        .read())
                    .reinterpret_cast::<i64>()
                    .write(__rhs)
                };
                break 'switch;
            }
            __v if __v == ((Width_enum::W_32 as i32) as u32) => {
                {
                    let __rhs = ((*count.borrow()) as i32);
                    (((*s.borrow())
                        .reinterpret_cast::<u8>()
                        .offset(8usize)
                        .reinterpret_cast::<AnyPtr>() as Ptr<AnyPtr>)
                        .read())
                    .reinterpret_cast::<i32>()
                    .write(__rhs)
                };
                break 'switch;
            }
            __v if __v == ((Width_enum::W_16 as i32) as u32) => {
                {
                    let __rhs = ((*count.borrow()) as i16);
                    (((*s.borrow())
                        .reinterpret_cast::<u8>()
                        .offset(8usize)
                        .reinterpret_cast::<AnyPtr>() as Ptr<AnyPtr>)
                        .read())
                    .reinterpret_cast::<i16>()
                    .write(__rhs)
                };
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let buf64: Value<i64> = Rc::new(RefCell::new(0_i64));
    let buf32: Value<i32> = Rc::new(RefCell::new(0));
    let buf16: Value<i16> = Rc::new(RefCell::new(0_i16));
    let s: Value<Sink> = <Value<Sink>>::default();
    (*s.borrow_mut()).width = Width_enum::W_64;
    (s.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<AnyPtr>() as Ptr<AnyPtr>)
        .write(((buf64.as_pointer()) as Ptr<i64>).to_any());
    ({ write_count_1((s.as_pointer()), 1234605616436508552_i64) });
    assert!(((((*buf64.borrow()) == 1234605616436508552_i64) as i32) != 0));
    (*s.borrow_mut()).width = Width_enum::W_32;
    (s.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<AnyPtr>() as Ptr<AnyPtr>)
        .write(((buf32.as_pointer()) as Ptr<i32>).to_any());
    ({ write_count_1((s.as_pointer()), 305419896_i64) });
    assert!(((((*buf32.borrow()) == 305419896) as i32) != 0));
    (*s.borrow_mut()).width = Width_enum::W_16;
    (s.as_pointer()
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<AnyPtr>() as Ptr<AnyPtr>)
        .write(((buf16.as_pointer()) as Ptr<i16>).to_any());
    ({ write_count_1((s.as_pointer()), 4660_i64) });
    assert!((((((*buf16.borrow()) as i32) == 4660) as i32) != 0));
    return 0;
}
