extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive()]
pub struct Pointers {
    pub x1: Ptr<i32>,
    pub x2: Ptr<i32>,
    pub x3: Box<[Ptr<i32>]>,
    pub x4: Box<[Ptr<i32>]>,
    pub x5: i32,
}
impl Clone for Pointers {
    fn clone(&self) -> Self {
        let mut this = Self {
            x1: (self.x1).clone(),
            x2: (self.x2).clone(),
            x3: (self.x3).clone(),
            x4: (self.x4).clone(),
            x5: self.x5,
        };
        this
    }
}
impl Default for Pointers {
    fn default() -> Self {
        Pointers {
            x1: Ptr::<i32>::null(),
            x2: Ptr::<i32>::null(),
            x3: (0..5)
                .map(|_| Ptr::<i32>::null())
                .collect::<Box<[Ptr<i32>]>>(),
            x4: (0..10)
                .map(|_| Ptr::<i32>::null())
                .collect::<Box<[Ptr<i32>]>>(),
            x5: <i32>::default(),
        }
    }
}
impl ByteRepr for Pointers {
    fn byte_size() -> usize {
        144
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x1.to_bytes(&mut buf[0..8]);
        self.x2.to_bytes(&mut buf[8..16]);
        self.x3.to_bytes(&mut buf[16..56]);
        self.x4.to_bytes(&mut buf[56..136]);
        self.x5.to_bytes(&mut buf[136..140]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x1: <Ptr<i32>>::from_bytes(&buf[0..8]),
            x2: <Ptr<i32>>::from_bytes(&buf[8..16]),
            x3: <Box<[Ptr<i32>]>>::from_bytes(&buf[16..56]),
            x4: <Box<[Ptr<i32>]>>::from_bytes(&buf[56..136]),
            x5: <i32>::from_bytes(&buf[136..140]),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let default_pointers: Value<Ptr<Pointers>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..10_usize)
            .map(|_| <Pointers>::default())
            .collect::<Box<[Pointers]>>(),
    )));
    (*default_pointers.borrow()).delete_array();
    return 0;
}
