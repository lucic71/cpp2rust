extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone)]
pub struct pair {
    pub a: Box<[i32]>,
    pub b: Box<[i32]>,
}
impl Default for pair {
    fn default() -> Self {
        pair {
            a: (0..4).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            b: (0..4).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
        }
    }
}
impl ByteRepr for pair {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..16]);
        self.b.to_bytes(&mut buf[16..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <Box<[i32]>>::from_bytes(&buf[0..16]),
            b: <Box<[i32]>>::from_bytes(&buf[16..32]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<pair> = <Value<pair>>::default();
    assert!(
        (((((*s.borrow()).a.as_pointer() as Ptr::<i32>).offset((4) as isize)
            == ((*s.borrow()).b.as_pointer() as Ptr::<i32>)) as i32)
            != 0)
    );
    return 0;
}
