extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, MoveAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub v: i32,
}
impl MoveOnly {
    pub fn new(this: Ptr<MoveOnly>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn move_from(this: Ptr<MoveOnly>, o: Ptr<MoveOnly>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(o, v).write(0);
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg)]
#[byte_size(4)]
pub struct ConstMoveAssign {
    #[offset(0)]
    pub mark: i32,
}
impl ConstMoveAssign {
    pub fn new(this: Ptr<ConstMoveAssign>) {
        field!(this, mark).write(0);
    }
}
impl Default for ConstMoveAssign {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<ConstMoveAssign> = Rc::new(RefCell::new(<ConstMoveAssign>::uninit()));
            ConstMoveAssign::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn make_0(__ret: Ptr<MoveOnly>, mut v: i32) {
    let m: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::new(m.as_pointer(), { v });
    MoveOnly::move_from(__ret.clone(), { m.as_pointer() });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::new(a.as_pointer(), { 1 });
    let b: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::new(b.as_pointer(), { 2 });
    let c: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::new(c.as_pointer(), { 3 });
    ({ MoveOnlyImpl::move_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 2) }));
    assert!(({ ({ (*b.borrow()).v } == 0) }));
    ({
        let _o: Value<MoveOnly> = {
            let __v: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
            MoveOnly::new(__v.as_pointer(), { 3 });
            __v
        };
        MoveOnlyImpl::move_assign(&b.as_pointer(), _o.as_pointer())
    });
    ({
        MoveOnlyImpl::move_assign(
            &c.as_pointer(),
            ({ MoveOnlyImpl::move_assign(&a.as_pointer(), b.as_pointer()) }),
        )
    });
    assert!(
        ({
            (({ (*b.borrow()).v } == 0) && ({ (*a.borrow()).v } == 0)) && ({ (*c.borrow()).v } == 3)
        })
    );
    ({
        let _o: Value<MoveOnly> = {
            let __v: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
            MoveOnly::new(__v.as_pointer(), { 5 });
            __v
        };
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o.as_pointer())
    });
    assert!(({ ({ (*a.borrow()).v } == 5) }));
    ({
        let _o: Value<MoveOnly> = {
            let __v: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
            ({ make_0(__v.as_pointer(), 6) });
            __v
        };
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o.as_pointer())
    });
    assert!(({ ({ (*a.borrow()).v } == 6) }));
    ({
        let _o: Ptr<MoveOnly> = a.as_pointer();
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o)
    });
    assert!(({ ({ (*a.borrow()).v } == 6) }));
    let vec_: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = Rc::try_unwrap({
            let __v: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
            MoveOnly::new(__v.as_pointer(), { 7 });
            __v
        })
        .ok()
        .unwrap()
        .into_inner();
        (*vec_.borrow_mut()).push(__a1)
    };
    let d: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::new(d.as_pointer(), { 8 });
    ({ MoveOnlyImpl::move_assign(&vec_.decay().offset(0_usize), d.as_pointer()) });
    assert!(({ ({ (*vec_.borrow())[0_usize].v } == 8) }));
    assert!(({ ({ (*d.borrow()).v } == 0) }));
    let m: Value<ConstMoveAssign> = Rc::new(RefCell::new(<ConstMoveAssign>::uninit()));
    ConstMoveAssign::new(m.as_pointer());
    let m1: Value<ConstMoveAssign> = Rc::new(RefCell::new(<ConstMoveAssign>::uninit()));
    ConstMoveAssign::new(m1.as_pointer());
    let m2: Value<ConstMoveAssign> = Rc::new(RefCell::new(<ConstMoveAssign>::uninit()));
    ConstMoveAssign::new(m2.as_pointer());
    let cm: Value<ConstMoveAssign> = Rc::new(RefCell::new(<ConstMoveAssign>::uninit()));
    ConstMoveAssign::new(cm.as_pointer());
    ({ ConstMoveAssignImpl::operator_assign_2(&m1.as_pointer(), m.as_pointer()) });
    ({ ConstMoveAssignImpl::operator_assign_3(&m2.as_pointer(), cm.as_pointer()) });
    assert!(({ ({ (*m1.borrow()).mark } == 1) }));
    assert!(({ ({ (*m2.borrow()).mark } == 10) }));
    return 0;
}
pub trait ConstMoveAssignImpl {
    fn operator_assign_2(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
    fn operator_assign_3(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
}
impl ConstMoveAssignImpl for Ptr<ConstMoveAssign> {
    fn operator_assign_2(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1) });
        return (*self).clone();
    }
    fn operator_assign_3(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 10) });
        return (*self).clone();
    }
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, o: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, o: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
