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
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Point {
    pub fn copy_from(this: Ptr<Point>, _a0: Ptr<Point>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Shape {
    #[offset(0)]
    pub id: i32,
    #[offset(4)]
    #[byte_size(16)]
    #[array_len(4)]
    pub coords: Box<[i32]>,
    #[offset(20)]
    #[byte_size(24)]
    #[array_len(3)]
    pub points: Box<[Point]>,
    #[offset(44)]
    pub tail: i32,
}
impl Shape {
    pub fn copy_from(this: Ptr<Shape>, _a0: Ptr<Shape>) {
        field!(this, id).write(_a0.with(|__s| __s.id));
        field!(this, coords).write(Box::new(std::array::from_fn::<_, 4, _>(|__i: usize| {
            (*_a0.upgrade().deref()).coords[(__i) as usize]
        })));
        {
            let __d = array_field_ptr!(this, points);
            for __i in 0..3_usize {
                Point::copy_from(__d.offset(__i as isize), {
                    array_field_ptr!(_a0, points).offset(__i)
                });
            }
        };
        field!(this, tail).write(_a0.with(|__s| __s.tail));
    }
}
impl Default for Shape {
    fn default() -> Self {
        {
            Shape {
                id: 0_i32,
                coords: (0..4).map(|_| 0_i32).collect::<Box<[i32]>>(),
                points: (0..3).map(|_| <Point>::default()).collect::<Box<[Point]>>(),
                tail: 0_i32,
            }
        }
    }
}
pub fn sum_0(mut p: Ptr<i32>, mut n: i32) -> i32 {
    let mut s: i32 = 0;
    {
        let mut i: i32 = 0;
        'loop_: while (i < n) {
            s += { (elem!(p, i).read()) };
            i.prefix_inc();
        }
    }
    return s;
}
pub fn set_y_1(mut p: Ptr<Point>, mut y: i32) {
    field!(p, y).write(y);
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Shape> = Rc::new(RefCell::new(<Shape>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, id).write(1);
        field!(__d, coords).write(Box::new([10, 20, 30, 40]));
        {
            let __d = array_field_ptr!(__d, points);
            {
                let __d = __d.offset(0);
                field!(__d, x).write(1);
                field!(__d, y).write(2);
            };
            {
                let __d = __d.offset(1);
                field!(__d, x).write(3);
                field!(__d, y).write(4);
            };
            {
                let __d = __d.offset(2);
                field!(__d, x).write(5);
                field!(__d, y).write(6);
            };
        };
        field!(__d, tail).write(99);
    };
    let mut c: Ptr<i32> = (array_field_ptr!(s.as_pointer(), coords).offset((1) as isize));
    assert!(((c.read()) == 20));
    c.write(21);
    assert!(((array_elem!(s.as_pointer(), coords, 1).read()) == 21));
    c += 2;
    assert!(((c.read()) == 40));
    assert!(
        (((c).clone() - (array_field_ptr!(s.as_pointer(), coords).offset((0) as isize))) as i64
            == 3_i64)
    );
    assert!(((elem!(c, -1_i32).read()) == 30));
    assert!((({ sum_0(array_field_ptr!(s.as_pointer(), coords), 4,) }) == 101));
    assert!(
        (({
            sum_0(
                (array_field_ptr!(s.as_pointer(), coords).offset((2) as isize)),
                2,
            )
        }) == 70)
    );
    let mut p: Ptr<Point> = (array_field_ptr!(s.as_pointer(), points).offset((1) as isize));
    assert!((p.with(|__s| __s.x) == 3));
    ({ set_y_1(p.offset((1) as isize), 60) });
    assert!((array_elem!(s.as_pointer(), points, 2).with(|__s| __s.y) == 60));
    let mut py: Ptr<i32> = (field_ptr!(
        array_field_ptr!(s.as_pointer(), points).offset((0) as isize),
        y
    ));
    py.write(7);
    assert!((array_elem!(s.as_pointer(), points, 0).with(|__s| __s.y) == 7));
    let mut px: Ptr<i32> = (field_ptr!((p.offset((1) as isize)), x));
    {
        px.with_mut(|__v| *__v = *__v + 50)
    };
    assert!((array_elem!(s.as_pointer(), points, 2).with(|__s| __s.x) == 55));
    let mut sp: Ptr<Shape> = (s.as_pointer());
    let mut d: Ptr<i32> = array_field_ptr!(sp, coords).offset((3) as isize);
    d.write(41);
    assert!(((array_elem!(s.as_pointer(), coords, 3).read()) == 41));
    field!(array_elem!(sp, points, 1), y).write({
        ({ (array_elem!(sp, coords, 0).read()) } + { array_elem!(sp, points, 0).with(|__s| __s.x) })
    });
    assert!((array_elem!(s.as_pointer(), points, 1).with(|__s| __s.y) == 11));
    let mut q: Ptr<Point> = array_field_ptr!(sp, points);
    field!(elem!(q, 2), x).write(8);
    assert!((array_elem!(sp, points, 2).with(|__s| __s.x) == 8));
    let t: Value<Shape> = Rc::new(RefCell::new(<Shape>::uninit()));
    Shape::copy_from(t.as_pointer(), { s.as_pointer() });
    array_elem!(t.as_pointer(), coords, 0).write(0);
    field!(array_elem!(t.as_pointer(), points, 0), x).write(0);
    assert!(
        ((array_elem!(s.as_pointer(), coords, 0).read()) == 10)
            && (array_elem!(s.as_pointer(), points, 0).with(|__s| __s.x) == 1)
    );
    assert!(
        ({
            (((array_elem!(t.as_pointer(), coords, 1).read()) == 21)
                && (array_elem!(t.as_pointer(), points, 2).with(|__s| __s.y) == 60))
                && ({ (*t.borrow()).tail } == 99)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
