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
pub struct inner {
    pub x: i32,
    pub y: i32,
}
impl ByteRepr for inner {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct outer {
    pub in_: inner,
    pub total: i32,
}
impl ByteRepr for outer {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.in_.to_bytes(&mut buf[0..8]);
        self.total.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            in_: <inner>::from_bytes(&buf[0..8]),
            total: <i32>::from_bytes(&buf[8..12]),
        }
    }
}
pub fn read_total_0(o: Ptr<outer>) -> i32 {
    let o: Value<Ptr<outer>> = Rc::new(RefCell::new(o));
    return (*o.borrow()).with(|__v| (*__v).total);
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let o: Value<outer> = Rc::new(RefCell::new(outer {
        in_: inner { x: 1, y: 2 },
        total: 10,
    }));
    let p: Value<Ptr<outer>> = Rc::new(RefCell::new((o.as_pointer())));
    let q: Value<Ptr<outer>> = Rc::new(RefCell::new((o.as_pointer())));
    {
        let __rhs = {
            let _lhs = (*q.borrow()).with(|__v| (*__v).in_.x);
            _lhs + (*q.borrow()).with(|__v| (*__v).in_.y)
        };
        (*p.borrow()).with_mut(|__v| __v.total = __rhs)
    };
    assert!(((((*o.borrow()).total == 3) as i32) != 0));
    let ip: Value<Ptr<inner>> = Rc::new(RefCell::new(
        ((*p.borrow()).field_ptr(
            0,
            |__v: &outer| ::std::slice::from_ref(&__v.in_),
            |__v: &mut outer| ::std::slice::from_mut(&mut __v.in_),
        )),
    ));
    {
        let __rhs = ((*p.borrow()).with(|__v| (*__v).total) + 1);
        (*ip.borrow()).with_mut(|__v| __v.x = __rhs)
    };
    assert!(((((*o.borrow()).in_.x == 4) as i32) != 0));
    {
        let __rhs = (*q.borrow()).with(|__v| (*__v).in_.x);
        (*p.borrow()).with_mut(|__v| __v.total += __rhs)
    };
    assert!(((((*o.borrow()).total == 7) as i32) != 0));
    {
        let __rhs = ({ read_total_0((*q.borrow()).clone()) });
        (*p.borrow()).with_mut(|__v| __v.in_.y = __rhs)
    };
    assert!(((((*o.borrow()).in_.y == 7) as i32) != 0));
    let h: Value<Ptr<outer>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(::std::mem::size_of::<outer>()).reinterpret_cast::<outer>(),
    ));
    let ha: Value<Ptr<outer>> = Rc::new(RefCell::new((*h.borrow()).clone()));
    (*h.borrow()).with_mut(|__v| __v.total = 5);
    (*h.borrow()).with_mut(|__v| __v.in_.x = 1);
    {
        let __rhs = {
            let _lhs = (*h.borrow()).with(|__v| (*__v).total);
            _lhs + (*ha.borrow()).with(|__v| (*__v).in_.x)
        };
        (*ha.borrow()).with_mut(|__v| __v.total = __rhs)
    };
    assert!(((((*h.borrow()).with(|__v| (*__v).total) == 6) as i32) != 0));
    libcc2rs::free_refcount(((*h.borrow()).clone() as Ptr<outer>).to_any().clone());
    return 0;
}
