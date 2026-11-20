extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Layout {
    #[offset(0)]
    pub a: u8,
    #[offset(4)]
    pub b: u32,
    #[offset(8)]
    pub c: u16,
}
impl Layout {
    pub fn copy_from(this: Ptr<Layout>, _a0: Ptr<Layout>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, b).write(_a0.with(|__s| __s.b));
        field!(this, c).write(_a0.with(|__s| __s.c));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(66)]
pub struct Frame {
    #[offset(0)]
    pub tag: u16,
    #[offset(2)]
    #[byte_size(64)]
    #[array_len(64)]
    pub body: Box<[i8]>,
}
impl Frame {
    pub fn copy_from(this: Ptr<Frame>, _a0: Ptr<Frame>) {
        field!(this, tag).write(_a0.with(|__s| __s.tag));
        field!(this, body).write(Box::new(std::array::from_fn::<_, 64, _>(|__i: usize| {
            (*_a0.upgrade().deref()).body[(__i) as usize]
        })));
    }
}
impl Default for Frame {
    fn default() -> Self {
        {
            Frame {
                tag: 0_u16,
                body: (0..64).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!((0_usize == 0_usize));
    assert!((4_usize == 4_usize));
    assert!((8_usize == 8_usize));
    let v: Value<Layout> = Rc::new(RefCell::new(<Layout>::uninit()));
    {
        let __d = v.as_pointer();
        field!(__d, a).write(0_u8);
        field!(__d, b).write(0_u32);
        field!(__d, c).write(0_u16);
    };
    (*v.borrow_mut()).b = 3735928559_u32;
    let mut base: Ptr<u8> = (v.as_pointer()).reinterpret_cast::<u8>();
    let mut bp: Ptr<u32> = (base.offset((4_usize) as isize)).reinterpret_cast::<u32>();
    assert!(((bp.read()) == 3735928559_u32));
    (base.offset((4_usize) as isize))
        .reinterpret_cast::<u32>()
        .write(305419896_u32);
    assert!(({ ({ (*v.borrow()).b } == 305419896_u32) }));
    let mut text: Ptr<i8> = Ptr::<i8>::from_string_literal(b"example-body");
    let mut len: usize = (text.to_c_string_iterator().count()).wrapping_add(1_usize);
    let mut total: usize = ((2_usize as u64).wrapping_add((len as u64)) as usize);
    assert!((total == (2_usize).wrapping_add(len)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
