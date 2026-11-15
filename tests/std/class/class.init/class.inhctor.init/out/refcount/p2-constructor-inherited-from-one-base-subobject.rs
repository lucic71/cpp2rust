extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static default_ctors_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static int_ctors_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static shared_int_ctors_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct HasTwoConstructors {
    #[offset(0)]
    pub value: i32,
}
impl HasTwoConstructors {
    pub fn new_2() -> Self {
        let __this: HasTwoConstructors = Self { value: -1_i32 };
        (*default_ctors_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn new_1(mut v: i32) -> Self {
        let __this: HasTwoConstructors = Self { value: v };
        (*int_ctors_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for HasTwoConstructors {
    fn default() -> Self {
        { HasTwoConstructors::new_2() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct InheritsConstructors {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct DoesNotInherit {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct JoinsInheritingAndPlain {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct HasIntConstructor {
    #[offset(0)]
    pub value: i32,
}
impl HasIntConstructor {
    pub fn new(mut v: i32) -> Self {
        let __this: HasIntConstructor = Self { value: v };
        (*shared_int_ctors_2.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct SharedBase {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct FirstVirtualPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct SecondVirtualPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct JoinsVirtualPaths {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let joined: Value<JoinsInheritingAndPlain> =
        Rc::new(RefCell::new(JoinsInheritingAndPlain::new_1({ 0 })));
    assert!(
        (int_ctors_1.with(|rc| *rc.borrow()) == 1)
            && (default_ctors_0.with(|rc| *rc.borrow()) == 1)
    );
    assert!(
        ({ ({ (((*joined.borrow()) as InheritsConstructors) as HasTwoConstructors).value } == 0) })
    );
    assert!(
        ({ ({ (((*joined.borrow()) as DoesNotInherit) as HasTwoConstructors).value } == -1_i32) })
    );
    let shared: Value<JoinsVirtualPaths> = Rc::new(RefCell::new(JoinsVirtualPaths::new_1({ 5 })));
    assert!((shared_int_ctors_2.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ ((*shared.borrow()) as HasIntConstructor).value } == 5) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = default_ctors_0.with(|_| ());
    let _ = int_ctors_1.with(|_| ());
    let _ = shared_int_ctors_2.with(|_| ());
}
