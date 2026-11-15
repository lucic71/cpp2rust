extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static first_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static second_calls_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait HasTwoPureVirtuals {
    fn first(&mut self);
    fn second(&mut self);
}
pub trait OverridesFirstOnly {
    fn first(&mut self) {
        (*first_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct OverridesSecond {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<OverridesSecond> = Rc::new(RefCell::new(<OverridesSecond>::default()));
    let mut base: PtrDyn<dyn HasTwoPureVirtuals> =
        (d.as_pointer()).to_dyn::<dyn HasTwoPureVirtuals>(|w| w);
    ({ (*base.upgrade().deref_mut()).first() });
    assert!(
        (first_calls_0.with(|rc| *rc.borrow()) == 1)
            && (second_calls_1.with(|rc| *rc.borrow()) == 0)
    );
    ({ (*base.upgrade().deref_mut()).second() });
    assert!(
        (first_calls_0.with(|rc| *rc.borrow()) == 1)
            && (second_calls_1.with(|rc| *rc.borrow()) == 1)
    );
    let mut stillAbstract: PtrDyn<dyn OverridesFirstOnly> =
        (d.as_pointer()).to_dyn::<dyn OverridesFirstOnly>(|w| w);
    ({ (*stillAbstract.upgrade().deref_mut()).first() });
    assert!(
        (first_calls_0.with(|rc| *rc.borrow()) == 2)
            && (second_calls_1.with(|rc| *rc.borrow()) == 1)
    );
    ({
        (*(stillAbstract as *mut dyn HasTwoPureVirtuals)
            .upgrade()
            .deref_mut())
        .second()
    });
    assert!(
        (first_calls_0.with(|rc| *rc.borrow()) == 2)
            && (second_calls_1.with(|rc| *rc.borrow()) == 2)
    );
    return 0;
}
impl OverridesFirstOnly for OverridesSecond {
    fn second(&mut self) {
        (*second_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = first_calls_0.with(|_| ());
    let _ = second_calls_1.with(|_| ());
}
