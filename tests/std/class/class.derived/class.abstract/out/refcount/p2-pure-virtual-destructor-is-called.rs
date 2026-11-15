extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static abstract_dtors_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static concrete_dtors_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct OverridesDestructor {}
impl OverridesDestructor {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let d: Value<OverridesDestructor> = Rc::new(RefCell::new(<OverridesDestructor>::default()));
        let _dtor_d = std::mem::DropGuard::new(d.clone(), |__p| __p.as_pointer().destructor());
        assert!(
            (abstract_dtors_0.with(|rc| *rc.borrow()) == 0)
                && (concrete_dtors_1.with(|rc| *rc.borrow()) == 0)
        );
    }
    assert!(
        (abstract_dtors_0.with(|rc| *rc.borrow()) == 1)
            && (concrete_dtors_1.with(|rc| *rc.borrow()) == 1)
    );
    let mut p: PtrDyn<dyn HasPureVirtualDestructor> =
        Ptr::alloc(<OverridesDestructor>::default()).to_dyn::<dyn HasPureVirtualDestructor>(|w| w);
    {
        let __p = p;
        __p.destructor();
        __p.delete();
    };
    assert!(
        (abstract_dtors_0.with(|rc| *rc.borrow()) == 2)
            && (concrete_dtors_1.with(|rc| *rc.borrow()) == 2)
    );
    return 0;
}
impl HasPureVirtualDestructor for OverridesDestructor {}
pub fn __cpp2rust_init_globals() {
    let _ = abstract_dtors_0.with(|_| ());
    let _ = concrete_dtors_1.with(|_| ());
}
