extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<BTreeMap<i32, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
            __v.entry(0.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                .as_pointer()
        })
        .write(1);
    let end: Value<RefcountMapIter<i32, i32>> = Rc::new(RefCell::new(RefcountMapIter::end(
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>),
    )));
    let const_it: Value<RefcountMapIter<i32, i32>> = Rc::new(RefCell::new(
        RefcountMapIter::find_key((m.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>), &0),
    ));
    return if (*const_it.borrow()) == (*end.borrow()) {
        0
    } else {
        1
    };
}
