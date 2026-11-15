extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static abstract_hook_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static derived_hook_calls_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static helper_calls_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait CallsHookInConstructor {
    pub fn new() -> Self {
        let __this: Value<CallsHookInConstructor> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<CallsHookInConstructor> = __this.as_pointer();
        ({ CallsHookInConstructorImpl::helper(&this) });
        ({ (*this.upgrade().deref_mut()).hook() });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn helper(&self) {
        (*helper_calls_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn hook(&mut self) {
        (*abstract_hook_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn pure(&mut self);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct OverridesHook {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let d: Value<OverridesHook> = Rc::new(RefCell::new(<OverridesHook>::default()));
        assert!((helper_calls_2.with(|rc| *rc.borrow()) == 1));
        assert!(
            (abstract_hook_calls_0.with(|rc| *rc.borrow()) == 1)
                && (derived_hook_calls_1.with(|rc| *rc.borrow()) == 0)
        );
        let mut p: PtrDyn<dyn CallsHookInConstructor> =
            (d.as_pointer()).to_dyn::<dyn CallsHookInConstructor>(|w| w);
        ({ (*p.upgrade().deref_mut()).hook() });
        assert!(
            (abstract_hook_calls_0.with(|rc| *rc.borrow()) == 1)
                && (derived_hook_calls_1.with(|rc| *rc.borrow()) == 1)
        );
    }
    assert!((helper_calls_2.with(|rc| *rc.borrow()) == 2));
    assert!(
        (abstract_hook_calls_0.with(|rc| *rc.borrow()) == 2)
            && (derived_hook_calls_1.with(|rc| *rc.borrow()) == 1)
    );
    return 0;
}
impl CallsHookInConstructor for OverridesHook {
    fn hook(&mut self) {
        (*derived_hook_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn pure(&mut self) {}
}
pub fn __cpp2rust_init_globals() {
    let _ = abstract_hook_calls_0.with(|_| ());
    let _ = derived_hook_calls_1.with(|_| ());
    let _ = helper_calls_2.with(|_| ());
}
