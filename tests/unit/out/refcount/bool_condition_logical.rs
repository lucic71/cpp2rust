extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Code = ();
pub const Code_CODE_OK: Code = 0;
pub const Code_CODE_ERR: Code = 1;
pub const Code_CODE_FATAL: Code = 2;
thread_local!(
    pub static side_effect_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn observe_1(v: ()) -> () {
    let v: Value<()> = Rc::new(RefCell::new(v));
    (*side_effect_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (*v.borrow());
}
pub fn returns_one_2() -> () {
    return 1_();
}
pub fn returns_zero_3() -> () {
    return 0_();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut n: () = 3_();
    let mut zero: () = 0_();
    let storage: Value<()> = Rc::new(RefCell::new(7_()));
    let mut p: Ptr<()> = (storage.as_pointer());
    let mut np: Ptr<()> = Ptr::<()>::null();
    let mut u: () = 4_();
    let mut code: Code = Code_CODE_OK;
    if (n != 0) && (!(p).is_null()) {
        unimplemented!();
    }
    if (n != 0) && (!(np).is_null()) {
        unimplemented!();
    }
    if (zero != 0) || (!(p).is_null()) {
        unimplemented!();
    }
    if (zero != 0) || (!(np).is_null()) {
        unimplemented!();
    }
    if (((n != 0) && (u != 0)) && (!(p).is_null())) && ((code as ()) == (Code_CODE_OK as ())) {
        unimplemented!();
    }
    side_effect_0.with(|rc| *rc.borrow_mut() = 0_());
    if (zero != 0) && (({ observe_1(1_()) }) != 0) {
        unimplemented!();
    }
    unimplemented!();
    if (n != 0) || (({ observe_1(1_()) }) != 0) {
        unimplemented!();
    }
    unimplemented!();
    let mut x: () = 5_();
    let mut y: () = 3_();
    let mut flags: () = 2_();
    if (x > y) || ((flags & 1_()) != 0) {
        unimplemented!();
    }
    if (x < y) || ((flags & 1_()) != 0) {
        unimplemented!();
    }
    let mut a: () = 1_();
    let mut b: () = 2_();
    let mut c: () = 3_();
    if (a != c) && (b != c) {
        unimplemented!();
    }
    let mut s: () = -1_();
    if (!((p).is_null())) && (s < 0_()) {
        unimplemented!();
    }
    let mut k: () = 2_();
    let mut done: () = false;
    if (k > 1_()) || (!(done)) {
        unimplemented!();
    }
    if (x > y) || ((flags & 4_()) != 0) {
        unimplemented!();
    }
    let mut ull: () = 7_();
    if (!((p).is_null())) && (ull != 0) {
        unimplemented!();
    }
    if (x > y) && (ull != 0) {
        unimplemented!();
    }
    let mut mask: () = ((1_() << 4_()) | (1_() << 5_()));
    let mut bits: () = (1_() << 4_());
    if (n != 0_()) && ((bits & mask) != 0) {
        unimplemented!();
    }
    if (n != 0_()) || ((bits & 256_()) != 0) {
        unimplemented!();
    }
    let mut cp: Ptr<()> = Ptr::<()>::from_string_literal(b"hi");
    let mut cnp: Ptr<()> = Ptr::<()>::null();
    if (x > y) && (!(cp).is_null()) {
        unimplemented!();
    }
    if (x < y) || (!(cnp).is_null()) {
        unimplemented!();
    }
    if (x > y) && ((n != 0) && (!(cp).is_null())) {
        unimplemented!();
    }
    if (x > y) && (({ returns_one_2() }) != 0) {
        unimplemented!();
    }
    if (x > y) && (!(({ returns_zero_3() }) != 0)) {
        unimplemented!();
    }
    if (x < y) || (({ returns_one_2() }) != 0) {
        unimplemented!();
    }
    if (x < y) || (!(({ returns_one_2() }) != 0)) {
        unimplemented!();
    }
    if ((!((p).is_null())) && (({ returns_one_2() }) != 0)) && (n != 0_()) {
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = side_effect_0.with(|_| ());
}
