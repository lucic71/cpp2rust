extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static failed_casts_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static successful_casts_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct CastsToDerived {}
impl CastsToDerived {
    pub fn new() -> Self {
        let __this: Value<CastsToDerived> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<CastsToDerived> = __this.as_pointer();
        ({ castToDerived_2((this).clone()) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for CastsToDerived {
    fn default() -> Self {
        { CastsToDerived::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct DerivesFromCastsToDerived {}
impl DerivesFromCastsToDerived {
    pub fn new() -> Self {
        let __this: Value<DerivesFromCastsToDerived> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<DerivesFromCastsToDerived> = __this.as_pointer();
        ({ castToDerived_2((this).clone()) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for DerivesFromCastsToDerived {
    fn default() -> Self {
        { DerivesFromCastsToDerived::new() }
    }
}
pub fn castToDerived_2(mut object: Ptr<CastsToDerived>) {
    if (object).is_null() {
        (*failed_casts_0.with(Value::clone).borrow_mut()).prefix_inc();
    } else {
        (*successful_casts_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let d: Value<DerivesFromCastsToDerived> =
            Rc::new(RefCell::new(DerivesFromCastsToDerived::new()));
        let _dtor_d = std::mem::DropGuard::new(d.clone(), |__p| __p.as_pointer().destructor());
        assert!(
            (failed_casts_0.with(|rc| *rc.borrow()) == 1)
                && (successful_casts_1.with(|rc| *rc.borrow()) == 1)
        );
        ({ castToDerived_2((d.as_pointer())) });
        assert!(
            (failed_casts_0.with(|rc| *rc.borrow()) == 1)
                && (successful_casts_1.with(|rc| *rc.borrow()) == 2)
        );
    }
    assert!(
        (failed_casts_0.with(|rc| *rc.borrow()) == 2)
            && (successful_casts_1.with(|rc| *rc.borrow()) == 3)
    );
    return 0;
}
impl CastsToDerived for DerivesFromCastsToDerived {}
pub fn __cpp2rust_init_globals() {
    let _ = failed_casts_0.with(|_| ());
    let _ = successful_casts_1.with(|_| ());
}
