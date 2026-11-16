extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static first_destructions_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static second_destructions_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait DeclaresVirtual {
    fn id(&mut self) -> i32;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct FirstOverrider {}
impl FirstOverrider {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct SecondOverrider {}
impl SecondOverrider {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let storage: Value<Box<[u8]>> =
        Rc::new(RefCell::new((0..64).map(|_| 0_u8).collect::<Box<[u8]>>()));
    let mut object: PtrDyn<dyn DeclaresVirtual> =
        Ptr::alloc(<FirstOverrider>::default()).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!((({ (*object.upgrade().deref_mut()).id() }) == 1));
    ({ (*object.upgrade().deref_mut())() });
    assert!(
        (first_destructions_0.with(|rc| *rc.borrow()) == 1)
            && (second_destructions_1.with(|rc| *rc.borrow()) == 0)
    );
    object = Ptr::alloc(<SecondOverrider>::default()).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!((({ (*object.upgrade().deref_mut()).id() }) == 2));
    ({ (*object.upgrade().deref_mut())() });
    assert!(
        (first_destructions_0.with(|rc| *rc.borrow()) == 1)
            && (second_destructions_1.with(|rc| *rc.borrow()) == 1)
    );
    return 0;
}
impl DeclaresVirtual for FirstOverrider {
    fn id(&mut self) -> i32 {
        return 1;
    }
}
impl DeclaresVirtual for SecondOverrider {
    fn id(&mut self) -> i32 {
        return 2;
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = first_destructions_0.with(|_| ());
    let _ = second_destructions_1.with(|_| ());
}
