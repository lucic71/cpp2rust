extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait DeclaresVirtuals {
    fn run(&mut self);
    fn increment(&mut self);
}
thread_local!(
    pub static other_pointer_to_object_0: Value<PtrDyn<dyn DeclaresVirtuals>> =
        Rc::new(RefCell::new(Ptr::<dyn DeclaresVirtuals>::null()));
);
pub fn incrementThroughOtherPointer_1() {
    ({
        (*(*other_pointer_to_object_0.with(Value::clone).borrow())
            .upgrade()
            .deref_mut())
        .increment()
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ReEntered {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<ReEntered> = Rc::new(RefCell::new(<ReEntered>::default()));
    other_pointer_to_object_0
        .with(|rc| *rc.borrow_mut() = (d.as_pointer()).to_dyn::<dyn DeclaresVirtuals>(|w| w));
    let mut base: PtrDyn<dyn DeclaresVirtuals> =
        (d.as_pointer()).to_dyn::<dyn DeclaresVirtuals>(|w| w);
    ({ (*base.upgrade().deref_mut()).run() });
    assert!(({ ({ ((*d.borrow()) as DeclaresVirtuals).value } == 111) }));
    ({ (*base.upgrade().deref_mut()).increment() });
    assert!(({ ({ ((*d.borrow()) as DeclaresVirtuals).value } == 112) }));
    return 0;
}
impl DeclaresVirtuals for ReEntered {
    fn run(&mut self) {
        self.value = 10;
        ({ incrementThroughOtherPointer_1() });
        self.value += 100;
    }
    fn increment(&mut self) {
        self.value.prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = other_pointer_to_object_0.with(|_| ());
}
