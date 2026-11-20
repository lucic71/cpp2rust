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
pub struct Point {
    #[offset(0)]
    pub x: i32,
}
impl Point {
    pub fn copy_from(this: Ptr<Point>, _a0: Ptr<Point>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_int_ {
    #[offset(0)]
    pub val: i32,
}
impl Box_int_ {
    pub fn copy_from(this: Ptr<Box_int_>, _a0: Ptr<Box_int_>) {
        field!(this, val).write(_a0.with(|__s| __s.val));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_Point_ {
    #[offset(0)]
    #[byte_size(4)]
    pub val: Point,
}
impl Box_Point_ {
    pub fn copy_from(this: Ptr<Box_Point_>, _a0: Ptr<Box_Point_>) {
        Point::copy_from(field_ptr!(this, val), { field_ptr!(_a0, val) });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Deduced_int_ {
    #[offset(0)]
    pub val: i32,
}
impl Deduced_int_ {
    pub fn copy_from(this: Ptr<Deduced_int_>, _a0: Ptr<Deduced_int_>) {
        field!(this, val).write(_a0.with(|__s| __s.val));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Deduced_Point_ {
    #[offset(0)]
    #[byte_size(4)]
    pub val: Point,
}
impl Deduced_Point_ {
    pub fn copy_from(this: Ptr<Deduced_Point_>, _a0: Ptr<Deduced_Point_>) {
        Point::copy_from(field_ptr!(this, val), { field_ptr!(_a0, val) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let i: Value<Box_int_> = Rc::new(RefCell::new(<Box_int_>::uninit()));
    {
        let __d = i.as_pointer();
        field!(__d, val).write(3);
    };
    assert!((({ Box_int_Impl::twice(&i.as_pointer(),) }) == 6));
    let p: Value<Box_Point_> = Rc::new(RefCell::new(<Box_Point_>::uninit()));
    {
        let __d = p.as_pointer();
        {
            let __d = field_ptr!(__d, val);
            field!(__d, x).write(4);
        };
    };
    assert!(
        ({
            Rc::try_unwrap({
                let __v: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
                ({ Box_Point_Impl::get(&p.as_pointer(), __v.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner()
            .x
        } == 4)
    );
    let d: Value<Deduced_int_> = Rc::new(RefCell::new(<Deduced_int_>::uninit()));
    {
        let __d = d.as_pointer();
        field!(__d, val).write(5);
    };
    assert!((({ Deduced_int_Impl::twice(&d.as_pointer(),) }) == 10));
    let dp: Value<Deduced_Point_> = Rc::new(RefCell::new(<Deduced_Point_>::uninit()));
    {
        let __d = dp.as_pointer();
        {
            let __d = field_ptr!(__d, val);
            field!(__d, x).write(6);
        };
    };
    assert!(
        ({
            Rc::try_unwrap({
                let __v: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
                ({ Deduced_Point_Impl::get(&dp.as_pointer(), __v.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner()
            .x
        } == 6)
    );
    return 0;
}
pub trait Box_Point_Impl {
    fn get(&self, __ret: Ptr<Point>);
    fn twice(&self, __ret: Ptr<Point>) {
        unimplemented!()
    }
}
impl Box_Point_Impl for Ptr<Box_Point_> {
    fn get(&self, __ret: Ptr<Point>) {
        Point::copy_from(__ret.clone(), { field_ptr!((*self), val) });
        return;
    }
}
pub trait Box_int_Impl {
    fn get(&self) -> i32 {
        unimplemented!()
    }
    fn twice(&self) -> i32;
}
impl Box_int_Impl for Ptr<Box_int_> {
    fn twice(&self) -> i32 {
        return ((*self).with(|__s| __s.val) + (*self).with(|__s| __s.val));
    }
}
pub trait Deduced_Point_Impl {
    fn get(&self, __ret: Ptr<Point>);
}
impl Deduced_Point_Impl for Ptr<Deduced_Point_> {
    fn get(&self, __ret: Ptr<Point>) {
        Point::copy_from(__ret.clone(), { field_ptr!((*self), val) });
        return;
    }
}
pub trait Deduced_int_Impl {
    fn twice(&self) -> i32;
}
impl Deduced_int_Impl for Ptr<Deduced_int_> {
    fn twice(&self) -> i32 {
        return ((*self).with(|__s| __s.val) + (*self).with(|__s| __s.val));
    }
}
pub fn __cpp2rust_init_globals() {}
