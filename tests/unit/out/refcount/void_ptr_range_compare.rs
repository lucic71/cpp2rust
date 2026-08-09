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
pub struct region {
    pub start: AnyPtr,
    pub mid: AnyPtr,
    pub end: AnyPtr,
}
impl ByteRepr for region {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.start.to_bytes(&mut buf[0..8]);
        self.mid.to_bytes(&mut buf[8..16]);
        self.end.to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start: <AnyPtr>::from_bytes(&buf[0..8]),
            mid: <AnyPtr>::from_bytes(&buf[8..16]),
            end: <AnyPtr>::from_bytes(&buf[16..24]),
        }
    }
}
pub fn in_low_half_0(r: Ptr<region>, p: AnyPtr) -> i32 {
    let r: Value<Ptr<region>> = Rc::new(RefCell::new(r));
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    return ((((({
        let _lhs = (*p.borrow()).clone();
        (*r.borrow()).with(|__v| _lhs >= (__v.start.clone()))
    }) as i32)
        != 0)
        && ((({
            let _lhs = (*p.borrow()).clone();
            (*r.borrow()).with(|__v| _lhs < (__v.mid.clone()))
        }) as i32)
            != 0)) as i32);
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(64_usize).reinterpret_cast::<u8>(),
    ));
    let r: Value<region> = <Value<region>>::default();
    (*r.borrow_mut()).start = ((*buf.borrow()).clone() as Ptr<u8>).to_any();
    (*r.borrow_mut()).mid = ((*buf.borrow()).offset(((32) as isize)) as Ptr<u8>).to_any();
    (*r.borrow_mut()).end = ((*buf.borrow()).offset(((64) as isize)) as Ptr<u8>).to_any();
    assert!(
        ((({
            let _lhs = ((*r.borrow()).start).clone();
            _lhs < ((*r.borrow()).mid).clone()
        }) as i32)
            != 0)
    );
    assert!(
        ((({
            let _lhs = ((*r.borrow()).mid).clone();
            _lhs < ((*r.borrow()).end).clone()
        }) as i32)
            != 0)
    );
    assert!(
        (({
            in_low_half_0(
                (r.as_pointer()),
                ((*buf.borrow()).offset(((10) as isize)) as Ptr<u8>).to_any(),
            )
        }) != 0)
    );
    assert!(
        ((!(({
            in_low_half_0(
                (r.as_pointer()),
                ((*buf.borrow()).offset(((40) as isize)) as Ptr<u8>).to_any(),
            )
        }) != 0) as i32)
            != 0)
    );
    assert!(
        (({
            in_low_half_0(
                (r.as_pointer()),
                ((*buf.borrow()).clone() as Ptr<u8>).to_any(),
            )
        }) != 0)
    );
    assert!(
        ((!(({
            in_low_half_0(
                (r.as_pointer()),
                ((*buf.borrow()).offset(((32) as isize)) as Ptr<u8>).to_any(),
            )
        }) != 0) as i32)
            != 0)
    );
    let other: Value<Ptr<u8>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(64_usize).reinterpret_cast::<u8>(),
    ));
    let op: Value<AnyPtr> = Rc::new(RefCell::new(
        ((*other.borrow()).clone() as Ptr<u8>).to_any(),
    ));
    assert!(
        ((!(((((({
            let _lhs = (*op.borrow()).clone();
            _lhs >= ((*r.borrow()).start).clone()
        }) as i32)
            != 0)
            && ((({
                let _lhs = (*op.borrow()).clone();
                _lhs < ((*r.borrow()).end).clone()
            }) as i32)
                != 0)) as i32)
            != 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*other.borrow()).clone() as Ptr<u8>).to_any().clone());
    libcc2rs::free_refcount(((*buf.borrow()).clone() as Ptr<u8>).to_any().clone());
    return 0;
}
