extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(72)]
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
    #[offset(8)]
    #[byte_size(20)]
    #[array_len(5)]
    pub a: Box<[i32]>,
    #[offset(32)]
    #[byte_size(8)]
    pub r: Ptr<i32>,
    #[offset(40)]
    #[byte_size(8)]
    pub p: Ptr<i32>,
    #[offset(48)]
    #[byte_size(8)]
    pub pair: Ptr<Pair>,
    #[offset(56)]
    #[byte_size(16)]
    #[array_len(2)]
    pub ap: Box<[Ptr<i32>]>,
}
impl Pair {
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
        field!(this, a).write(Box::new(std::array::from_fn::<_, 5, _>(|__i: usize| {
            (*_a0.upgrade().deref()).a[(__i) as usize]
        })));
        field!(this, r).write((_a0.with(|__s| __s.r.clone())).clone());
        field!(this, p).write(_a0.with(|__s| __s.p.clone()));
        field!(this, pair).write(_a0.with(|__s| __s.pair.clone()));
        field!(this, ap).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            ((*_a0.upgrade().deref()).ap[(__i) as usize]).clone()
        })));
    }
    pub fn move_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, y).write({ (*_a0.upgrade().deref()).y });
        field!(this, a).write(Box::new(std::array::from_fn::<_, 5, _>(|__i: usize| {
            (*_a0.upgrade().deref()).a[(__i) as usize]
        })));
        field!(this, r).write(({ (*_a0.upgrade().deref()).r.clone() }).clone());
        field!(this, p).write({ (*_a0.upgrade().deref()).p.clone() });
        field!(this, pair).write({ (*_a0.upgrade().deref()).pair.clone() });
        field!(this, ap).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            ((*_a0.upgrade().deref()).ap[(__i) as usize]).clone()
        })));
    }
}
impl Default for Pair {
    fn default() -> Self {
        {
            Pair {
                x: 0_i32,
                y: 0_i32,
                a: (0..5).map(|_| 0_i32).collect::<Box<[i32]>>(),
                r: <Ptr<i32>>::default(),
                p: Ptr::<i32>::null(),
                pair: Ptr::<Pair>::null(),
                ap: (0..2)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            }
        }
    }
}
pub fn zero_0() -> i32 {
    return 0;
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X1 {}
impl X1 {
    pub fn new_1(this: Ptr<X1>) {}
    pub fn copy_from(this: Ptr<X1>, _a0: Ptr<X1>) {}
    pub fn move_from(this: Ptr<X1>, _a0: Ptr<X1>) {}
}
pub fn foo_1(mut x1: i32, x2: Ptr<i32>, mut x3: Ptr<i32>, p2: Ptr<Pair>, mut p3: Ptr<Pair>) {}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let c1: Value<i32> = Rc::new(RefCell::new((*x1.borrow())));
    let rx1: Ptr<i32> = x1.as_pointer();
    let px1: Value<Ptr<i32>> = Rc::new(RefCell::new((x1.as_pointer())));
    let x2: Value<i32> = Rc::new(RefCell::new((rx1.read())));
    let rx2: Ptr<i32> = (rx1).clone();
    let mut px2: Ptr<i32> = (rx1).clone();
    let mut x3: i32 = ((*px1.borrow()).read());
    let rx3: Ptr<i32> = (*px1.borrow()).clone();
    let mut px3: Ptr<i32> = (*px1.borrow()).clone();
    let mut res: i32 = ((*x1.borrow()) + (*x2.borrow()));
    res = ((*x1.borrow()) + (*x2.borrow()));
    let y1: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    {
        let __d = y1.as_pointer();
        field!(__d, x).write(1);
        field!(__d, y).write(2);
        field!(__d, a).write(Box::new([1, 2, 3, 4, 5]));
        field!(__d, r).write(x1.as_pointer());
        field!(__d, p).write(Ptr::<i32>::null());
        field!(__d, pair).write(Ptr::<Pair>::null());
        field!(__d, ap).write(Box::new([Ptr::<i32>::null(), Ptr::<i32>::null()]));
    };
    let y4: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    {
        let __d = y4.as_pointer();
        field!(__d, x).write({ (*y1.borrow()).x });
        field!(__d, y).write({ (*y1.borrow()).y });
        field!(__d, a).write(Box::new([
            (array_elem!(y1.as_pointer(), a, 0).read()),
            (array_elem!(y1.as_pointer(), a, 1).read()),
            (array_elem!(y1.as_pointer(), a, 2).read()),
            (array_elem!(y1.as_pointer(), a, 3).read()),
            (array_elem!(y1.as_pointer(), a, 4).read()),
        ]));
        field!(__d, r).write(({ (*y1.borrow()).r.clone() }).clone());
        field!(__d, p).write({ (*y1.borrow()).p.clone() });
        field!(__d, pair).write({ (*y1.borrow()).pair.clone() });
        field!(__d, ap).write(Box::new([
            (array_elem!(y1.as_pointer(), ap, 0).read()),
            (array_elem!(y1.as_pointer(), ap, 1).read()),
        ]));
    };
    let ry1: Ptr<Pair> = y1.as_pointer();
    let py1: Value<Ptr<Pair>> = Rc::new(RefCell::new((y1.as_pointer())));
    let y2: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    {
        let __d = y2.as_pointer();
        field!(__d, x).write(ry1.with(|__s| __s.x));
        field!(__d, y).write(ry1.with(|__s| __s.y));
        field!(__d, a).write(Box::new([
            (array_elem!(ry1, a, 0).read()),
            (array_elem!(ry1, a, 1).read()),
            (array_elem!(ry1, a, 2).read()),
            (array_elem!(ry1, a, 3).read()),
            (array_elem!(ry1, a, 4).read()),
        ]));
        field!(__d, r).write((ry1.with(|__s| __s.r.clone())).clone());
        field!(__d, p).write(ry1.with(|__s| __s.p.clone()));
        field!(__d, pair).write(ry1.with(|__s| __s.pair.clone()));
        field!(__d, ap).write(Box::new([
            (array_elem!(ry1, ap, 0).read()),
            (array_elem!(ry1, ap, 1).read()),
        ]));
    };
    let ry2: Ptr<Pair> = (ry1).clone();
    let mut py2: Ptr<Pair> = (ry1).clone();
    let y3: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    {
        let __d = y3.as_pointer();
        field!(__d, x).write((*py1.borrow()).with(|__s| __s.x));
        field!(__d, y).write((*py1.borrow()).with(|__s| __s.y));
        field!(__d, a).write(Box::new([
            (array_elem!((*py1.borrow()), a, 0).read()),
            (array_elem!((*py1.borrow()), a, 1).read()),
            (array_elem!((*py1.borrow()), a, 2).read()),
            (array_elem!((*py1.borrow()), a, 3).read()),
            (array_elem!((*py1.borrow()), a, 4).read()),
        ]));
        field!(__d, r).write(((*py1.borrow()).with(|__s| __s.r.clone())).clone());
        field!(__d, p).write((*py1.borrow()).with(|__s| __s.p.clone()));
        field!(__d, pair).write((*py1.borrow()).with(|__s| __s.pair.clone()));
        field!(__d, ap).write(Box::new([
            (array_elem!((*py1.borrow()), ap, 0).read()),
            (array_elem!((*py1.borrow()), ap, 1).read()),
        ]));
    };
    let ry3: Ptr<Pair> = (*py1.borrow()).clone();
    let mut py3: Ptr<Pair> = (*py1.borrow()).clone();
    py3 = Ptr::<Pair>::null();
    let mut ptr2pair: Ptr<Pair> = (py3).clone();
    ({
        let _x1: i32 = (*x1.borrow());
        let _x2: Ptr<i32> = x1.as_pointer();
        let _x3: Ptr<i32> = (x1.as_pointer());
        let _p2: Ptr<Pair> = y1.as_pointer();
        let _p3: Ptr<Pair> = (y1.as_pointer());
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    ({
        let _x1: i32 = (rx1.read());
        let _x2: Ptr<i32> = (rx1).clone();
        let _x3: Ptr<i32> = (rx1).clone();
        let _p2: Ptr<Pair> = (ry1).clone();
        let _p3: Ptr<Pair> = (ry1).clone();
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    ({
        let _x1: i32 = ((*px1.borrow()).read());
        let _x2: Ptr<i32> = (*px1.borrow()).clone();
        let _x3: Ptr<i32> = (*px1.borrow()).clone();
        let _p2: Ptr<Pair> = (*py1.borrow()).clone();
        let _p3: Ptr<Pair> = (*py1.borrow()).clone();
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    let cr1: Ptr<i32> = c1.as_pointer();
    let mut cp1: Ptr<i32> = (c1.as_pointer());
    (*x1.borrow_mut()) = (*c1.borrow());
    (*x1.borrow_mut()) = 1;
    (*x1.borrow_mut()) = { (cr1.read()) };
    (*x1.borrow_mut()) = { (cp1.read()) };
    rx1.write({ (*c1.borrow()) });
    rx2.write({ (cr1.read()) });
    rx3.write({ (cp1.read()) });
    (*px1.borrow()).write({ (*c1.borrow()) });
    px2.write({ (cr1.read()) });
    px3.write({ (cp1.read()) });
    (*px1.borrow_mut()) = (c1.as_pointer());
    px2 = (cr1).clone();
    px3 = (cp1).clone();
    (*y1.borrow_mut()).x = 2;
    (*y1.borrow_mut()).y = 3;
    array_elem!(y1.as_pointer(), a, 0).write(100);
    { (*y1.borrow()).r.clone() }.write(10);
    (*y1.borrow_mut()).p = (px3).clone();
    px3 = (px2).clone();
    (*y1.borrow_mut()).pair = (y3.as_pointer());
    field!({ (*y1.borrow()).pair.clone() }, x).write(100);
    field!({ (*y1.borrow()).pair.clone() }, pair).write((y2.as_pointer()));
    field!(
        { (*y1.borrow()).pair.clone() }.with(|__s| (__s).pair.clone()),
        x
    )
    .write(100);
    array_elem!(y1.as_pointer(), ap, 0).write((x1.as_pointer()));
    array_elem!(y1.as_pointer(), ap, 1).write((x2.as_pointer()));
    (array_elem!(y1.as_pointer(), ap, 0).read()).write(0);
    (*c1.borrow_mut()) = ((*x1.borrow()) + 1);
    let j: Value<i32> = Rc::new(RefCell::new(0));
    let new_y: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    {
        let __d = new_y.as_pointer();
        field!(__d, x).write(1);
        field!(__d, y).write(2);
        field!(__d, a).write(Box::new([1, 2, 3, 4, 5]));
        field!(__d, r).write(j.as_pointer());
        field!(__d, p).write(Ptr::<i32>::null());
        field!(__d, pair).write(Ptr::<Pair>::null());
        field!(__d, ap).write(Box::new([Ptr::<i32>::null(), Ptr::<i32>::null()]));
    };
    (*y1.borrow_mut()).x = { { (*new_y.borrow()).x } };
    let mut i: u32 = 1_u32;
    array_elem!(y1.as_pointer(), a, i).write(-1_i32);
    (*x1.borrow_mut()).postfix_inc();
    (*x1.borrow_mut()).prefix_inc();
    (*y1.borrow_mut()).x.postfix_inc();
    field!({ (*y1.borrow()).pair.clone() }, pair).write((y2.as_pointer()));
    field!(
        { (*y1.borrow()).pair.clone() }.with(|__s| __s.pair.clone()),
        x
    )
    .write(10);
    ({ PairImpl::method(&y1.as_pointer()) });
    (*y1.borrow_mut()).pair = (y2.as_pointer());
    (*y2.borrow_mut()).pair = (y3.as_pointer());
    ({ PairImpl::method(&{ (*y1.borrow()).pair.clone() }.with(|__s| __s.pair.clone())) });
    let x: Value<X1> = Rc::new(RefCell::new(<X1>::uninit()));
    {
        let __d = x.as_pointer();
    };
    let y: Value<X1> = Rc::new(RefCell::new(<X1>::uninit()));
    {
        let __d = y.as_pointer();
    };
    (*x1.borrow_mut()) = (({ zero_0() }) + { (*y1.borrow()).x });
    (*y1.borrow_mut()).x = (({ zero_0() }) + 5);
    let mut ptr2ptr_1: Ptr<Ptr<i32>> = (px1.as_pointer());
    let mut ptr2ptr_2: Ptr<Ptr<Pair>> = (py1.as_pointer());
    return 0;
}
pub trait PairImpl {
    fn method(&self);
    fn as_val(&self) -> i32;
    fn as_ref(&self) -> Ptr<i32>;
    fn as_ptr(&self) -> Ptr<i32>;
}
impl PairImpl for Ptr<Pair> {
    fn method(&self) {
        field!((*self), x).with_mut(|__v| __v.postfix_inc());
        field!((*self), y).with_mut(|__v| {
            __v.prefix_inc();
        });
        array_elem!((*self), a, 4).write(1);
        (*self).with(|__s| __s.r.clone()).write(1);
        field!((*self), p).write(Ptr::<i32>::null());
        field!((*self), pair).write(Ptr::<Pair>::null());
        array_elem!((*self), ap, 0).write(Ptr::<i32>::null());
    }
    fn as_val(&self) -> i32 {
        return (*self).with(|__s| __s.x);
    }
    fn as_ref(&self) -> Ptr<i32> {
        return field_ptr!((*self), x);
    }
    fn as_ptr(&self) -> Ptr<i32> {
        return (field_ptr!((*self), x));
    }
}
pub fn __cpp2rust_init_globals() {}
