extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Level0_Level1_1_Level2_1_Level3_1 {
    #[offset(0)]
    pub x1: i32,
}
impl Level0_Level1_1_Level2_1_Level3_1 {
    pub fn copy_from(
        this: Ptr<Level0_Level1_1_Level2_1_Level3_1>,
        _a0: Ptr<Level0_Level1_1_Level2_1_Level3_1>,
    ) {
        field!(this, x1).write(_a0.with(|__s| __s.x1));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Level0_Level1_1_Level2_1_Level3_2 {
    #[offset(0)]
    pub x1: i32,
    #[offset(4)]
    pub x2: i32,
}
impl Level0_Level1_1_Level2_1_Level3_2 {
    pub fn copy_from(
        this: Ptr<Level0_Level1_1_Level2_1_Level3_2>,
        _a0: Ptr<Level0_Level1_1_Level2_1_Level3_2>,
    ) {
        field!(this, x1).write(_a0.with(|__s| __s.x1));
        field!(this, x2).write(_a0.with(|__s| __s.x2));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Level0_Level1_1_Level2_1 {
    #[offset(0)]
    pub x1: i32,
}
impl Level0_Level1_1_Level2_1 {
    pub fn copy_from(this: Ptr<Level0_Level1_1_Level2_1>, _a0: Ptr<Level0_Level1_1_Level2_1>) {
        field!(this, x1).write(_a0.with(|__s| __s.x1));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Level0_Level1_1 {
    #[offset(0)]
    pub x1: i32,
}
impl Level0_Level1_1 {
    pub fn copy_from(this: Ptr<Level0_Level1_1>, _a0: Ptr<Level0_Level1_1>) {
        field!(this, x1).write(_a0.with(|__s| __s.x1));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Level0_Level1_2 {
    #[offset(0)]
    pub x1: i32,
    #[offset(4)]
    pub x2: i32,
}
impl Level0_Level1_2 {
    pub fn copy_from(this: Ptr<Level0_Level1_2>, _a0: Ptr<Level0_Level1_2>) {
        field!(this, x1).write(_a0.with(|__s| __s.x1));
        field!(this, x2).write(_a0.with(|__s| __s.x2));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Level0 {}
impl Level0 {
    pub fn copy_from(this: Ptr<Level0>, _a0: Ptr<Level0>) {}
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<Level0_Level1_1> = Rc::new(RefCell::new(<Level0_Level1_1>::uninit()));
    {
        let __d = x1.as_pointer();
        field!(__d, x1).write(0);
    };
    let x2: Value<Level0_Level1_2> = Rc::new(RefCell::new(<Level0_Level1_2>::uninit()));
    {
        let __d = x2.as_pointer();
        field!(__d, x1).write(1);
        field!(__d, x2).write(2);
    };
    let x3: Value<Level0_Level1_1_Level2_1> =
        Rc::new(RefCell::new(<Level0_Level1_1_Level2_1>::uninit()));
    {
        let __d = x3.as_pointer();
        field!(__d, x1).write(3);
    };
    let x4: Value<Level0_Level1_1_Level2_1_Level3_1> =
        Rc::new(RefCell::new(<Level0_Level1_1_Level2_1_Level3_1>::uninit()));
    {
        let __d = x4.as_pointer();
        field!(__d, x1).write(4);
    };
    let x5: Value<Level0_Level1_1_Level2_1_Level3_2> =
        Rc::new(RefCell::new(<Level0_Level1_1_Level2_1_Level3_2>::uninit()));
    {
        let __d = x5.as_pointer();
        field!(__d, x1).write(5);
        field!(__d, x2).write(6);
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
