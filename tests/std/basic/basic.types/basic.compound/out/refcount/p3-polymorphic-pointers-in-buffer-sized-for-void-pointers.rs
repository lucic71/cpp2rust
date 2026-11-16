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
thread_local!(
    pub static third_calls_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait DeclaresVirtual {
    fn f(&mut self);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct FirstOverrider {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct SecondOverrider {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct ThirdOverrider {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let first: Value<FirstOverrider> = Rc::new(RefCell::new(<FirstOverrider>::default()));
    let second: Value<SecondOverrider> = Rc::new(RefCell::new(<SecondOverrider>::default()));
    let third: Value<ThirdOverrider> = Rc::new(RefCell::new(<ThirdOverrider>::default()));
    let mut table: Ptr<PtrDyn<dyn DeclaresVirtual>> =
        libcc2rs::malloc_refcount((3_usize).wrapping_mul((8usize as usize)))
            .reinterpret_cast::<PtrDyn<dyn DeclaresVirtual>>();
    elem!(table, 0).write({ (first.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w) });
    elem!(table, 1).write({ (second.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w) });
    elem!(table, 2).write({ (third.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w) });
    assert!(
        ({ (elem!(table, 0).read()) } == {
            (first.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w)
        })
    );
    assert!(
        ({ (elem!(table, 1).read()) } == {
            (second.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w)
        })
    );
    assert!(
        ({ (elem!(table, 2).read()) } == {
            (third.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w)
        })
    );
    ({ (*(elem!(table, 0).read()).upgrade().deref_mut()).f() });
    assert!(
        ((first_calls_0.with(|rc| *rc.borrow()) == 1)
            && (second_calls_1.with(|rc| *rc.borrow()) == 0))
            && (third_calls_2.with(|rc| *rc.borrow()) == 0)
    );
    ({ (*(elem!(table, 1).read()).upgrade().deref_mut()).f() });
    assert!(
        ((first_calls_0.with(|rc| *rc.borrow()) == 1)
            && (second_calls_1.with(|rc| *rc.borrow()) == 1))
            && (third_calls_2.with(|rc| *rc.borrow()) == 0)
    );
    ({ (*(elem!(table, 2).read()).upgrade().deref_mut()).f() });
    assert!(
        ((first_calls_0.with(|rc| *rc.borrow()) == 1)
            && (second_calls_1.with(|rc| *rc.borrow()) == 1))
            && (third_calls_2.with(|rc| *rc.borrow()) == 1)
    );
    libcc2rs::free_refcount((table).to_any());
    return 0;
}
impl DeclaresVirtual for FirstOverrider {
    fn f(&mut self) {
        (*first_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl DeclaresVirtual for SecondOverrider {
    fn f(&mut self) {
        (*second_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl DeclaresVirtual for ThirdOverrider {
    fn f(&mut self) {
        (*third_calls_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = first_calls_0.with(|_| ());
    let _ = second_calls_1.with(|_| ());
    let _ = third_calls_2.with(|_| ());
}
