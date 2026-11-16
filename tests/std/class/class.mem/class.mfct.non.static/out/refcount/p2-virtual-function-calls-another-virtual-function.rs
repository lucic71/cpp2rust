extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait DeclaresVirtuals {
    fn first(&mut self);
    fn second(&mut self);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct FirstCallsSecond {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct OverridesSecond {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<FirstCallsSecond> = Rc::new(RefCell::new(<FirstCallsSecond>::default()));
    let mut base: PtrDyn<dyn DeclaresVirtuals> =
        (d.as_pointer()).to_dyn::<dyn DeclaresVirtuals>(|w| w);
    ({ (*base.upgrade().deref_mut()).first() });
    assert!(({ ({ ((*d.borrow()) as DeclaresVirtuals).steps } == 11) }));
    let e: Value<OverridesSecond> = Rc::new(RefCell::new(<OverridesSecond>::default()));
    base = (e.as_pointer()).to_dyn::<dyn DeclaresVirtuals>(|w| w);
    ({ (*base.upgrade().deref_mut()).first() });
    assert!(({ ({ ((*e.borrow()) as DeclaresVirtuals).steps } == 101) }));
    assert!(({ ({ ((*d.borrow()) as DeclaresVirtuals).steps } == 11) }));
    return 0;
}
impl DeclaresVirtuals for FirstCallsSecond {
    fn first(&mut self) {
        self.steps.prefix_inc();
        ({ self.second() });
    }
    fn second(&mut self) {
        self.steps += 10;
    }
}
impl FirstCallsSecond for OverridesSecond {
    fn second(&mut self) {
        self.steps += 100;
    }
}
pub fn __cpp2rust_init_globals() {}
