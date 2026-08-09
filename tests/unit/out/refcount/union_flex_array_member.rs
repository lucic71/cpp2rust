extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn bytes(&self) -> Ptr<u8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn aligner(&self) -> Ptr<AnyPtr> {
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
pub struct node {
    pub len: usize,
    pub pos: usize,
    pub x: anon_0,
}
impl ByteRepr for node {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.len.to_bytes(&mut buf[0..8]);
        self.pos.to_bytes(&mut buf[8..16]);
        self.x.to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            len: <usize>::from_bytes(&buf[0..8]),
            pos: <usize>::from_bytes(&buf[8..16]),
            x: <anon_0>::from_bytes(&buf[16..24]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let tail_size: Value<usize> = Rc::new(RefCell::new(32_usize));
    let n: Value<Ptr<node>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(
            ((24usize as u64).wrapping_add(((*tail_size.borrow()) as u64)) as usize),
        )
        .reinterpret_cast::<node>(),
    ));
    {
        let __rhs = (*tail_size.borrow());
        (*n.borrow()).with_mut(|__v| __v.len = __rhs)
    };
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while (((*i.borrow()) < (*tail_size.borrow())) as i32) != 0 {
        {
            let __rhs = (((*i.borrow()) & 255_usize) as u8);
            (((*n.borrow()).reinterpret_cast::<u8>().offset(16usize) as Ptr<u8>) as Ptr<u8>)
                .offset(((*i.borrow()) as isize))
                .write(__rhs)
        };
        (*i.borrow_mut()).postfix_inc();
    }
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while (((*i.borrow()) < (*tail_size.borrow())) as i32) != 0 {
        assert!(
            ((({
                let _lhs = (((((*n.borrow()).reinterpret_cast::<u8>().offset(16usize) as Ptr<u8>)
                    as Ptr<u8>)
                    .offset(((*i.borrow()) as isize))
                    .read()) as i32);
                _lhs == ((((*i.borrow()) & 255_usize) as u8) as i32)
            }) as i32)
                != 0)
        );
        (*i.borrow_mut()).postfix_inc();
    }
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((((*n.borrow()).reinterpret_cast::<u8>().offset(16usize) as Ptr<u8>) as Ptr<u8>)
            .offset(((10) as isize))),
    ));
    assert!(((((((*p.borrow()).read()) as i32) == 10) as i32) != 0));
    (*p.borrow()).write(170_u8);
    assert!(
        ((((((((*n.borrow()).reinterpret_cast::<u8>().offset(16usize) as Ptr<u8>) as Ptr::<u8>)
            .offset(((10) as isize))
            .read()) as i32)
            == 170) as i32)
            != 0)
    );
    (*n.borrow()).with_mut(|__v| __v.pos = 20_usize);
    let q: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((((*n.borrow()).reinterpret_cast::<u8>().offset(16usize) as Ptr<u8>) as Ptr<u8>)
            .offset((((*n.borrow()).with(|__v| __v.pos)) as isize))),
    ));
    assert!(((((((*q.borrow()).read()) as i32) == 20) as i32) != 0));
    (*q.borrow()).write(187_u8);
    assert!(((((((*q.borrow()).read()) as i32) == 187) as i32) != 0));
    libcc2rs::free_refcount(((*n.borrow()).clone() as Ptr<node>).to_any().clone());
    return 0;
}
