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
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.elem.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            elem: <Ptr<Ptr<u8>>>::from_bytes(&buf[0..8]),
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
    pub fn other(&self) -> Ptr<i32> {
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
pub struct entry {
    pub c: anon_0,
}
impl ByteRepr for entry {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.c.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            c: <anon_0>::from_bytes(&buf[0..8]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct holder {
    pub table: Ptr<entry>,
}
impl ByteRepr for holder {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.table.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            table: <Ptr<entry>>::from_bytes(&buf[0..8]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<Ptr<holder>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(8usize).reinterpret_cast::<holder>(),
    ));
    (*h.borrow())
        .with_mut(|__v| __v.table = libcc2rs::malloc_refcount(8usize).reinterpret_cast::<entry>());
    (*h.borrow()).with(|__v| {
        (*__v).table.offset(((0) as isize)).clone().with_mut(|__v| {
            __v.c.set().with_mut(|__v| {
                __v.elem = libcc2rs::malloc_refcount(8usize).reinterpret_cast::<Ptr<u8>>()
            })
        })
    });
    let __rhs = libcc2rs::strdup_refcount(Ptr::from_string_literal(b"alpha").clone());
    (*h.borrow())
        .with(|__v| {
            (*__v).table.offset(((0) as isize)).clone().with(|__v| {
                (*__v)
                    .c
                    .set()
                    .clone()
                    .with(|__v| (*__v).elem.offset(((0) as isize)).clone())
            })
        })
        .write(__rhs);
    assert!(
        ((({
            let mut __it1 = ((*h.borrow())
                .with(|__v| {
                    (*__v).table.offset(((0) as isize)).clone().with(|__v| {
                        (*__v)
                            .c
                            .set()
                            .clone()
                            .with(|__v| (*__v).elem.offset(((0) as isize)).clone())
                    })
                })
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
    libcc2rs::free_refcount(
        (((*h.borrow())
            .with(|__v| {
                (*__v).table.offset(((0) as isize)).clone().with(|__v| {
                    (*__v)
                        .c
                        .set()
                        .clone()
                        .with(|__v| (*__v).elem.offset(((0) as isize)).clone())
                })
            })
            .read())
        .clone() as Ptr<u8>)
            .to_any(),
    );
    (*h.borrow())
        .with(|__v| {
            (*__v).table.offset(((0) as isize)).clone().with(|__v| {
                (*__v)
                    .c
                    .set()
                    .clone()
                    .with(|__v| (*__v).elem.offset(((0) as isize)).clone())
            })
        })
        .write(Ptr::<u8>::null());
    assert!(
        (((((*h.borrow())
            .with(|__v| (*__v)
                .table
                .offset(((0) as isize))
                .clone()
                .with(|__v| (*__v)
                    .c
                    .set()
                    .clone()
                    .with(|__v| (*__v).elem.offset(((0) as isize)).clone())))
            .read())
        .is_null()) as i32)
            != 0)
    );
    libcc2rs::free_refcount(
        (((*h.borrow()).with(|__v| {
            (*__v)
                .table
                .offset(((0) as isize))
                .clone()
                .with(|__v| (*__v).c.set().clone().with(|__v| (*__v).elem.clone()))
        }))
        .clone() as Ptr<Ptr<u8>>)
            .to_any(),
    );
    (*h.borrow()).with(|__v| {
        (*__v).table.offset(((0) as isize)).clone().with_mut(|__v| {
            __v.c
                .set()
                .with_mut(|__v| __v.elem = Ptr::<Ptr<u8>>::null())
        })
    });
    assert!(
        (((((*h.borrow()).with(|__v| (*__v)
            .table
            .offset(((0) as isize))
            .clone()
            .with(|__v| (*__v).c.set().clone().with(|__v| (*__v).elem.clone()))))
        .is_null()) as i32)
            != 0)
    );
    libcc2rs::free_refcount(
        (((*h.borrow()).with(|__v| (*__v).table.clone())).clone() as Ptr<entry>).to_any(),
    );
    libcc2rs::free_refcount(((*h.borrow()).clone() as Ptr<holder>).to_any());
    return 0;
}
