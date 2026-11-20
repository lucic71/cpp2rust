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
pub struct X {
    #[offset(0)]
    pub x: i32,
}
impl X {
    pub fn new_1(this: Ptr<X>) {}
    pub fn copy_from(this: Ptr<X>, _a0: Ptr<X>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
    pub fn move_from(this: Ptr<X>, _a0: Ptr<X>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
    }
}
pub fn func_0() -> i32 {
    return 42;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0_i32));
    let mut p: Ptr<i32> = Ptr::<i32>::null();
    let g: Ptr<i32> = x.as_pointer();
    let mut q: Ptr<i32> = (x.as_pointer());
    let mut z: Ptr<i32> = (p).clone();
    let xx: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
    X::new_1(xx.as_pointer());
    let mut zz: Ptr<X> = (xx.as_pointer());
    (*xx.borrow_mut()).x = 1;
    q = (field_ptr!(xx.as_pointer(), x));
    q = (field_ptr!(zz, x));
    field!(zz, x).write(2);
    let ww: Value<X> = Rc::new(RefCell::new(<X>::uninit()));
    X::copy_from(ww.as_pointer(), { xx.as_pointer() });
    (*ww.borrow_mut()) = (*xx.borrow());
    let mut aa: i32 = ({ func_0() });
    aa = ({ func_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
