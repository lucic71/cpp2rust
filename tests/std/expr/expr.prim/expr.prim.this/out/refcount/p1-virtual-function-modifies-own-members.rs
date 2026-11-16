extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait DeclaresVirtual {
    fn add(&mut self, amount: i32);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct AddsTwice {
    #[offset(12)]
    pub calls: i32,
}
impl Default for AddsTwice {
    fn default() -> Self {
        { AddsTwice { calls: 0 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<AddsTwice> = Rc::new(RefCell::new(<AddsTwice>::default()));
    let mut base: PtrDyn<dyn DeclaresVirtual> =
        (d.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w);
    let mut derived: Ptr<AddsTwice> = (d.as_pointer());
    ({ (*base.upgrade().deref_mut()).add(3) });
    assert!(
        ({ ({ ((*d.borrow()) as DeclaresVirtual).total } == 6) && ({ (*d.borrow()).calls } == 1) })
    );
    assert!(((derived as *mut dyn DeclaresVirtual).with(|__s| __s.total) == 6));
    ({ (*base.upgrade().deref_mut()).add(1) });
    assert!((base.with(|__s| __s.total) == 8));
    assert!((derived.with(|__s| __s.calls) == 2));
    return 0;
}
impl DeclaresVirtual for AddsTwice {
    fn add(&mut self, mut amount: i32) {
        self.total += (2 * amount);
        self.calls.prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {}
