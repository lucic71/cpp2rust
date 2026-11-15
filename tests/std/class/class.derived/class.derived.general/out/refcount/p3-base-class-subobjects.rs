extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct DeclaresABC {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
    #[offset(8)]
    pub c: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct RedeclaresB {
    #[offset(12)]
    pub b: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(20)]
pub struct RedeclaresC {
    #[offset(16)]
    pub c: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<RedeclaresC> = Rc::new(RefCell::new(<RedeclaresC>::default()));
    ((*d.borrow_mut()) as DeclaresABC).a = 1;
    ((*d.borrow_mut()) as DeclaresABC).b = 2;
    ((*d.borrow_mut()) as DeclaresABC).c = 3;
    ((*d.borrow_mut()) as RedeclaresB).b = 20;
    (*d.borrow_mut()).c = 30;
    assert!(({ ({ ((*d.borrow()) as DeclaresABC).a } == 1) }));
    assert!(({ ({ ((*d.borrow()) as RedeclaresB).b } == 20) }));
    assert!(({ ({ (*d.borrow()).c } == 30) }));
    assert!(({ ({ ((*d.borrow()) as DeclaresABC).b } == 2) }));
    assert!(({ ({ ((*d.borrow()) as DeclaresABC).c } == 3) }));
    let asRedeclaresB: Ptr<RedeclaresB> = d.as_pointer();
    assert!((asRedeclaresB.with(|__s| (__s as DeclaresABC).a) == 1));
    assert!((asRedeclaresB.with(|__s| __s.b) == 20));
    assert!((asRedeclaresB.with(|__s| (__s as DeclaresABC).c) == 3));
    let asDeclaresABC: Ptr<DeclaresABC> = d.as_pointer();
    assert!((asDeclaresABC.with(|__s| __s.a) == 1));
    assert!((asDeclaresABC.with(|__s| __s.b) == 2));
    assert!((asDeclaresABC.with(|__s| __s.c) == 3));
    field!(asDeclaresABC, b).write(4);
    field!(asRedeclaresB, c).write(5);
    assert!(({ ({ ((*d.borrow()) as RedeclaresB).b } == 20) }));
    assert!(({ ({ (*d.borrow()).c } == 30) }));
    assert!(({ ({ ((*d.borrow()) as DeclaresABC).b } == 4) }));
    assert!(({ ({ ((*d.borrow()) as DeclaresABC).c } == 5) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
