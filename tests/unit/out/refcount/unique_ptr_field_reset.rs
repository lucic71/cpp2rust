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
pub struct Wrapper {
    pub single: Option<Value<i32>>,
    pub array: Option<Value<Box<[u8]>>>,
}
impl ByteRepr for Wrapper {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.single.to_bytes(&mut buf[0..8]);
        self.array.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            single: <Option<Value<i32>>>::from_bytes(&buf[0..8]),
            array: <Option<Value<Box<[u8]>>>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn setup_0(w: Ptr<Wrapper>, value: i32) {
    let w: Value<Ptr<Wrapper>> = Rc::new(RefCell::new(w));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    {
        let _p: Ptr<_> = Ptr::alloc((*value.borrow()));
        (*w.borrow()).with_mut(|__v| __v.single = _p.to_owned_opt())
    };
    {
        let __rhs = Ptr::alloc_array(
            (0..((*value.borrow()) as usize))
                .map(|_| <u8>::default())
                .collect::<Box<[u8]>>(),
        )
        .to_owned_opt();
        (*w.borrow()).with_mut(|__v| __v.array = __rhs)
    };
}
pub fn clear_1(w: Ptr<Wrapper>) {
    let w: Value<Ptr<Wrapper>> = Rc::new(RefCell::new(w));
    {
        let _p: Ptr<_> = Ptr::<i32>::null();
        (*w.borrow()).with_mut(|__v| __v.single = _p.to_owned_opt())
    };
    (*w.borrow()).with_mut(|__v| __v.array = None);
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let w: Value<Wrapper> = Rc::new(RefCell::new(<Wrapper>::default()));
    ({ setup_0((w.as_pointer()), 3) });
    {
        let __rhs = ((*(*w.borrow()).single.as_ref().unwrap().borrow()) as u8);
        (*(*w.borrow()).array.as_ref().unwrap().borrow_mut())[(0_usize) as usize] = __rhs
    };
    assert!((((*(*w.borrow()).array.as_ref().unwrap().borrow())[(0_usize) as usize] as i32) == 3));
    ({ clear_1((w.as_pointer())) });
    assert!(((*w.borrow()).single.as_pointer()).is_null());
    return 0;
}
