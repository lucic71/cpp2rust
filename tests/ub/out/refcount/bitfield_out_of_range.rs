extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C, align(4))]
#[derive(Clone, Default)]
#[bitfields(__bits_0 { b: u32 @ 0..3 unsigned, w: u32 @ 3..15 unsigned, s: i32 @ 15..18 signed })]
pub struct bits {
    pub __bits_0: [u8; 3],
}
impl ByteRepr for bits {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf[0..3].copy_from_slice(&self.__bits_0);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            __bits_0: buf[0..3].try_into().unwrap(),
        }
    }
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let nine: Value<i32> = Rc::new(RefCell::new(9));
    let big: Value<i32> = Rc::new(RefCell::new(4660));
    let seven: Value<i32> = Rc::new(RefCell::new(7));
    let v: Value<bits> = <Value<bits>>::default();
    (*v.borrow_mut()).set_b(0_u32);
    (*v.borrow_mut()).set_w(0_u32);
    (*v.borrow_mut()).set_s(0);
    {
        let __bf_v = ((*nine.borrow()) as u32);
        (*v.borrow_mut()).set_b(__bf_v)
    };
    assert!((((((*v.borrow_mut()).b() as i32) == 1) as i32) != 0));
    (*v.borrow_mut()).set_b(7_u32);
    {
        let __bf_old = (*v.borrow_mut()).b();
        {
            let __bf_v = (((__bf_old as i32) + ((1) as i32)) as u32);
            (*v.borrow_mut()).set_b(__bf_v)
        };
        __bf_old
    };
    assert!((((((*v.borrow_mut()).b() as i32) == 0) as i32) != 0));
    (*v.borrow_mut()).set_b(0_u32);
    {
        let __bf_old = (*v.borrow_mut()).b();
        {
            let __bf_v = (((__bf_old as i32) - ((1) as i32)) as u32);
            (*v.borrow_mut()).set_b(__bf_v)
        };
        __bf_old
    };
    assert!((((((*v.borrow_mut()).b() as i32) == 7) as i32) != 0));
    {
        let __bf_v = ((*big.borrow()) as u32);
        (*v.borrow_mut()).set_w(__bf_v)
    };
    assert!((((((*v.borrow_mut()).w() as i32) == 564) as i32) != 0));
    {
        let __bf_v = (*seven.borrow());
        (*v.borrow_mut()).set_s(__bf_v)
    };
    assert!(((((*v.borrow_mut()).s() == -1_i32) as i32) != 0));
    (*v.borrow_mut()).set_s(3);
    {
        let __bf_old = (*v.borrow_mut()).s();
        {
            let __bf_v = (((__bf_old as i32) + ((1) as i32)) as i32);
            (*v.borrow_mut()).set_s(__bf_v)
        };
        __bf_old
    };
    assert!(((((*v.borrow_mut()).s() == -4_i32) as i32) != 0));
    {
        let __bf_v = -4_i32;
        (*v.borrow_mut()).set_s(__bf_v)
    };
    {
        let __bf_old = (*v.borrow_mut()).s();
        {
            let __bf_v = (((__bf_old as i32) - ((1) as i32)) as i32);
            (*v.borrow_mut()).set_s(__bf_v)
        };
        __bf_old
    };
    assert!(((((*v.borrow_mut()).s() == 3) as i32) != 0));
    return 0;
}
