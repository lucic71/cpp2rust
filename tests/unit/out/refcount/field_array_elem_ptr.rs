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
    pub x: (),
    #[offset(4)]
    pub y: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Shape {
    #[offset(0)]
    pub id: (),
    #[offset(4)]
    #[byte_size(16)]
    pub coords: Box<[()]>,
    #[offset(20)]
    #[byte_size(24)]
    pub points: Box<[Point]>,
    #[offset(44)]
    pub tail: (),
}
impl Default for Shape {
    fn default() -> Self {
        {
            Shape {
                id: unimplemented!(),
                coords: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                points: (0..3).map(|_| <Point>::default()).collect::<Box<[Point]>>(),
                tail: unimplemented!(),
            }
        }
    }
}
pub fn sum_0(mut p: Ptr<()>, mut n: ()) -> () {
    let s: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while (i < n) {
        (*s.borrow_mut()) += { (elem!(p, i).read()) };
        i.prefix_inc();
    }
    return (*s.borrow());
}
pub fn set_y_1(mut p: Ptr<Point>, mut y: ()) {
    field!(p, y).write(y);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<Shape> = Rc::new(RefCell::new(Shape {
        id: 1_(),
        coords: Box::new([10_(), 20_(), 30_(), 40_()]),
        points: Box::new([
            Point { x: 1_(), y: 2_() },
            Point { x: 3_(), y: 4_() },
            Point { x: 5_(), y: 6_() },
        ]),
        tail: 99_(),
    }));
    let mut c: Ptr<()> =
        ((array_field_ptr!(s.as_pointer(), coords) as Ptr<()>).offset((1_()) as isize));
    unimplemented!();
    c.write(21_());
    unimplemented!();
    c += 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: Ptr<Point> =
        ((array_field_ptr!(s.as_pointer(), points) as Ptr<Point>).offset((1_()) as isize));
    unimplemented!();
    ({ set_y_1(p.offset((1_()) as isize), 60_()) });
    unimplemented!();
    let mut py: Ptr<()> = (field_ptr!(
        (array_field_ptr!(s.as_pointer(), points) as Ptr<Point>).offset((0_()) as isize),
        y
    ));
    py.write(7_());
    unimplemented!();
    let mut px: Ptr<()> = (field_ptr!((p.offset((1_()) as isize)), x));
    {
        px.with_mut(|__v| *__v = *__v + 50_())
    };
    unimplemented!();
    let mut sp: Ptr<Shape> = (s.as_pointer());
    let mut d: Ptr<()> = (array_field_ptr!(sp, coords) as Ptr<()>).offset((3_()) as isize);
    d.write(41_());
    unimplemented!();
    field!(array_elem!(sp, points, 1_()), y).write({
        ({ (array_elem!(sp, coords, 0_()).read()) } + {
            array_elem!(sp, points, 0_()).with(|__s| __s.x)
        })
    });
    unimplemented!();
    let mut q: Ptr<Point> = (array_field_ptr!(sp, points) as Ptr<Point>);
    field!(elem!(q, 2_()), x).write(8_());
    unimplemented!();
    let mut t: Shape = (*s.borrow()).clone();
    t.coords[(0_()) as usize] = 0_();
    t.points[(0_()) as usize].x = 0_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
