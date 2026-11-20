extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Pair {
    #[offset(0)]
    #[byte_size(8)]
    pub x1: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub x2: Ptr<i32>,
}
impl Pair {
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, x1).write((_a0.with(|__s| __s.x1.clone())).clone());
        field!(this, x2).write((_a0.with(|__s| __s.x2.clone())).clone());
    }
    pub fn move_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, x1).write(({ (*_a0.upgrade().deref()).x1.clone() }).clone());
        field!(this, x2).write(({ (*_a0.upgrade().deref()).x2.clone() }).clone());
    }
}
pub fn mkPair_0(__ret: Ptr<Pair>, x1: Ptr<i32>, x2: Ptr<i32>) {
    {
        let __d = __ret.clone();
        field!(__d, x1).write((x1).clone());
        field!(__d, x2).write((x2).clone());
    };
    return;
}
pub fn fill_1(arr: Ptr<Option<Value<Box<[Ptr<i32>]>>>>, n1: Ptr<i32>) {
    let n2: Value<i32> = Rc::new(RefCell::new((n1.read())));
    let pair: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    ({
        let _x1: Ptr<i32> = (n1).clone();
        let _x2: Ptr<i32> = n2.as_pointer();
        mkPair_0(pair.as_pointer(), _x1, _x2)
    });
    ({ (*arr.upgrade().deref()).decay().offset(0_usize) })
        .write(({ (*pair.borrow()).x1.clone() }).clone());
    ({ (*arr.upgrade().deref()).decay().offset(1_usize) })
        .write(({ (*pair.borrow()).x2.clone() }).clone());
}
pub fn any_2(arr: Ptr<Option<Value<Box<[Ptr<i32>]>>>>, n1: Ptr<i32>) -> bool {
    let mut out: bool = false;
    {
        let mut i: i32 = 0;
        'loop_: while ({ i } < { (n1.read()) }) {
            let __rhs = (out)
                || (((({ (*arr.upgrade().deref()).decay().offset((i as usize)) }).read()).read())
                    == 0);
            out = __rhs;
            i.prefix_inc();
        }
    }
    return out;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(2));
    let arr: Value<Option<Value<Box<[Ptr<i32>]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..({ ((*n.borrow()) as usize) }))
                .map(|_| Ptr::<i32>::null())
                .collect::<Box<[_]>>(),
        )))));
    ({ fill_1(arr.as_pointer(), n.as_pointer()) });
    return (({ any_2(arr.as_pointer(), n.as_pointer()) }) as i32);
}
pub fn __cpp2rust_init_globals() {}
