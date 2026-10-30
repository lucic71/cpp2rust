extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_std_pair_0(mut x: i32) -> (Value<i32>, Value<bool>) {
    return (Rc::new(RefCell::new(x)), Rc::new(RefCell::new((x > 0))));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let __decomp_1: Value<(Value<i32>, Value<bool>)> =
        Rc::new(RefCell::new(({ make_std_pair_0(5) })));
    let value: Ptr<i32> = unimplemented!();
    let positive: Ptr<bool> = unimplemented!();
    assert!(((value.read()) == 5));
    assert!(((positive.read()) as bool));
    let p: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2)),
    )));
    let __decomp_2: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new({
        let __a0 = (*p.borrow()).clone();
        (
            Rc::new(RefCell::new((__a0.0.as_pointer().read()))),
            Rc::new(RefCell::new((__a0.1.as_pointer().read()))),
        )
    }));
    let a: Ptr<i32> = unimplemented!();
    let b: Ptr<i32> = unimplemented!();
    a.write(10);
    assert!(((a.read()) == 10));
    assert!(((b.read()) == 2));
    assert!(((*(*p.borrow()).0.borrow()) == 1));
    let __decomp_3: Ptr<(Value<i32>, Value<i32>)> = p.as_pointer();
    let x: Ptr<i32> = unimplemented!();
    let y: Ptr<i32> = unimplemented!();
    x.write(3);
    {
        y.with_mut(|__v| *__v = *__v + 4)
    };
    assert!(((*(*p.borrow()).0.borrow()) == 3));
    assert!(((*(*p.borrow()).1.borrow()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
