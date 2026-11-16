extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static f_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait DeclaresVirtual {
    fn f(&mut self);
}
pub fn setValueThroughPointer_1(mut object: PtrDyn<dyn DeclaresVirtual>, mut value: i32) {
    (*object.upgrade().deref_mut()).value = value;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct PassesThis {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<PassesThis> = Rc::new(RefCell::new(<PassesThis>::default()));
    let mut base: PtrDyn<dyn DeclaresVirtual> =
        (d.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w);
    ({ (*base.upgrade().deref_mut()).f() });
    assert!((f_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ ((*d.borrow()) as DeclaresVirtual).value } == 7) }));
    assert!((base.with(|__s| __s.value) == 7));
    return 0;
}
impl DeclaresVirtual for PassesThis {
    fn f(&mut self) {
        (*f_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
        ({ setValueThroughPointer_1((*self).to_dyn::<dyn DeclaresVirtual>(|w| w), 7) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = f_calls_0.with(|_| ());
}
