extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static moves_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct Movable {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Movable {
    pub fn new(this: Ptr<Movable>, mut x: i32, mut y: i32) {
        field!(this, x).write(x);
        field!(this, y).write(y);
    }
    pub fn copy_from(this: Ptr<Movable>, other: Ptr<Movable>) {
        field!(this, x).write(other.with(|__s| __s.x));
        field!(this, y).write(other.with(|__s| __s.y));
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn move_from(this: Ptr<Movable>, other: Ptr<Movable>) {
        field!(this, x).write(other.with(|__s| __s.x));
        field!(this, y).write(other.with(|__s| __s.y));
        field!(other, x).write(0);
        field!(other, y).write(0);
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for Movable {
    fn clone(&self) -> Self {
        let __src: Value<Movable> = Rc::new(RefCell::new(Movable {
            x: self.x.clone(),
            y: self.y.clone(),
        }));
        let __dst: Value<Movable> = Rc::new(RefCell::new(<Movable>::uninit()));
        Movable::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub xs: Vec<i32>,
    #[offset(24)]
    #[byte_size(24)]
    pub ys: Vec<i32>,
}
impl Holder {
    pub fn copy_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        field!(this, xs).write((*_a0.upgrade().deref()).xs.clone());
        field!(this, ys).write((*_a0.upgrade().deref()).ys.clone());
    }
    pub fn move_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        field!(this, xs).write(field!(_a0, xs).with_mut(std::mem::take));
        field!(this, ys).write(field!(_a0, ys).with_mut(std::mem::take));
    }
    pub fn new_1(this: Ptr<Holder>) {
        field!(this, xs).write(Vec::new());
        field!(this, ys).write(Vec::new());
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<Movable> = Rc::new(RefCell::new(<Movable>::uninit()));
    Movable::new(m.as_pointer(), { 3 }, { 4 });
    let __decomp_2: Value<Movable> = {
        let __v: Value<Movable> = Rc::new(RefCell::new(<Movable>::uninit()));
        Movable::move_from(__v.as_pointer(), { m.as_pointer() });
        __v
    };
    assert!((moves_1.with(|rc| *rc.borrow()) == 1));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    assert!(({ ({ (*__decomp_2.borrow()).x } == 3) }));
    assert!(({ ({ (*__decomp_2.borrow()).y } == 4) }));
    assert!(({ ({ (*m.borrow()).x } == 0) }));
    assert!(({ ({ (*m.borrow()).y } == 0) }));
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    {
        let __d = h.as_pointer();
        field!(__d, xs).write(Vec::from([1, 2, 3]));
        field!(__d, ys).write(Vec::from([4, 5]));
    };
    let __decomp_3: Value<Holder> = {
        let __v: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
        ({ HolderImpl::extract(&h.as_pointer(), __v.as_pointer()) });
        __v
    };
    assert!(({ ((*__decomp_3.borrow()).xs.len() == 3_usize) }));
    assert!(({ ((*__decomp_3.borrow()).ys.len() == 2_usize) }));
    assert!(((elem!(field_ref!(__decomp_3.as_pointer(), xs).decay(), 2_usize).read()) == 3));
    assert!(((elem!(field_ref!(__decomp_3.as_pointer(), ys).decay(), 0_usize).read()) == 4));
    assert!(({ (*h.borrow()).xs.is_empty() }));
    assert!(({ (*h.borrow()).ys.is_empty() }));
    return 0;
}
pub trait HolderImpl {
    fn extract(&self, __ret: Ptr<Holder>);
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn extract(&self, __ret: Ptr<Holder>) {
        {
            let __d = __ret.clone();
            field!(__d, xs).write(field!((*self), xs).with_mut(std::mem::take));
            field!(__d, ys).write(field!((*self), ys).with_mut(std::mem::take));
        };
        return;
    }
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        field!((*self), xs).with_mut(|__b0| {
            (*__b0) = field!(_a0, xs).with_mut(std::mem::take);
        });
        field!((*self), ys).with_mut(|__b0| {
            (*__b0) = field!(_a0, ys).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
