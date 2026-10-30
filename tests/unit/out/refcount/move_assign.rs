extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub v: (),
}
impl MoveOnly {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn move_from(o: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0_());
        __this
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct ConstMoveAssign {
    #[offset(0)]
    pub mark: (),
}
impl ConstMoveAssign {
    pub fn new() -> Self {
        Self { mark: 0_() }
    }
}
impl Default for ConstMoveAssign {
    fn default() -> Self {
        { ConstMoveAssign::new() }
    }
}
pub fn make_0(mut v: ()) -> MoveOnly {
    let m: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ v })));
    return MoveOnly::move_from({ m.as_pointer() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 1_() })));
    let b: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 2_() })));
    let c: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 3_() })));
    ({ MoveOnlyImpl::move_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    unimplemented!();
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 3_() })));
        MoveOnlyImpl::move_assign(&b.as_pointer(), _o.as_pointer())
    });
    ({
        MoveOnlyImpl::move_assign(
            &c.as_pointer(),
            ({ MoveOnlyImpl::move_assign(&a.as_pointer(), b.as_pointer()) }),
        )
    });
    unimplemented!();
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 5_() })));
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o.as_pointer())
    });
    unimplemented!();
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(({ make_0(6_()) })));
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o.as_pointer())
    });
    unimplemented!();
    ({
        let _o: Ptr<MoveOnly> = a.as_pointer();
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o)
    });
    unimplemented!();
    let mut vec_: () = unimplemented!();
    unimplemented!();
    let d: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 8_() })));
    ({ MoveOnlyImpl::move_assign(&unimplemented!(), d.as_pointer()) });
    unimplemented!();
    unimplemented!();
    let m: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    let m1: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    let m2: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    let cm: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    ({ ConstMoveAssignImpl::operator_assign_2(&m1.as_pointer(), m.as_pointer()) });
    ({ ConstMoveAssignImpl::operator_assign_3(&m2.as_pointer(), cm.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait ConstMoveAssignImpl {
    fn operator_assign_2(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
    fn operator_assign_3(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
}
impl ConstMoveAssignImpl for Ptr<ConstMoveAssign> {
    fn operator_assign_2(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1_()) });
        return (*self).clone();
    }
    fn operator_assign_3(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 10_()) });
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
        field!(o, v).write(0_());
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
