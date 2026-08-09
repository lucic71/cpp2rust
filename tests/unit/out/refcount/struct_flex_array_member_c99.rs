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
pub struct entry {
    pub id: i32,
    pub weight: i32,
}
impl ByteRepr for entry {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.id.to_bytes(&mut buf[0..4]);
        self.weight.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            id: <i32>::from_bytes(&buf[0..4]),
            weight: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct table {
    pub n: i32,
    pub a: Box<[entry]>,
}
impl Default for table {
    fn default() -> Self {
        table {
            n: <i32>::default(),
            a: <Box<[entry]>>::default(),
        }
    }
}
impl ByteRepr for table {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.n.to_bytes(&mut buf[0..4]);
        self.a.to_bytes(&mut buf[4..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: <i32>::from_bytes(&buf[0..4]),
            a: <Box<[entry]>>::from_bytes(&buf[4..4]),
        }
    }
}
pub fn table_create_0(n: i32) -> Ptr<table> {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let raw_: Value<AnyPtr> = Rc::new(RefCell::new(libcc2rs::malloc_refcount(
        (4_usize as usize).wrapping_add(
            ((*n.borrow()) as usize).wrapping_mul((::std::mem::size_of::<entry>() as usize)),
        ),
    )));
    let t: Value<Ptr<table>> = Rc::new(RefCell::new((*raw_.borrow()).reinterpret_cast::<table>()));
    {
        let __rhs = (*n.borrow());
        (*t.borrow()).with_mut(|__v| __v.n = __rhs)
    };
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < (*n.borrow())) as i32) != 0) {
        {
            let __rhs = ((*i.borrow()) * 10);
            ((*t.borrow())
                .reinterpret_cast::<u8>()
                .offset(4usize)
                .reinterpret_cast::<entry>() as Ptr<entry>)
                .offset(((*i.borrow()) as isize))
                .with_mut(|__v| __v.id = __rhs)
        };
        {
            let __rhs = ((*i.borrow()) + 1);
            ((*t.borrow())
                .reinterpret_cast::<u8>()
                .offset(4usize)
                .reinterpret_cast::<entry>() as Ptr<entry>)
                .offset(((*i.borrow()) as isize))
                .with_mut(|__v| __v.weight = __rhs)
        };
        (*i.borrow_mut()).postfix_inc();
    }
    return (*t.borrow()).clone();
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((4usize == 4_usize) as i32) != 0));
    let t: Value<Ptr<table>> = Rc::new(RefCell::new(({ table_create_0(3) })));
    assert!(((((*t.borrow()).with(|__v| __v.n == 3)) as i32) != 0));
    assert!(
        (((((*t.borrow())
            .reinterpret_cast::<u8>()
            .offset(4usize)
            .reinterpret_cast::<entry>() as Ptr<entry>)
            .offset(((0) as isize))
            .with(|__v| __v.id == 0)) as i32)
            != 0)
    );
    assert!(
        (((((*t.borrow())
            .reinterpret_cast::<u8>()
            .offset(4usize)
            .reinterpret_cast::<entry>() as Ptr<entry>)
            .offset(((2) as isize))
            .with(|__v| __v.id == 20)) as i32)
            != 0)
    );
    assert!(
        (((((*t.borrow())
            .reinterpret_cast::<u8>()
            .offset(4usize)
            .reinterpret_cast::<entry>() as Ptr<entry>)
            .offset(((2) as isize))
            .with(|__v| __v.weight == 3)) as i32)
            != 0)
    );
    ((*t.borrow())
        .reinterpret_cast::<u8>()
        .offset(4usize)
        .reinterpret_cast::<entry>() as Ptr<entry>)
        .offset(((1) as isize))
        .with_mut(|__v| __v.id = 99);
    assert!(
        (((((*t.borrow())
            .reinterpret_cast::<u8>()
            .offset(4usize)
            .reinterpret_cast::<entry>() as Ptr<entry>)
            .offset(((1) as isize))
            .with(|__v| __v.id == 99)) as i32)
            != 0)
    );
    assert!(
        (((((*t.borrow())
            .reinterpret_cast::<u8>()
            .offset(4usize)
            .reinterpret_cast::<entry>() as Ptr<entry>)
            .offset(((0) as isize))
            .with(|__v| __v.id == 0)) as i32)
            != 0)
    );
    let next: Value<Ptr<table>> = Rc::new(RefCell::new(Ptr::<table>::null()));
    assert!(((((*next.borrow()).is_null()) as i32) != 0));
    libcc2rs::free_refcount(((*t.borrow()).clone() as Ptr<table>).to_any().clone());
    return 0;
}
