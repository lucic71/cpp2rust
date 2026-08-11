extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn append_0(out: Ptr<Vec<i32>>, v: i32) {
    let out: Value<Ptr<Vec<i32>>> = Rc::new(RefCell::new(out));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    {
        let a0_clone = (*v.borrow()).clone();
        (*out.borrow()).with_mut(|__v: &mut Vec<i32>| __v.push(a0_clone))
    };
}
#[repr(C)]
#[derive()]
pub struct Setup {
    pub size: i32,
    pub values: Vec<i32>,
}
impl Setup {
    pub fn Setup() -> Self {
        let this: Value<Self> = Rc::new(RefCell::new(Self {
            size: 0,
            values: Vec::new(),
        }));
        ({ this.as_pointer().init() });
        ({
            append_0(
                (this.as_pointer().field_ptr(
                    8,
                    |__v: &Setup| ::std::slice::from_ref(&__v.values),
                    |__v: &mut Setup| ::std::slice::from_mut(&mut __v.values),
                )),
                7,
            )
        });
        Rc::try_unwrap(this).ok().unwrap().into_inner()
    }
}
pub trait SetupMethods {
    fn init(&self);
}
impl Clone for Setup {
    fn clone(&self) -> Self {
        let mut this = Self {
            size: self.size,
            values: (self.values).clone(),
        };
        this
    }
}
impl Default for Setup {
    fn default() -> Self {
        { Setup::Setup() }
    }
}
impl ByteRepr for Setup {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.size.to_bytes(&mut buf[0..4]);
        self.values.to_bytes(&mut buf[8..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            size: <i32>::from_bytes(&buf[0..4]),
            values: <Vec<i32>>::from_bytes(&buf[8..32]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let s: Value<Setup> = Rc::new(RefCell::new(Setup::Setup()));
    assert!(((*s.borrow()).size == 3));
    assert!(((*s.borrow()).values.len() == 1_usize));
    assert!(
        (((s.as_pointer().field_ptr(
            8,
            |__v: &Setup| &__v.values[..],
            |__v: &mut Setup| &mut __v.values[..]
        ) as Ptr<i32>)
            .offset(0_usize)
            .read())
            == 7)
    );
    return 0;
}
impl SetupMethods for Ptr<Setup> {
    fn init(&self) {
        self.with_mut(|__v| __v.size = 3);
    }
}
