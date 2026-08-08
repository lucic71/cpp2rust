extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct S {
    pub parts: Vec<Value<Vec<i32>>>,
    pub a: i32,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {
            parts: self
                .parts
                .iter()
                .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
                .collect(),
            a: self.a,
        };
        this
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.parts.to_bytes(&mut buf[0..24]);
        self.a.to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            parts: <Vec<Value<Vec<i32>>>>::from_bytes(&buf[0..24]),
            a: <i32>::from_bytes(&buf[24..28]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*s.borrow_mut()).a = 0;
    {
        let _a0 = 3_usize as usize;
        (s.as_pointer().field_ptr(
            0,
            |__v: &S| ::std::slice::from_ref(&__v.parts),
            |__v: &mut S| ::std::slice::from_mut(&mut __v.parts),
        ) as Ptr<Vec<Value<Vec<i32>>>>)
            .with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
                __v.resize_with(_a0, <Value<Vec<i32>>>::default)
            })
    };
    {
        let __a0 = 2_usize as usize;
        (s.as_pointer().field_ptr(
            0,
            |__v: &S| &__v.parts[..],
            |__v: &mut S| &mut __v.parts[..],
        ) as Ptr<Value<Vec<i32>>>)
            .offset(2_usize)
            .with_mut(|__v: &mut Value<Vec<i32>>| {
                (*__v.borrow_mut()).resize_with(__a0, || <i32>::default())
            })
    };
    let points: Value<i32> = Rc::new(RefCell::new(0));
    let p: Value<Ptr<S>> = Rc::new(RefCell::new((s.as_pointer())));
    'loop_: for mut part in ((*p.borrow()).field_ptr(
        0,
        |__v: &S| &__v.parts[..],
        |__v: &mut S| &mut __v.parts[..],
    ) as Ptr<Value<Vec<i32>>>)
    {
        let part: Ptr<Vec<i32>> = (part.read()).as_pointer();
        {
            let rhs_0 = ((((*points.borrow()) as usize).wrapping_add((part.read()).len())) as i32);
            (*points.borrow_mut()) = rhs_0
        };
        (*s.borrow_mut()).a.postfix_inc();
    }
    assert!(((*s.borrow()).a == 3));
    assert!(((*points.borrow()) == 2));
    return 0;
}
