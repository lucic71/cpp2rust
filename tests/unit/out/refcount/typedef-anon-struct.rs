extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_RunInfo {
    #[offset(0)]
    pub block_idx: i32,
    #[offset(4)]
    pub num_extra_zero_runs: i32,
}
impl Outer_RunInfo {
    pub fn new_1(this: Ptr<Outer_RunInfo>) {}
    pub fn copy_from(this: Ptr<Outer_RunInfo>, _a0: Ptr<Outer_RunInfo>) {
        field!(this, block_idx).write(_a0.with(|__s| __s.block_idx));
        field!(this, num_extra_zero_runs).write(_a0.with(|__s| __s.num_extra_zero_runs));
    }
    pub fn move_from(this: Ptr<Outer_RunInfo>, _a0: Ptr<Outer_RunInfo>) {
        field!(this, block_idx).write({ (*_a0.upgrade().deref()).block_idx });
        field!(this, num_extra_zero_runs).write({ (*_a0.upgrade().deref()).num_extra_zero_runs });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub runs: Vec<Outer_RunInfo>,
}
impl Outer {
    pub fn copy_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, runs).write((*_a0.upgrade().deref()).runs.clone());
    }
    pub fn move_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, runs).write(field!(_a0, runs).with_mut(std::mem::take));
    }
    pub fn new_1(this: Ptr<Outer>) {
        field!(this, runs).write(Vec::new());
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
    Outer::new_1(o.as_pointer());
    let info: Value<Outer_RunInfo> = Rc::new(RefCell::new(<Outer_RunInfo>::uninit()));
    Outer_RunInfo::new_1(info.as_pointer());
    (*info.borrow_mut()).block_idx = 1;
    (*info.borrow_mut()).num_extra_zero_runs = 2;
    {
        let a0_clone = ({ (*info.borrow()) }).clone();
        (*o.borrow_mut()).runs.push(a0_clone)
    };
    assert!(({ ((*o.borrow()).runs.len() == 1_usize) }));
    assert!((vec_elem!(o.as_pointer(), runs, 0_usize).with(|__s| __s.block_idx) == 1));
    assert!((vec_elem!(o.as_pointer(), runs, 0_usize).with(|__s| __s.num_extra_zero_runs) == 2));
    return 0;
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field!((*self), runs).with_mut(|__b0| {
            (*__b0) = field!(_a0, runs).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
