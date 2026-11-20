extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Pair {
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[Pair]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <Pair>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = arr.decay();
        {
            let __d = __d.offset(0);
            field!(__d, first).write(1);
            field!(__d, second).write(2);
        };
        {
            let __d = __d.offset(1);
            field!(__d, first).write(3);
            field!(__d, second).write(4);
        };
        {
            let __d = __d.offset(2);
            field!(__d, first).write(5);
            field!(__d, second).write(6);
        };
    };
    'loop_: for __decomp_0 in IterRange::new(arr.decay(), arr.decay().offset((3_i64) as isize)) {
        let __decomp_0: Ptr<Pair> = (__decomp_0).clone();
        {
            let __rhs = __decomp_0.with(|__s| __s.second);
            field!(__decomp_0, first).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
    assert!(({ ({ (*arr.borrow())[(0) as usize].first } == 3) }));
    assert!(({ ({ (*arr.borrow())[(2) as usize].first } == 11) }));
    'loop_: for __decomp_1 in IterRange::new(arr.decay(), arr.decay().offset((3_i64) as isize)) {
        let __decomp_1: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
        Pair::copy_from(__decomp_1.as_pointer(), { (__decomp_1).clone() });
        (*__decomp_1.borrow_mut()).first = 0;
        (*__decomp_1.borrow_mut()).second = 0;
    }
    assert!(({ ({ (*arr.borrow())[(1) as usize].first } == 7) }));
    assert!(({ ({ (*arr.borrow())[(1) as usize].second } == 4) }));
    let mut sum: i32 = 0;
    'loop_: for __decomp_2 in IterRange::new(arr.decay(), arr.decay().offset((3_i64) as isize)) {
        let __decomp_2: Ptr<Pair> = (__decomp_2).clone();
        sum += (__decomp_2.with(|__s| __s.first) * __decomp_2.with(|__s| __s.second));
    }
    assert!((sum == (((3 * 2) + (7 * 4)) + (11 * 6))));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
