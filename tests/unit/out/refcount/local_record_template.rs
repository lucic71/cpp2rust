extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(t: Value<Local_1>) -> i32 {
    return ({ (*t.borrow()).x } as i32);
}
pub fn get_2(t: Value<Local_3>) -> i32 {
    return { (*t.borrow()).x };
}
pub fn get_4(t: Value<Local_5>) -> i32 {
    return { (*t.borrow()).x };
}
pub fn get_6(t: Value<Local_7>) -> i32 {
    return ({ (*t.borrow()).x } as i32);
}
pub fn twice_8(t: Value<Local_3>) -> i32 {
    return ({ (*t.borrow()).x } * 2);
}
pub fn wrap_9(mut v: i32) -> i32 {
    let l: Value<Local_5> = Rc::new(RefCell::new(<Local_5>::uninit()));
    {
        let __d = l.as_pointer();
        field!(__d, x).write(v);
    };
    return ({
        get_4({
            let __v: Value<Local_5> = Rc::new(RefCell::new(<Local_5>::uninit()));
            Local_5::copy_from(__v.as_pointer(), { l.as_pointer() });
            __v
        })
    });
}
pub fn wrap_10(mut v: i64) -> i32 {
    let l: Value<Local_7> = Rc::new(RefCell::new(<Local_7>::uninit()));
    {
        let __d = l.as_pointer();
        field!(__d, x).write(v);
    };
    return ({
        get_6({
            let __v: Value<Local_7> = Rc::new(RefCell::new(<Local_7>::uninit()));
            Local_7::copy_from(__v.as_pointer(), { l.as_pointer() });
            __v
        })
    });
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_5 {
    #[offset(0)]
    pub x: i32,
}
impl Local_5 {
    pub fn new_1(this: Ptr<Local_5>) {}
    pub fn copy_from(this: Ptr<Local_5>, _a0: Ptr<Local_5>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
    pub fn move_from(this: Ptr<Local_5>, _a0: Ptr<Local_5>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Local_7 {
    #[offset(0)]
    pub x: i64,
}
impl Local_7 {
    pub fn new_1(this: Ptr<Local_7>) {}
    pub fn copy_from(this: Ptr<Local_7>, _a0: Ptr<Local_7>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
    pub fn move_from(this: Ptr<Local_7>, _a0: Ptr<Local_7>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
    }
}
pub fn other_11() -> i32 {
    let l: Value<Local_1> = Rc::new(RefCell::new(<Local_1>::uninit()));
    {
        let __d = l.as_pointer();
        field!(__d, x).write(3_i64);
        field!(__d, y).write(4_i64);
    };
    return (({
        get_0({
            let __v: Value<Local_1> = Rc::new(RefCell::new(<Local_1>::uninit()));
            Local_1::copy_from(__v.as_pointer(), { l.as_pointer() });
            __v
        })
    }) + ({ (*l.borrow()).y } as i32));
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Local_1 {
    #[offset(0)]
    pub x: i64,
    #[offset(8)]
    pub y: i64,
}
impl Local_1 {
    pub fn new_1(this: Ptr<Local_1>) {}
    pub fn copy_from(this: Ptr<Local_1>, _a0: Ptr<Local_1>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
    pub fn move_from(this: Ptr<Local_1>, _a0: Ptr<Local_1>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, y).write({ (*_a0.upgrade().deref()).y });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let l: Value<Local_3> = Rc::new(RefCell::new(<Local_3>::uninit()));
    {
        let __d = l.as_pointer();
        field!(__d, x).write(7);
    };
    assert!(
        (({
            get_2({
                let __v: Value<Local_3> = Rc::new(RefCell::new(<Local_3>::uninit()));
                Local_3::copy_from(__v.as_pointer(), { l.as_pointer() });
                __v
            })
        }) == 7)
    );
    assert!(
        (({
            twice_8({
                let __v: Value<Local_3> = Rc::new(RefCell::new(<Local_3>::uninit()));
                Local_3::copy_from(__v.as_pointer(), { l.as_pointer() });
                __v
            })
        }) == 14)
    );
    assert!((({ other_11() }) == 7));
    assert!((({ wrap_9(5,) }) == 5));
    assert!((({ wrap_10(6_i64,) }) == 6));
    return 0;
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_3 {
    #[offset(0)]
    pub x: i32,
}
impl Local_3 {
    pub fn new_1(this: Ptr<Local_3>) {}
    pub fn copy_from(this: Ptr<Local_3>, _a0: Ptr<Local_3>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
    pub fn move_from(this: Ptr<Local_3>, _a0: Ptr<Local_3>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
    }
}
pub fn __cpp2rust_init_globals() {}
