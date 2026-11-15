extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static derived_hook_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait CallsPureVirtualInConstructor {
    pub fn new() -> Self {
        let __this: Value<CallsPureVirtualInConstructor> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<CallsPureVirtualInConstructor> = __this.as_pointer();
        ({ CallsPureVirtualInConstructorImpl::callHook(&this) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn callHook(&self) {
        ({ (*(*self).upgrade().deref_mut()).hook() });
    }
    fn hook(&mut self);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct OverridesHook {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut d: OverridesHook = <OverridesHook>::default();
    assert!((derived_hook_calls_0.with(|rc| *rc.borrow()) == 0));
    return 0;
}
impl CallsPureVirtualInConstructor for OverridesHook {
    fn hook(&mut self) {
        (*derived_hook_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = derived_hook_calls_0.with(|_| ());
}
