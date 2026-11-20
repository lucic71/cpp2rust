extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Item {
    #[offset(0)]
    pub flags: u8,
}
impl Item {
    pub fn new_1(this: Ptr<Item>) {}
    pub fn copy_from(this: Ptr<Item>, _a0: Ptr<Item>) {
        field!(this, flags).write(_a0.with(|__s| __s.flags));
    }
    pub fn move_from(this: Ptr<Item>, _a0: Ptr<Item>) {
        field!(this, flags).write({ (*_a0.upgrade().deref()).flags });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let item: Value<Item> = Rc::new(RefCell::new(<Item>::uninit()));
    {
        let __d = item.as_pointer();
        field!(__d, flags).write(0_u8);
    };
    let mut ptr: Ptr<Item> = (item.as_pointer());
    field!((ptr), flags).write({ (((ptr).with(|__s| __s.flags) as i32) | (1 << 0)) as u8 });
    field!((ptr), flags).write({ (((ptr).with(|__s| __s.flags) as i32) | (1 << 1)) as u8 });
    assert!(((ptr.with(|__s| __s.flags) as i32) == 3));
    field!((ptr), flags)
        .write({ (((ptr).with(|__s| __s.flags) as i32) & ((!(1 << 0) as u8) as i32)) as u8 });
    assert!(((ptr.with(|__s| __s.flags) as i32) == 2));
    let mut bits: [u8; 4] = [0_u8; 4];
    (bits)[((5) / 8) as usize] =
        { (((bits)[((5) / 8) as usize] as i32) | (((1 << ((5) & 7)) as u8) as i32)) as u8 };
    (bits)[((13) / 8) as usize] =
        { (((bits)[((13) / 8) as usize] as i32) | (((1 << ((13) & 7)) as u8) as i32)) as u8 };
    assert!(((bits[(0) as usize] as i32) == 32));
    assert!(((bits[(1) as usize] as i32) == 32));
    assert!(((bits[(2) as usize] as i32) == 0));
    if ((ptr.with(|__s| __s.flags) as i32) != 0) {
        field!((ptr), flags)
            .write({ (((ptr).with(|__s| __s.flags) as i32) & ((!(1 << 1) as u8) as i32)) as u8 });
    }
    assert!(((ptr.with(|__s| __s.flags) as i32) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
