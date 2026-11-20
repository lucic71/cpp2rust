extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(list: Vec<i32>) {
    let list: Value<Vec<i32>> = Rc::new(RefCell::new(list));
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl Pair {
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, b).write(_a0.with(|__s| __s.b));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Pair,
}
impl Holder {
    pub fn new_1(this: Ptr<Holder>) {
        {
            let __d = field_ptr!(this, p);
            field!(__d, a).write(0);
            field!(__d, b).write(0);
        };
    }
    pub fn copy_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        Pair::copy_from(field_ptr!(this, p), { field_ptr!(_a0, p) });
    }
}
impl Default for Holder {
    fn default() -> Self {
        {
            Holder {
                p: Rc::try_unwrap({
                    let __v: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
                    {
                        let __d = __v.as_pointer();
                        field!(__d, a).write(0);
                        field!(__d, b).write(0);
                    };
                    __v
                })
                .ok()
                .unwrap()
                .into_inner(),
            }
        }
    }
}
pub fn sum_1(p: Ptr<Pair>) -> i32 {
    return ({ p.with(|__s| __s.a) } + { p.with(|__s| __s.b) });
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Seven {
    #[offset(0)]
    pub v: i32,
}
impl Seven {
    pub fn copy_from(this: Ptr<Seven>, _a0: Ptr<Seven>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
impl Default for Seven {
    fn default() -> Self {
        { Seven { v: 7 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut i1: i32 = 3;
    let mut i2: i32 = 0_i32;
    let mut carr1: [i32; 2] = [1, 2];
    let mut carr2: [i32; 3] = [1, 0_i32, 0_i32];
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::<[i32]>::from([1, 2, 3])));
    let mut vec_: Vec<i32> = Vec::from([1, 2, 3]);
    ({ f_0(Vec::from([1, 2, 3, 4])) });
    let p: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    {
        let __d = p.as_pointer();
        field!(__d, a).write(1);
        field!(__d, b).write(2);
    };
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    Holder::new_1(h.as_pointer());
    ({ HolderImpl::operator_assign_1(&h.as_pointer(), p.as_pointer()) });
    assert!(({ ({ (*h.borrow()).p.a } == 1) && ({ (*h.borrow()).p.b } == 2) }));
    assert!((({ sum_1(p.as_pointer(),) }) == 3));
    let sevens: Value<Box<[Seven]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <Seven>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = sevens.decay();
        {
            let __d = __d.offset(0);
            field!(__d, v).write(7);
        };
        {
            let __d = __d.offset(1);
            field!(__d, v).write(7);
        };
        {
            let __d = __d.offset(2);
            field!(__d, v).write(7);
        };
        {
            let __d = __d.offset(3);
            field!(__d, v).write(7);
        };
    };
    {
        let mut i: i32 = 0;
        'loop_: while (i < 4) {
            assert!(({ ({ (*sevens.borrow())[(i) as usize].v } == 7) }));
            i.postfix_inc();
        }
    }
    return 0;
}
pub trait HolderImpl {
    fn operator_assign_1(&self, o: Ptr<Pair>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn operator_assign_1(&self, o: Ptr<Pair>) -> Ptr<Holder> {
        field!((*self), p).write({ (*o.upgrade().deref()) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
