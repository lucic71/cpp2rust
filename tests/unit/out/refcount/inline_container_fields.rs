extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(48)]
pub struct Elem {
    #[offset(0)]
    pub id: (),
    #[offset(4)]
    #[byte_size(16)]
    pub hist: Box<[()]>,
    #[offset(24)]
    #[byte_size(24)]
    pub coeffs: (),
}
impl Elem {
    pub fn move_from(_a0: Ptr<Elem>) -> Self {
        Self {
            id: { (*_a0.upgrade().deref()).id },
            hist: Box::new(std::array::from_fn::<_, 4, _>(|__i: usize| {
                (*_a0.upgrade().deref()).hist[(__i) as usize]
            })),
            coeffs: unimplemented!(),
        }
    }
}
impl Default for Elem {
    fn default() -> Self {
        {
            Elem {
                id: unimplemented!(),
                hist: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                coeffs: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(88)]
pub struct State {
    #[offset(0)]
    pub count: (),
    #[offset(4)]
    #[byte_size(24)]
    pub grid: Box<[Box<[()]>]>,
    #[offset(28)]
    #[byte_size(12)]
    pub arr: (),
    #[offset(40)]
    #[byte_size(24)]
    pub elems: (),
    #[offset(64)]
    #[byte_size(24)]
    pub rows: (),
}
impl State {
    pub fn move_from(_a0: Ptr<State>) -> Self {
        Self {
            count: { (*_a0.upgrade().deref()).count },
            grid: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                Box::<[_]>::from(std::array::from_fn::<_, 3, _>(|__i2: usize| {
                    (*_a0.upgrade().deref()).grid[(__i) as usize][(__i2) as usize]
                }))
            })),
            arr: field!(_a0, arr).with_mut(std::mem::take),
            elems: unimplemented!(),
            rows: unimplemented!(),
        }
    }
}
impl Default for State {
    fn default() -> Self {
        {
            State {
                count: unimplemented!(),
                grid: (0..2)
                    .map(|_| (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>())
                    .collect::<Box<[Box<[()]>]>>(),
                arr: unimplemented!(),
                elems: unimplemented!(),
                rows: unimplemented!(),
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
pub fn fill_1(mut s: Ptr<State>) {
    let mut i: () = 0_();
    'loop_: while (i < 2_()) {
        let mut j: () = 0_();
        'loop_: while (j < 3_()) {
            index!(index!(field!(s, grid), i), j).write({ ((i * 3_()) + j) });
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    unimplemented!().write(7_());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < 3_()) {
        let __rhs = i;
        field!(unimplemented!(), id).write(__rhs);
        let __rhs = (10_() * i);
        array_elem!(unimplemented!(), hist, i).write(__rhs);
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<State> = Rc::new(RefCell::new(<State>::default()));
    ({ fill_1((s.as_pointer())) });
    unimplemented!();
    unimplemented!();
    let mut e: Ptr<Elem> = (unimplemented!());
    let mut h: Ptr<()> = (array_field_ptr!(e, hist) as Ptr<()>);
    let mut c: Ptr<()> = (unimplemented!());
    let mut id: Ptr<()> = (field_ptr!(unimplemented!(), id));
    let mut k: () = 0_();
    'loop_: while (k < 4_()) {
        {
            let __rhs = { k };
            elem!(h, k).with_mut(|__v| *__v = *__v + __rhs)
        };
        k.prefix_inc();
    }
    {
        c.with_mut(|__v| *__v = *__v + 5_())
    };
    id.write(11_());
    (*s.borrow_mut()).count = 3_();
    unimplemented!();
    unimplemented!();
    let mut row: Ptr<()> = (unimplemented!());
    elem!(row, 2_()).write(9_());
    unimplemented!();
    let t: Value<State> = Rc::new(RefCell::new((*s.borrow()).clone()));
    array_elem!(unimplemented!(), hist, 0_()).write(100_());
    (*t.borrow_mut()).grid[(0_()) as usize][(0_()) as usize] = 100_();
    unimplemented!().write(100_());
    unimplemented!();
    unimplemented!();
    let mut p: Ptr<State> = (t.as_pointer());
    let __rhs = ({ (*p.upgrade().deref()).grid[(1_()) as usize][(1_()) as usize] } + {
        (unimplemented!().read())
    });
    array_elem!(unimplemented!(), hist, 3_()).write(__rhs);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut elems: () = unimplemented!();
    let mut first: Ptr<Elem> = (unimplemented!());
    unimplemented!();
    unimplemented!().write(4_());
    let __rhs = ((unimplemented!().read()) + 1_());
    index!(field!(unimplemented!(), hist), 2_()).write(__rhs);
    unimplemented!();
    let moved: Value<State> = Rc::new(RefCell::new(State::move_from({ t.as_pointer() })));
    unimplemented!();
    unimplemented!();
    let assigned: Value<State> = Rc::new(RefCell::new(<State>::default()));
    ({ StateImpl::move_assign(&assigned.as_pointer(), moved.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait ElemImpl {
    fn move_assign(&self, _a0: Ptr<Elem>) -> Ptr<Elem>;
}
impl ElemImpl for Ptr<Elem> {
    fn move_assign(&self, _a0: Ptr<Elem>) -> Ptr<Elem> {
        field!((*self), id).write({ { (*_a0.upgrade().deref()).id } });
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub trait StateImpl {
    fn move_assign(&self, _a0: Ptr<State>) -> Ptr<State>;
}
impl StateImpl for Ptr<State> {
    fn move_assign(&self, _a0: Ptr<State>) -> Ptr<State> {
        field!((*self), count).write({ { (*_a0.upgrade().deref()).count } });
        unimplemented!();
        field!((*self), arr).write({ { (*_a0.upgrade().deref()).arr.clone() } });
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
