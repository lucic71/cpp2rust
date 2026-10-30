extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(32)]
pub struct UserDefined {
    #[offset(0)]
    #[byte_size(4)]
    pub a: (),
    #[offset(8)]
    #[byte_size(24)]
    pub v: (),
}
impl UserDefined {
    pub fn move_from(_a0: Ptr<UserDefined>) -> Self {
        Self {
            a: field!(_a0, a).with_mut(std::mem::take),
            v: unimplemented!(),
        }
    }
}
impl Default for UserDefined {
    fn default() -> Self {
        {
            UserDefined {
                a: unimplemented!(),
                v: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct FieldIsLibcType {
    #[offset(0)]
    #[byte_size(16)]
    pub addr: (),
}
impl Default for FieldIsLibcType {
    fn default() -> Self {
        {
            FieldIsLibcType {
                addr: unimplemented!(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*p.borrow_mut()).fd = -1_();
    (*p.borrow_mut()).events = 0_();
    (*p.borrow_mut()).revents = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let ia: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ia.borrow_mut()).s_addr = 1_();
    unimplemented!();
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*t.borrow_mut()).tm_year = 124_();
    (*t.borrow_mut()).tm_mon = 5_();
    (*t.borrow_mut()).tm_mday = 15_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let st: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*st.borrow_mut()).st_size = 1024_();
    unimplemented!();
    let mut ud: UserDefined = <UserDefined>::default();
    unimplemented!();
    unimplemented!();
    let mut filt: FieldIsLibcType = <FieldIsLibcType>::default();
    unimplemented!();
    return 0_();
}
pub trait UserDefinedImpl {
    fn move_assign(&self, _a0: Ptr<UserDefined>) -> Ptr<UserDefined>;
}
impl UserDefinedImpl for Ptr<UserDefined> {
    fn move_assign(&self, _a0: Ptr<UserDefined>) -> Ptr<UserDefined> {
        field!((*self), a).write({ { (*_a0.upgrade().deref()).a.clone() } });
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
