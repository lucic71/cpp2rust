extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Clone)]
pub struct holder {
    pub mask: Box<[u8]>,
    pub after: u32,
}
impl Default for holder {
    fn default() -> Self {
        holder {
            mask: (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            after: <u32>::default(),
        }
    }
}
impl ByteRepr for holder {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.mask.to_bytes(&mut buf[0..4]);
        self.after.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mask: <Box<[u8]>>::from_bytes(&buf[0..4]),
            after: <u32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn encode_0(h: Ptr<holder>, out: Ptr<u8>) {
    let h: Value<Ptr<holder>> = Rc::new(RefCell::new(h));
    let out: Value<Ptr<u8>> = Rc::new(RefCell::new(out));
    ((*h.borrow()).field_ptr(
        0,
        |__v: &holder| &__v.mask[..],
        |__v: &mut holder| &mut __v.mask[..],
    ) as Ptr<u8>)
        .reinterpret_cast::<u8>()
        .write(7_u8);
    {
        ((*out.borrow()).clone() as Ptr<u8>).to_any().memcpy(
            &(((*h.borrow()).field_ptr(
                0,
                |__v: &holder| &__v.mask[..],
                |__v: &mut holder| &mut __v.mask[..],
            ) as Ptr<u8>) as Ptr<u8>)
                .to_any(),
            4usize as usize,
        );
        ((*out.borrow()).clone() as Ptr<u8>).to_any().clone()
    };
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let h: Value<holder> = Rc::new(RefCell::new(holder {
        mask: Box::new([1_u8, 2_u8, 3_u8, 4_u8]),
        after: 1432778632_u32,
    }));
    let out: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    ({ encode_0((h.as_pointer()), (out.as_pointer() as Ptr<u8>)) });
    assert!((((((*out.borrow())[(0) as usize] as i32) == 7) as i32) != 0));
    assert!((((((*out.borrow())[(3) as usize] as i32) == 4) as i32) != 0));
    assert!(((((*h.borrow()).after == 1432778632_u32) as i32) != 0));
    return 0;
}
