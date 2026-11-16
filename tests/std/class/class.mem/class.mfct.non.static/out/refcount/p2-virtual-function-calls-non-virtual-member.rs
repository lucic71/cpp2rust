extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait DeclaresVirtual {
    fn setValue(&self, mut v: i32) {
        (*(*self).upgrade().deref_mut()).value = v;
    }
    fn f(&mut self);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct CallsNonVirtualMembers {
    #[offset(12)]
    pub own: i32,
}
impl Default for CallsNonVirtualMembers {
    fn default() -> Self {
        { CallsNonVirtualMembers { own: 0 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<CallsNonVirtualMembers> =
        Rc::new(RefCell::new(<CallsNonVirtualMembers>::default()));
    let mut base: PtrDyn<dyn DeclaresVirtual> =
        (d.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!(
        ({ ({ ((*d.borrow()) as DeclaresVirtual).value } == 0) && ({ (*d.borrow()).own } == 0) })
    );
    ({ (*base.upgrade().deref_mut()).f() });
    assert!(
        ({ ({ ((*d.borrow()) as DeclaresVirtual).value } == 1) && ({ (*d.borrow()).own } == 2) })
    );
    return 0;
}
impl DeclaresVirtual for CallsNonVirtualMembers {
    fn f(&mut self) {
        ({ DeclaresVirtualImpl::setValue(&((*self) as *mut dyn DeclaresVirtual), 1) });
        ({ CallsNonVirtualMembersImpl::setOwn(&(*self), 2) });
    }
}
pub trait CallsNonVirtualMembersImpl {
    fn setOwn(&self, v: i32);
}
impl CallsNonVirtualMembersImpl for Ptr<CallsNonVirtualMembers> {
    fn setOwn(&self, mut v: i32) {
        field!((*self), own).write(v);
    }
}
pub fn __cpp2rust_init_globals() {}
