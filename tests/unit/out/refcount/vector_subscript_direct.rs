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
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Vec<i32>,
    #[offset(24)]
    #[byte_size(24)]
    pub points: Vec<Point>,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            values: field!(_a0, values).with_mut(std::mem::take),
            points: field!(_a0, points).with_mut(std::mem::take),
        }
    }
}
pub fn push_and_index_0(v: Ptr<Vec<i32>>) -> i32 {
    {
        let __a1 = 42;
        v.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    return (((*v.upgrade().deref()).len() as i32) - 1);
}
pub fn sum_ref_1(v: Ptr<Vec<i32>>) -> i32 {
    let s: Value<i32> = Rc::new(RefCell::new(0));
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*v.upgrade().deref()).len() }) {
        (*s.borrow_mut()) += ({
            let __idx = i;
            if __idx as usize >= (*v.upgrade().deref()).len() {
                panic!("out of bounds access")
            } else {
                v.decay().offset(__idx as isize)
            }
        }
        .read());
        i.prefix_inc();
    }
    return (*s.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    v.decay().offset(0_usize as isize).write(10);
    assert!(((v.decay().offset(0_usize as isize).read()) == 10));
    {
        v.decay()
            .offset(1_usize as isize)
            .with_mut(|__v| *__v = *__v + 5)
    };
    v.decay()
        .offset(2_usize as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(
        ((v.decay().offset(1_usize as isize).read()) == 7)
            && ((v.decay().offset(2_usize as isize).read()) == 4)
    );
    let __rhs = (v.decay().offset(1_usize as isize).read());
    v.decay().offset(0_usize as isize).write(__rhs);
    assert!(((v.decay().offset(0_usize as isize).read()) == 7));
    v.decay().offset(1_usize as isize).write(0);
    assert!(
        ((v.decay()
            .offset(((v.decay().offset(1_usize as isize).read()) as usize) as isize)
            .read())
            == 7)
    );
    let mut i: i32 = 0;
    v.decay()
        .offset((i.postfix_inc() as usize) as isize)
        .write(3);
    assert!((i == 1) && ((v.decay().offset(0_usize as isize).read()) == 3));
    assert!(
        ((v.decay()
            .offset((({ push_and_index_0(v.as_pointer(),) }) as usize) as isize)
            .read())
            == 42)
    );
    v.decay()
        .offset((({ push_and_index_0(v.as_pointer()) }) as usize) as isize)
        .write(5);
    assert!(((*v.borrow()).len() == 5_usize) && ((v.decay().offset(4_usize as isize).read()) == 5));
    let mut p: Ptr<i32> = (v.decay().offset(2_usize as isize));
    p.write(9);
    assert!(((v.decay().offset(2_usize as isize).read()) == 9));
    assert!((({ sum_ref_1(v.as_pointer(),) }) == ((((3 + 0) + 9) + 42) + 5)));
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    {
        let __a0 = 2_usize as usize;
        (*h.borrow_mut()).values.resize_with(__a0, || 0_i32)
    };
    field_ref!(h.as_pointer(), values)
        .decay()
        .offset(1_usize as isize)
        .write(6);
    {
        let __a1 = Point { x: 1, y: 2 };
        (*h.borrow_mut()).points.push(__a1)
    };
    field!(
        field_ref!(h.as_pointer(), points)
            .decay()
            .offset(0_usize as isize),
        y
    )
    .write(5);
    assert!(
        ((field_ref!(h.as_pointer(), values)
            .decay()
            .offset(1_usize as isize)
            .read())
            == 6)
    );
    assert!(
        (({
            PointImpl::sum(
                &field_ref!(h.as_pointer(), points)
                    .decay()
                    .offset(0_usize as isize),
            )
        }) == 6)
    );
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let __rhs = ((field_ref!(hp, values)
        .decay()
        .offset(1_usize as isize)
        .read())
        + 1);
    field_ref!(hp, values)
        .decay()
        .offset(0_usize as isize)
        .write(__rhs);
    let __rhs = (field_ref!(hp, values)
        .decay()
        .offset(0_usize as isize)
        .read());
    field!(field_ref!(hp, points).decay().offset(0_usize as isize), x).write(__rhs);
    assert!(
        ((field_ref!(h.as_pointer(), values)
            .decay()
            .offset(0_usize as isize)
            .read())
            == 7)
            && (field_ref!(h.as_pointer(), points)
                .decay()
                .offset(0_usize as isize)
                .with(|__s| __s.x)
                == 7)
    );
    let mut q: Point = (*field_ref!(h.as_pointer(), points)
        .decay()
        .offset(0_usize as isize)
        .upgrade()
        .deref())
    .clone();
    q.x = 0;
    assert!(
        (field_ref!(h.as_pointer(), points)
            .decay()
            .offset(0_usize as isize)
            .with(|__s| __s.x)
            == 7)
    );
    let grid: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = vec![0; 3_usize as usize];
        (*grid.borrow_mut()).push(__a1)
    };
    {
        let __a1 = vec![0; 3_usize as usize];
        (*grid.borrow_mut()).push(__a1)
    };
    grid.decay()
        .offset(1_usize as isize)
        .decay()
        .offset(2_usize as isize)
        .write(8);
    assert!(
        ((grid
            .decay()
            .offset(1_usize as isize)
            .decay()
            .offset(2_usize as isize)
            .read())
            == 8)
            && ((grid
                .decay()
                .offset(0_usize as isize)
                .decay()
                .offset(2_usize as isize)
                .read())
                == 0)
    );
    let __rhs = (grid
        .decay()
        .offset(1_usize as isize)
        .decay()
        .offset(2_usize as isize)
        .read());
    grid.decay()
        .offset(0_usize as isize)
        .decay()
        .offset(1_usize as isize)
        .write(__rhs);
    let __rhs = (grid
        .decay()
        .offset(0_usize as isize)
        .decay()
        .offset(1_usize as isize)
        .read());
    {
        grid.decay()
            .offset(1_usize as isize)
            .decay()
            .offset(0_usize as isize)
            .with_mut(|__v| *__v = *__v + __rhs)
    };
    assert!(
        ((grid
            .decay()
            .offset(0_usize as isize)
            .decay()
            .offset(1_usize as isize)
            .read())
            == 8)
            && ((grid
                .decay()
                .offset(1_usize as isize)
                .decay()
                .offset(0_usize as isize)
                .read())
                == 8)
    );
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([4, 5, 6])));
    let __rhs =
        ((a.decay().offset(0_usize as isize).read()) + (a.decay().offset(2_usize as isize).read()));
    a.decay().offset(1_usize as isize).write(__rhs);
    assert!(((a.decay().offset(1_usize as isize).read()) == 10));
    return 0;
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        field_ref!((*self), values)
            .as_pointer()
            .write(field!(_a0, values).with_mut(std::mem::take));
        field_ref!((*self), points)
            .as_pointer()
            .write(field!(_a0, points).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub trait PointImpl {
    fn sum(&self) -> i32;
}
impl PointImpl for Ptr<Point> {
    fn sum(&self) -> i32 {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.y));
    }
}
pub fn __cpp2rust_init_globals() {}
