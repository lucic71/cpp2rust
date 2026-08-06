extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static freed_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn real_free_1(p: AnyPtr) {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    (*p.borrow()).clone();
    (*freed_0.with(Value::clone).borrow_mut()).postfix_inc();
}
pub fn consume_2(data: AnyPtr, d: FnPtr<fn(AnyPtr)>) -> i32 {
    let data: Value<AnyPtr> = Rc::new(RefCell::new(data));
    let d: Value<FnPtr<fn(AnyPtr)>> = Rc::new(RefCell::new(d));
    if ((((*d.borrow()).is_null()) as i32) != 0) {
        return 1;
    }
    if ((((*d.borrow()) == (<FnPtr<fn(AnyPtr)>>::from_int(-1_i32))) as i32) != 0) {
        return 2;
    }
    ({ (*(*d.borrow()))((*data.borrow()).clone()) });
    return 3;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(7));
    assert!(
        (((({
            consume_2(
                ((x.as_pointer()) as Ptr<i32>).to_any(),
                (FnPtr::<fn(AnyPtr)>::null()),
            )
        }) == 1) as i32)
            != 0)
    );
    assert!(
        (((({
            consume_2(
                ((x.as_pointer()) as Ptr<i32>).to_any(),
                (<FnPtr<fn(AnyPtr)>>::from_int(-1_i32)),
            )
        }) == 2) as i32)
            != 0)
    );
    assert!(
        (((({
            consume_2(
                ((x.as_pointer()) as Ptr<i32>).to_any(),
                FnPtr::<fn(AnyPtr)>::new(real_free_1),
            )
        }) == 3) as i32)
            != 0)
    );
    assert!(((((*freed_0.with(Value::clone).borrow()) == 1) as i32) != 0));
    return 0;
}
