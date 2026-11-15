extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static defaultCtors_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static intCtors_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct CountsConstructors {
    #[offset(0)]
    pub value: i32,
}
impl CountsConstructors {
    pub fn new_2() -> Self {
        let __this: CountsConstructors = Self { value: -1_i32 };
        (*defaultCtors_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_1(mut v: i32) -> Self {
        let __this: CountsConstructors = Self { value: v };
        (*intCtors_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for CountsConstructors {
    fn default() -> Self {
        { CountsConstructors::new_2() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct OtherBase {
    #[offset(0)]
    pub other: i32,
}
impl OtherBase {
    pub fn new(mut v: i32) -> Self {
        Self { other: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct HasNoCtorInitializer {}
impl HasNoCtorInitializer {
    pub fn new_2() -> Self {
        Self {}
    }
    pub fn new_1(mut _a0: i32) -> Self {
        Self {}
    }
}
impl Default for HasNoCtorInitializer {
    fn default() -> Self {
        { HasNoCtorInitializer::new_2() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct InitializesOnlyOtherBase {}
impl InitializesOnlyOtherBase {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for InitializesOnlyOtherBase {
    fn default() -> Self {
        { InitializesOnlyOtherBase::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct HasImplicitConstructor {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct InitializesBase {}
impl InitializesBase {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for InitializesBase {
    fn default() -> Self {
        { InitializesBase::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<HasNoCtorInitializer> = Rc::new(RefCell::new(HasNoCtorInitializer::new_2()));
    assert!((defaultCtors_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ ((*a.borrow()) as CountsConstructors).value } == -1_i32) }));
    let b: Value<HasNoCtorInitializer> = Rc::new(RefCell::new(HasNoCtorInitializer::new_1({ 1 })));
    assert!((defaultCtors_0.with(|rc| *rc.borrow()) == 2));
    assert!(({ ({ ((*b.borrow()) as CountsConstructors).value } == -1_i32) }));
    let c: Value<InitializesOnlyOtherBase> = Rc::new(RefCell::new(InitializesOnlyOtherBase::new()));
    assert!((defaultCtors_0.with(|rc| *rc.borrow()) == 3));
    assert!(
        ({
            ({ ((*c.borrow()) as CountsConstructors).value } == -1_i32)
                && ({ ((*c.borrow()) as OtherBase).other } == 5)
        })
    );
    let d: Value<HasImplicitConstructor> =
        Rc::new(RefCell::new(<HasImplicitConstructor>::default()));
    assert!((defaultCtors_0.with(|rc| *rc.borrow()) == 4));
    assert!(({ ({ ((*d.borrow()) as CountsConstructors).value } == -1_i32) }));
    let e: Value<InitializesBase> = Rc::new(RefCell::new(InitializesBase::new()));
    assert!((defaultCtors_0.with(|rc| *rc.borrow()) == 4));
    assert!((intCtors_1.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ ((*e.borrow()) as CountsConstructors).value } == 7) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = defaultCtors_0.with(|_| ());
    let _ = intCtors_1.with(|_| ());
}
