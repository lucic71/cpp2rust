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
pub struct A {}
impl A {
    pub fn new_1(this: Ptr<A>) {}
    pub fn copy_from(this: Ptr<A>, _a0: Ptr<A>) {}
    pub fn move_from(this: Ptr<A>, _a0: Ptr<A>) {}
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct D {}
impl D {
    pub fn new_1(this: Ptr<D>) {}
    pub fn copy_from(this: Ptr<D>, _a0: Ptr<D>) {}
    pub fn move_from(this: Ptr<D>, _a0: Ptr<D>) {}
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut d: Ptr<i32> = Ptr::alloc(0);
    d.write(5);
    d.delete();
    let mut c: Ptr<i32> = Ptr::alloc(3);
    c.delete();
    ({
        DImpl::operator_call(
            &{
                let __v: Value<D> = Rc::new(RefCell::new(<D>::uninit()));
                {
                    let __d = __v.as_pointer();
                };
                __v
            }
            .as_pointer(),
            {
                let __p = Ptr::alloc(<A>::uninit());
                A::new_1(__p.clone());
                __p
            },
        )
    });
    return 0;
}
pub trait DImpl {
    fn operator_call(&self, ptr: Ptr<A>);
}
impl DImpl for Ptr<D> {
    fn operator_call(&self, mut ptr: Ptr<A>) {
        ptr.delete();
    }
}
pub fn __cpp2rust_init_globals() {}
