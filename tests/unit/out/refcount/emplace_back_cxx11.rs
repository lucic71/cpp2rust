extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Pair {
    pub fn new(mut a: i32, mut b: i32) -> Self {
        Self {
            first: a,
            second: (b * 2),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut pairs: Vec<Pair> = Vec::new();
    {
        let __init = Pair::new({ 1 }, { 2 });
        pairs.push(__init)
    };
    assert!((pairs.len() == 1_usize));
    assert!(({ pairs[0_usize].first } == 1) && ({ pairs[0_usize].second } == 4));
    let nested: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __init = vec![5; (3 as usize) as usize];
        (nested.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>)
            .with_mut(|__v: &mut Vec<Value<Vec<i32>>>| __v.push(Rc::new(RefCell::new(__init))))
    };
    assert!(((*nested.borrow()).len() == 1_usize));
    assert!(
        ((*((nested.as_pointer() as Ptr<Value<Vec<i32>>>)
            .offset(0_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<Vec<i32>>)
            .upgrade()
            .deref())
        .len()
            == 3_usize)
            && ((elem!(
                ((nested.as_pointer() as Ptr<Value<Vec<i32>>>)
                    .offset(0_usize)
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<i32>),
                2_usize
            )
            .read())
                == 5)
    );
    let queue: Value<Vec<Pair>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __init = Pair::new({ 6 }, { 7 });
        (*queue.borrow_mut()).push(__init)
    };
    assert!(
        ((queue.as_pointer() as Ptr<Pair>).with(|__s| __s.first) == 6)
            && ((queue.as_pointer() as Ptr<Pair>).with(|__s| __s.second) == 14)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
