extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait DeclaresVirtual {
    fn id(&mut self) -> i32;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct HoldsId {
    #[offset(8)]
    pub held: i32,
}
impl Default for HoldsId {
    fn default() -> Self {
        { HoldsId { held: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct HasPolymorphicMembers {
    #[offset(0)]
    pub before: i32,
    #[offset(8)]
    #[byte_size(16)]
    pub first: HoldsId,
    #[offset(24)]
    #[byte_size(16)]
    pub second: HoldsId,
}
impl Default for HasPolymorphicMembers {
    fn default() -> Self {
        {
            HasPolymorphicMembers {
                before: 0,
                first: <HoldsId>::default(),
                second: <HoldsId>::default(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let owner: Value<HasPolymorphicMembers> =
        Rc::new(RefCell::new(<HasPolymorphicMembers>::default()));
    (*owner.borrow_mut()).first.held = 4;
    (*owner.borrow_mut()).second.held = 5;
    let mut base: PtrDyn<dyn DeclaresVirtual> =
        (field_ptr!(owner.as_pointer(), first)).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!((({ (*base.upgrade().deref_mut()).id() }) == 4));
    base = (field_ptr!(owner.as_pointer(), second)).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!((({ (*base.upgrade().deref_mut()).id() }) == 5));
    (*owner.borrow_mut()).second.held = 6;
    assert!((({ (*base.upgrade().deref_mut()).id() }) == 6));
    return 0;
}
impl DeclaresVirtual for HoldsId {
    fn id(&mut self) -> i32 {
        return { self.held };
    }
}
pub fn __cpp2rust_init_globals() {}
