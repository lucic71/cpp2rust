extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Edge {
    #[offset(0)]
    pub u: i32,
    #[offset(4)]
    pub v: i32,
    #[offset(8)]
    pub weight: f64,
}
impl Edge {
    pub fn new_1(this: Ptr<Edge>) {}
    pub fn copy_from(this: Ptr<Edge>, _a0: Ptr<Edge>) {
        field!(this, u).write(_a0.with(|__s| __s.u));
        field!(this, v).write(_a0.with(|__s| __s.v));
        field!(this, weight).write(_a0.with(|__s| __s.weight));
    }
    pub fn move_from(this: Ptr<Edge>, _a0: Ptr<Edge>) {
        field!(this, u).write({ (*_a0.upgrade().deref()).u });
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
        field!(this, weight).write({ (*_a0.upgrade().deref()).weight });
    }
}
pub fn partition_0(arr: Ptr<Option<Value<Box<[Edge]>>>>, mut start: i32, mut end: i32) -> i32 {
    let pivot: Ptr<Edge> = ({ (*arr.upgrade().deref()).decay().offset((start as usize)) });
    let mut count: i32 = 0;
    {
        let mut i: i32 = (start + 1);
        'loop_: while (i <= end) {
            if ({
                ({ (*arr.upgrade().deref()).decay().offset((i as usize)) }).with(|__s| __s.weight)
            } <= { pivot.with(|__s| __s.weight) })
            {
                count.postfix_inc();
            }
            i.prefix_inc();
        }
    }
    let mut pidx: i32 = (start + count);
    let tmp: Value<Edge> = Rc::new(RefCell::new(<Edge>::uninit()));
    {
        let __d = tmp.as_pointer();
        field!(__d, u).write(
            ({ (*arr.upgrade().deref()).decay().offset((pidx as usize)) }).with(|__s| __s.u),
        );
        field!(__d, v).write(
            ({ (*arr.upgrade().deref()).decay().offset((pidx as usize)) }).with(|__s| __s.v),
        );
        field!(__d, weight).write(
            ({ (*arr.upgrade().deref()).decay().offset((pidx as usize)) }).with(|__s| __s.weight),
        );
    };
    let __rhs = Edge {
        u: ({ (*arr.upgrade().deref()).decay().offset((start as usize)) }).with(|__s| __s.u),
        v: ({ (*arr.upgrade().deref()).decay().offset((start as usize)) }).with(|__s| __s.v),
        weight: ({ (*arr.upgrade().deref()).decay().offset((start as usize)) })
            .with(|__s| __s.weight),
    };
    ({ (*arr.upgrade().deref()).decay().offset((pidx as usize)) }).write(__rhs);
    ({ (*arr.upgrade().deref()).decay().offset((start as usize)) }).write(Edge {
        u: { (*tmp.borrow()).u },
        v: { (*tmp.borrow()).v },
        weight: { (*tmp.borrow()).weight },
    });
    let mut i: i32 = start;
    let mut j: i32 = end;
    'loop_: while (i < pidx) && (j > pidx) {
        'loop_: while ({
            ({ (*arr.upgrade().deref()).decay().offset((i as usize)) }).with(|__s| __s.weight)
        } <= { pivot.with(|__s| __s.weight) })
        {
            i.prefix_inc();
        }
        'loop_: while ({
            ({ (*arr.upgrade().deref()).decay().offset((j as usize)) }).with(|__s| __s.weight)
        } > { pivot.with(|__s| __s.weight) })
        {
            j.prefix_dec();
        }
        if (i < pidx) && (j > pidx) {
            (*tmp.borrow_mut()) = Edge {
                u: ({ (*arr.upgrade().deref()).decay().offset((i as usize)) }).with(|__s| __s.u),
                v: ({ (*arr.upgrade().deref()).decay().offset((i as usize)) }).with(|__s| __s.v),
                weight: ({ (*arr.upgrade().deref()).decay().offset((i as usize)) })
                    .with(|__s| __s.weight),
            };
            let __rhs = Edge {
                u: ({ (*arr.upgrade().deref()).decay().offset((j as usize)) }).with(|__s| __s.u),
                v: ({ (*arr.upgrade().deref()).decay().offset((j as usize)) }).with(|__s| __s.v),
                weight: ({ (*arr.upgrade().deref()).decay().offset((j as usize)) })
                    .with(|__s| __s.weight),
            };
            ({ (*arr.upgrade().deref()).decay().offset((i as usize)) }).write(__rhs);
            ({ (*arr.upgrade().deref()).decay().offset((j as usize)) }).write(Edge {
                u: { (*tmp.borrow()).u },
                v: { (*tmp.borrow()).v },
                weight: { (*tmp.borrow()).weight },
            });
            i.postfix_inc();
            j.postfix_dec();
        }
    }
    return pidx;
}
pub fn quicksort_1(arr: Ptr<Option<Value<Box<[Edge]>>>>, mut start: i32, mut end: i32) {
    if (start >= end) {
        return;
    }
    let mut p: i32 = ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = start;
        let _end: i32 = end;
        partition_0(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = start;
        let _end: i32 = (p - 1);
        quicksort_1(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = (p + 1);
        let _end: i32 = end;
        quicksort_1(_arr, _start, _end)
    });
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct DisjointSet {
    #[offset(0)]
    #[byte_size(8)]
    pub rank: Option<Value<Box<[i32]>>>,
    #[offset(8)]
    #[byte_size(8)]
    pub parent: Option<Value<Box<[i32]>>>,
    #[offset(16)]
    pub n: i32,
}
impl DisjointSet {
    pub fn move_from(this: Ptr<DisjointSet>, _a0: Ptr<DisjointSet>) {
        field!(this, rank)
            .write(field!(_a0, rank).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()));
        field!(this, parent)
            .write(field!(_a0, parent).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take()));
        field!(this, n).write({ (*_a0.upgrade().deref()).n });
    }
    pub fn new_1(this: Ptr<DisjointSet>) {
        field!(this, rank).write(None);
        field!(this, parent).write(None);
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct Graph {
    #[offset(0)]
    #[byte_size(8)]
    pub edges: Option<Value<Box<[Edge]>>>,
    #[offset(8)]
    pub V: i32,
    #[offset(12)]
    pub E: i32,
}
impl Graph {
    pub fn move_from(this: Ptr<Graph>, _a0: Ptr<Graph>) {
        field!(this, edges)
            .write(field!(_a0, edges).with_mut(|__v: &mut Option<Value<Box<[Edge]>>>| __v.take()));
        field!(this, V).write({ (*_a0.upgrade().deref()).V });
        field!(this, E).write({ (*_a0.upgrade().deref()).E });
    }
    pub fn new_1(this: Ptr<Graph>) {
        field!(this, edges).write(None);
    }
}
pub fn MSTKruskal_2(graph: Ptr<Graph>) -> f64 {
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = field_ptr!(graph, edges);
        let _end: i32 = (graph.with(|__s| __s.E) - 1);
        quicksort_1(_arr, 0, _end)
    });
    let set: Value<DisjointSet> = Rc::new(RefCell::new(<DisjointSet>::uninit()));
    {
        let __d = set.as_pointer();
        field!(__d, rank).write(Some(Rc::new(RefCell::new(
            (0..(graph.with(|__s| __s.V) as usize))
                .map(|_| 0_i32)
                .collect::<Box<[_]>>(),
        ))));
        field!(__d, parent).write(Some(Rc::new(RefCell::new(
            (0..(graph.with(|__s| __s.V) as usize))
                .map(|_| 0_i32)
                .collect::<Box<[_]>>(),
        ))));
        field!(__d, n).write(graph.with(|__s| __s.V));
    };
    ({ DisjointSetImpl::makeSet(&set.as_pointer()) });
    let mut total_weight: f64 = 0_f64;
    {
        let mut i: i32 = 0;
        'loop_: while ({ i } < { graph.with(|__s| __s.E) }) {
            let mut x: i32 = graph
                .with(|__s| __s.edges.clone())
                .decay()
                .offset((i as usize))
                .with(|__s| __s.u);
            let mut y: i32 = graph
                .with(|__s| __s.edges.clone())
                .decay()
                .offset((i as usize))
                .with(|__s| __s.v);
            let mut w: f64 = graph
                .with(|__s| __s.edges.clone())
                .decay()
                .offset((i as usize))
                .with(|__s| __s.weight);
            if (({ DisjointSetImpl::find(&set.as_pointer(), x) })
                != ({ DisjointSetImpl::find(&set.as_pointer(), y) }))
            {
                ({ DisjointSetImpl::merge(&set.as_pointer(), x, y) });
                total_weight += w;
            }
            i.prefix_inc();
        }
    }
    return total_weight;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut V: i32 = 4;
    let mut E: i32 = 5;
    let graph: Value<Graph> = Rc::new(RefCell::new(<Graph>::uninit()));
    {
        let __d = graph.as_pointer();
        field!(__d, edges).write(Some(Rc::new(RefCell::new(
            (0..(E as usize))
                .map(|_| <Edge>::default())
                .collect::<Box<[_]>>(),
        ))));
        field!(__d, V).write(V);
        field!(__d, E).write(E);
    };
    ({ { (*graph.borrow()).edges.clone() } })
        .decay()
        .offset(0_usize)
        .write(Edge {
            u: 0,
            v: 1,
            weight: 10_f64,
        });
    ({ { (*graph.borrow()).edges.clone() } })
        .decay()
        .offset(1_usize)
        .write(Edge {
            u: 1,
            v: 3,
            weight: 15_f64,
        });
    ({ { (*graph.borrow()).edges.clone() } })
        .decay()
        .offset(2_usize)
        .write(Edge {
            u: 2,
            v: 3,
            weight: 4_f64,
        });
    ({ { (*graph.borrow()).edges.clone() } })
        .decay()
        .offset(3_usize)
        .write(Edge {
            u: 2,
            v: 0,
            weight: 6_f64,
        });
    ({ { (*graph.borrow()).edges.clone() } })
        .decay()
        .offset(4_usize)
        .write(Edge {
            u: 0,
            v: 3,
            weight: 5_f64,
        });
    let mut total_weight: f64 = ({ MSTKruskal_2(graph.as_pointer()) });
    assert!((total_weight == 19_f64));
    return 0;
}
pub trait DisjointSetImpl {
    fn makeSet(&self);
    fn find(&self, x: i32) -> i32;
    fn merge(&self, x: i32, y: i32);
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet>;
}
impl DisjointSetImpl for Ptr<DisjointSet> {
    fn makeSet(&self) {
        {
            let mut i: i32 = 0;
            'loop_: while (i < (*self).with(|__s| __s.n)) {
                let __rhs = i;
                (*self)
                    .with(|__s| __s.parent.clone())
                    .decay()
                    .offset((i as usize))
                    .write(__rhs);
                (*self)
                    .with(|__s| __s.rank.clone())
                    .decay()
                    .offset((i as usize))
                    .write(1);
                i.postfix_inc();
            }
        }
    }
    fn find(&self, mut x: i32) -> i32 {
        if (((*self)
            .with(|__s| __s.parent.clone())
            .decay()
            .offset((x as usize))
            .read())
            != x)
        {
            let __rhs = ({
                let _x: i32 = ((*self)
                    .with(|__s| __s.parent.clone())
                    .decay()
                    .offset((x as usize))
                    .read());
                DisjointSetImpl::find(self, _x)
            });
            (*self)
                .with(|__s| __s.parent.clone())
                .decay()
                .offset((x as usize))
                .write(__rhs);
        }
        return ((*self)
            .with(|__s| __s.parent.clone())
            .decay()
            .offset((x as usize))
            .read());
    }
    fn merge(&self, mut x: i32, mut y: i32) {
        let mut xset: i32 = ({ DisjointSetImpl::find(self, x) });
        let mut yset: i32 = ({ DisjointSetImpl::find(self, y) });
        if (xset == yset) {
            return;
        }
        if (((*self)
            .with(|__s| __s.rank.clone())
            .decay()
            .offset((xset as usize))
            .read())
            < ((*self)
                .with(|__s| __s.rank.clone())
                .decay()
                .offset((yset as usize))
                .read()))
        {
            (*self)
                .with(|__s| __s.parent.clone())
                .decay()
                .offset((xset as usize))
                .write(yset);
        } else if (((*self)
            .with(|__s| __s.rank.clone())
            .decay()
            .offset((xset as usize))
            .read())
            > ((*self)
                .with(|__s| __s.rank.clone())
                .decay()
                .offset((yset as usize))
                .read()))
        {
            (*self)
                .with(|__s| __s.parent.clone())
                .decay()
                .offset((yset as usize))
                .write(xset);
        } else {
            (*self)
                .with(|__s| __s.parent.clone())
                .decay()
                .offset((yset as usize))
                .write(xset);
            let __rhs = (((*self)
                .with(|__s| __s.rank.clone())
                .decay()
                .offset((xset as usize))
                .read())
                + 1);
            (*self)
                .with(|__s| __s.rank.clone())
                .decay()
                .offset((xset as usize))
                .write(__rhs);
        }
    }
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet> {
        field!((*self), rank).with_mut(|__b0| {
            (*__b0) = field!(_a0, rank).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take());
        });
        field!((*self), parent).with_mut(|__b0| {
            (*__b0) =
                field!(_a0, parent).with_mut(|__v: &mut Option<Value<Box<[i32]>>>| __v.take());
        });
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        return (*self).clone();
    }
}
pub trait GraphImpl {
    fn move_assign(&self, _a0: Ptr<Graph>) -> Ptr<Graph>;
}
impl GraphImpl for Ptr<Graph> {
    fn move_assign(&self, _a0: Ptr<Graph>) -> Ptr<Graph> {
        field!((*self), edges).with_mut(|__b0| {
            (*__b0) =
                field!(_a0, edges).with_mut(|__v: &mut Option<Value<Box<[Edge]>>>| __v.take());
        });
        field!((*self), V).write({ { (*_a0.upgrade().deref()).V } });
        field!((*self), E).write({ { (*_a0.upgrade().deref()).E } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
