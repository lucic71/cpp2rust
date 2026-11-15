extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct HasMember {
    #[offset(0)]
    pub value: i32,
}
impl Default for HasMember {
    fn default() -> Self {
        { HasMember { value: 7 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct HasNonTrivialDestructor {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Ptr<HasNonTrivialDestructor> = Ptr::alloc(<HasNonTrivialDestructor>::default());
    ({ HasNonTrivialDestructorImpl::destructor(&p) });
    return (p as *mut HasMember).with(|__s| __s.value);
}
pub trait HasNonTrivialDestructorImpl {
    fn destructor(&self);
}
impl HasNonTrivialDestructorImpl for Ptr<HasNonTrivialDestructor> {
    fn destructor(&self) {
        field!(((*self) as *mut HasMember), value).write(0);
    }
}
pub fn __cpp2rust_init_globals() {}
