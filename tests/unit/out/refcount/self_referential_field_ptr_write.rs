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
pub struct Inner {
    pub value: i32,
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.value.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct Outer {
    pub slots: Box<[Inner]>,
    pub cur: Ptr<Inner>,
}
impl Default for Outer {
    fn default() -> Self {
        Outer {
            slots: (0..2).map(|_| <Inner>::default()).collect::<Box<[Inner]>>(),
            cur: Ptr::<Inner>::null(),
        }
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.slots.to_bytes(&mut buf[0..8]);
        self.cur.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            slots: <Box<[Inner]>>::from_bytes(&buf[0..8]),
            cur: <Ptr<Inner>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn set_current_0(p: Ptr<Outer>, src: Ptr<i32>) {
    let p: Value<Ptr<Outer>> = Rc::new(RefCell::new(p));
    let src: Value<Ptr<i32>> = Rc::new(RefCell::new(src));
    {
        let __rhs = ((*src.borrow()).read());
        {
            let __obj = (*p.borrow()).with(|__v| __v.cur.clone());
            __obj.with_mut(|__v| __v.value = __rhs)
        }
    };
}
pub fn bump_current_1(p: Ptr<Outer>) {
    let p: Value<Ptr<Outer>> = Rc::new(RefCell::new(p));
    {
        let __rhs = ((*p.borrow()).with(|__v| __v.slots[(0) as usize].value) + 1);
        {
            let __obj = (*p.borrow()).with(|__v| __v.cur.clone());
            __obj.with_mut(|__v| __v.value = __rhs)
        }
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<Outer>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(16usize).reinterpret_cast::<Outer>(),
    ));
    let a: Value<i32> = Rc::new(RefCell::new(7));
    let b: Value<i32> = Rc::new(RefCell::new(8));
    (*p.borrow()).with_mut(|__v| __v.slots[(0) as usize].value = 1);
    (*p.borrow()).with_mut(|__v| __v.slots[(1) as usize].value = 2);
    {
        let __rhs = (((*p.borrow()).field_ptr(
            0,
            |__v: &Outer| &__v.slots[..],
            |__v: &mut Outer| &mut __v.slots[..],
        ) as Ptr<Inner>)
            .offset(0));
        (*p.borrow()).with_mut(|__v| __v.cur = __rhs)
    };
    ({ set_current_0((*p.borrow()).clone(), (a.as_pointer())) });
    assert!(((((*p.borrow()).with(|__v| __v.slots[(0) as usize].value) == 7) as i32) != 0));
    {
        let __rhs = (((*p.borrow()).field_ptr(
            0,
            |__v: &Outer| &__v.slots[..],
            |__v: &mut Outer| &mut __v.slots[..],
        ) as Ptr<Inner>)
            .offset(1));
        (*p.borrow()).with_mut(|__v| __v.cur = __rhs)
    };
    ({ set_current_0((*p.borrow()).clone(), (b.as_pointer())) });
    assert!(((((*p.borrow()).with(|__v| __v.slots[(1) as usize].value) == 8) as i32) != 0));
    ({ bump_current_1((*p.borrow()).clone()) });
    assert!(((((*p.borrow()).with(|__v| __v.slots[(1) as usize].value) == 8) as i32) != 0));
    libcc2rs::free_refcount(((*p.borrow()).clone() as Ptr<Outer>).to_any().clone());
    return 0;
}
