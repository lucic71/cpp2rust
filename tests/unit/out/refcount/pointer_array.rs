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
pub struct StackArray {
    pub arr: Box<[Ptr<i32>]>,
}
impl Clone for StackArray {
    fn clone(&self) -> Self {
        let mut this = Self {
            arr: (self.arr).clone(),
        };
        this
    }
}
impl Default for StackArray {
    fn default() -> Self {
        StackArray {
            arr: (0..3)
                .map(|_| Ptr::<i32>::null())
                .collect::<Box<[Ptr<i32>]>>(),
        }
    }
}
impl ByteRepr for StackArray {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.arr.to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            arr: <Box<[Ptr<i32>]>>::from_bytes(&buf[0..24]),
        }
    }
}
pub fn IncrementAll_0(s: Ptr<StackArray>) {
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        {
            let _ptr = s
                .with(|__v| (*__v).arr[(*i.borrow()) as usize].clone())
                .clone();
            _ptr.write((_ptr.read()) + 1)
        };
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let s: Value<StackArray> = Rc::new(RefCell::new(StackArray {
        arr: Box::new([(x.as_pointer()), (x.as_pointer()), (x.as_pointer())]),
    }));
    ({ IncrementAll_0(s.as_pointer()) });
    return (*x.borrow());
}
