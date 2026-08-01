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
pub struct anon_1 {
    pub elem: Ptr<Ptr<u8>>,
    pub size: i64,
    pub idx: i64,
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.elem.to_bytes(&mut buf[0..8]);
        self.size.to_bytes(&mut buf[8..16]);
        self.idx.to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            elem: <Ptr<Ptr<u8>>>::from_bytes(&buf[0..8]),
            size: <i64>::from_bytes(&buf[8..16]),
            idx: <i64>::from_bytes(&buf[16..24]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct anon_2 {
    pub min: i32,
    pub max: i32,
}
impl ByteRepr for anon_2 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.min.to_bytes(&mut buf[0..4]);
        self.max.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            min: <i32>::from_bytes(&buf[0..4]),
            max: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn set(&self) -> Ptr<anon_1> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn range(&self) -> Ptr<anon_2> {
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
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 24]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        24
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
pub struct entry {
    pub kind: i32,
    pub c: anon_0,
}
impl ByteRepr for entry {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.kind.to_bytes(&mut buf[0..4]);
        self.c.to_bytes(&mut buf[8..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: <i32>::from_bytes(&buf[0..4]),
            c: <anon_0>::from_bytes(&buf[8..32]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let table: Value<Ptr<entry>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount((2_usize).wrapping_mul((32usize as usize)))
            .reinterpret_cast::<entry>(),
    ));
    assert!((((!((*table.borrow()).is_null())) as i32) != 0));
    {
        ((*table.borrow()).clone() as Ptr<entry>).to_any().memset(
            (0) as u8,
            (2_usize).wrapping_mul((32usize as usize)) as usize,
        );
        ((*table.borrow()).clone() as Ptr<entry>).to_any().clone()
    };
    let e: Value<Ptr<entry>> = Rc::new(RefCell::new(((*table.borrow()).offset(((0) as isize)))));
    (*e.borrow()).with_mut(|__v| __v.kind = 1);
    ((*e.borrow())
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
        .with_mut(|__v| __v.size = 7_i64);
    ((*e.borrow())
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
        .with_mut(|__v| __v.idx = 3_i64);
    ((*e.borrow())
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
        .with_mut(|__v| __v.elem = libcc2rs::malloc_refcount(8usize).reinterpret_cast::<Ptr<u8>>());
    assert!(
        (((!((((*e.borrow())
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
            .with(|__v| (*__v).elem.clone()))
        .is_null())) as i32)
            != 0)
    );
    let __rhs = libcc2rs::strdup_refcount(Ptr::from_string_literal(b"alpha").clone());
    ((*e.borrow())
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
        .with(|__v| (*__v).elem.offset(((0) as isize)).clone())
        .write(__rhs);
    assert!(
        (((!((((*e.borrow())
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
            .with(|__v| (*__v).elem.offset(((0) as isize)).clone())
            .read())
        .is_null())) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (((*e.borrow())
                .reinterpret_cast::<u8>()
                .offset(8usize)
                .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
                .with(|__v| (*__v).elem.offset(((0) as isize)).clone())
                .read())
            .to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"alpha").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((((*e.borrow())
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
            .with(|__v| (*__v).size)
            == 7_i64) as i32)
            != 0)
    );
    assert!(
        (((((*e.borrow())
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
            .with(|__v| (*__v).idx)
            == 3_i64) as i32)
            != 0)
    );
    assert!(((((*e.borrow()).with(|__v| (*__v).kind) == 1) as i32) != 0));
    (*e.borrow_mut()) = ((*table.borrow()).offset(((1) as isize)));
    (*e.borrow()).with_mut(|__v| __v.kind = 2);
    ((*e.borrow())
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<anon_2>() as Ptr<anon_2>)
        .with_mut(|__v| __v.min = 10);
    ((*e.borrow())
        .reinterpret_cast::<u8>()
        .offset(8usize)
        .reinterpret_cast::<anon_2>() as Ptr<anon_2>)
        .with_mut(|__v| __v.max = 20);
    assert!(
        (((((*e.borrow())
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<anon_2>() as Ptr<anon_2>)
            .with(|__v| (*__v).min)
            == 10) as i32)
            != 0)
    );
    assert!(
        (((((*e.borrow())
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<anon_2>() as Ptr<anon_2>)
            .with(|__v| (*__v).max)
            == 20) as i32)
            != 0)
    );
    assert!(
        (((((*table.borrow())
            .offset(((0) as isize))
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
            .with(|__v| (*__v).size)
            == 7_i64) as i32)
            != 0)
    );
    libcc2rs::free_refcount(
        ((((*table.borrow())
            .offset(((0) as isize))
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
            .with(|__v| (*__v).elem.offset(((0) as isize)).clone())
            .read())
        .clone() as Ptr<u8>)
            .to_any(),
    );
    libcc2rs::free_refcount(
        ((((*table.borrow())
            .offset(((0) as isize))
            .reinterpret_cast::<u8>()
            .offset(8usize)
            .reinterpret_cast::<anon_1>() as Ptr<anon_1>)
            .with(|__v| (*__v).elem.clone()))
        .clone() as Ptr<Ptr<u8>>)
            .to_any(),
    );
    libcc2rs::free_refcount(((*table.borrow()).clone() as Ptr<entry>).to_any());
    return 0;
}
