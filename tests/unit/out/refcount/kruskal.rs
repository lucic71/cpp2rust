extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Edge {
    pub u: i32,
    pub v: i32,
    pub weight: f64,
}
impl Clone for Edge {
    fn clone(&self) -> Self {
        let mut this = Self {
            u: self.u,
            v: self.v,
            weight: self.weight,
        };
        this
    }
}
impl ByteRepr for Edge {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.u.to_bytes(&mut buf[0..4]);
        self.v.to_bytes(&mut buf[4..8]);
        self.weight.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            u: <i32>::from_bytes(&buf[0..4]),
            v: <i32>::from_bytes(&buf[4..8]),
            weight: <f64>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn partition_0(arr: Ptr<Option<Value<Box<[Edge]>>>>, start: i32, end: i32) -> i32 {
    let start: Value<i32> = Rc::new(RefCell::new(start));
    let end: Value<i32> = Rc::new(RefCell::new(end));
    let pivot: Ptr<Edge> = ((*arr.upgrade().deref())
        .as_ref()
        .unwrap()
        .as_pointer()
        .offset(((*start.borrow()) as usize)))
    .clone();
    let count: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(((*start.borrow()) + 1)));
    'loop_: while ((*i.borrow()) <= (*end.borrow())) {
        if {
            let _lhs = (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*i.borrow()) as usize) as usize]
                .weight;
            _lhs <= (*pivot.upgrade().deref()).weight
        } {
            (*count.borrow_mut()).postfix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let pidx: Value<i32> = Rc::new(RefCell::new(((*start.borrow()) + (*count.borrow()))));
    let tmp: Value<Edge> = Rc::new(RefCell::new(Edge {
        u: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
            [((*pidx.borrow()) as usize) as usize]
            .u,
        v: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
            [((*pidx.borrow()) as usize) as usize]
            .v,
        weight: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
            [((*pidx.borrow()) as usize) as usize]
            .weight,
    }));
    let __rhs = Edge {
        u: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
            [((*start.borrow()) as usize) as usize]
            .u,
        v: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
            [((*start.borrow()) as usize) as usize]
            .v,
        weight: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
            [((*start.borrow()) as usize) as usize]
            .weight,
    };
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()[((*pidx.borrow()) as usize) as usize] =
        __rhs;
    (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
        [((*start.borrow()) as usize) as usize] = Edge {
        u: (*tmp.borrow()).u,
        v: (*tmp.borrow()).v,
        weight: (*tmp.borrow()).weight,
    };
    let i: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
    let j: Value<i32> = Rc::new(RefCell::new((*end.borrow())));
    'loop_: while ((*i.borrow()) < (*pidx.borrow())) && ((*j.borrow()) > (*pidx.borrow())) {
        'loop_: while {
            let _lhs = (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*i.borrow()) as usize) as usize]
                .weight;
            _lhs <= (*pivot.upgrade().deref()).weight
        } {
            (*i.borrow_mut()).prefix_inc();
        }
        'loop_: while {
            let _lhs = (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                [((*j.borrow()) as usize) as usize]
                .weight;
            _lhs > (*pivot.upgrade().deref()).weight
        } {
            (*j.borrow_mut()).prefix_dec();
        }
        if ((*i.borrow()) < (*pidx.borrow())) && ((*j.borrow()) > (*pidx.borrow())) {
            (*tmp.borrow_mut()) = Edge {
                u: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as usize) as usize]
                    .u,
                v: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as usize) as usize]
                    .v,
                weight: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as usize) as usize]
                    .weight,
            };
            let __rhs = Edge {
                u: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*j.borrow()) as usize) as usize]
                    .u,
                v: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*j.borrow()) as usize) as usize]
                    .v,
                weight: (*arr.upgrade().deref()).as_ref().unwrap().borrow()
                    [((*j.borrow()) as usize) as usize]
                    .weight,
            };
            (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
                [((*i.borrow()) as usize) as usize] = __rhs;
            (*arr.upgrade().deref()).as_ref().unwrap().borrow_mut()
                [((*j.borrow()) as usize) as usize] = Edge {
                u: (*tmp.borrow()).u,
                v: (*tmp.borrow()).v,
                weight: (*tmp.borrow()).weight,
            };
            (*i.borrow_mut()).postfix_inc();
            (*j.borrow_mut()).postfix_dec();
        }
    }
    return (*pidx.borrow());
}
pub fn quicksort_1(arr: Ptr<Option<Value<Box<[Edge]>>>>, start: i32, end: i32) {
    let start: Value<i32> = Rc::new(RefCell::new(start));
    let end: Value<i32> = Rc::new(RefCell::new(end));
    if ((*start.borrow()) >= (*end.borrow())) {
        return;
    }
    let p: Value<i32> = Rc::new(RefCell::new(
        ({
            let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
            let _start: i32 = (*start.borrow());
            let _end: i32 = (*end.borrow());
            partition_0(_arr, _start, _end)
        }),
    ));
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = (*start.borrow());
        let _end: i32 = ((*p.borrow()) - 1);
        quicksort_1(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (arr).clone();
        let _start: i32 = ((*p.borrow()) + 1);
        let _end: i32 = (*end.borrow());
        quicksort_1(_arr, _start, _end)
    });
}
#[derive(Default)]
pub struct DisjointSet {
    pub rank: Option<Value<Box<[i32]>>>,
    pub parent: Option<Value<Box<[i32]>>>,
    pub n: i32,
}
impl DisjointSet {
    pub fn makeSet(&self) {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < self.n) {
            let __rhs = (*i.borrow());
            self.parent.as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
            self.rank.as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = 1;
            (*i.borrow_mut()).postfix_inc();
        }
    }
    pub fn find(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        if (self.parent.as_ref().unwrap().borrow()[((*x.borrow()) as usize) as usize]
            != (*x.borrow()))
        {
            let __rhs = ({
                let _x: i32 =
                    self.parent.as_ref().unwrap().borrow()[((*x.borrow()) as usize) as usize];
                self.find(_x)
            });
            self.parent.as_ref().unwrap().borrow_mut()[((*x.borrow()) as usize) as usize] = __rhs;
        }
        return self.parent.as_ref().unwrap().borrow()[((*x.borrow()) as usize) as usize];
    }
    pub fn merge(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let xset: Value<i32> = Rc::new(RefCell::new(({ self.find((*x.borrow())) })));
        let yset: Value<i32> = Rc::new(RefCell::new(({ self.find((*y.borrow())) })));
        if ((*xset.borrow()) == (*yset.borrow())) {
            return;
        }
        if (self.rank.as_ref().unwrap().borrow()[((*xset.borrow()) as usize) as usize]
            < self.rank.as_ref().unwrap().borrow()[((*yset.borrow()) as usize) as usize])
        {
            self.parent.as_ref().unwrap().borrow_mut()[((*xset.borrow()) as usize) as usize] =
                (*yset.borrow());
        } else if (self.rank.as_ref().unwrap().borrow()[((*xset.borrow()) as usize) as usize]
            > self.rank.as_ref().unwrap().borrow()[((*yset.borrow()) as usize) as usize])
        {
            self.parent.as_ref().unwrap().borrow_mut()[((*yset.borrow()) as usize) as usize] =
                (*xset.borrow());
        } else {
            self.parent.as_ref().unwrap().borrow_mut()[((*yset.borrow()) as usize) as usize] =
                (*xset.borrow());
            let __rhs =
                (self.rank.as_ref().unwrap().borrow()[((*xset.borrow()) as usize) as usize] + 1);
            self.rank.as_ref().unwrap().borrow_mut()[((*xset.borrow()) as usize) as usize] = __rhs;
        }
    }
}
impl ByteRepr for DisjointSet {}
#[derive(Default)]
pub struct Graph {
    pub edges: Option<Value<Box<[Edge]>>>,
    pub V: i32,
    pub E: i32,
}
impl ByteRepr for Graph {}
pub fn MSTKruskal_2(graph: Ptr<Graph>) -> f64 {
    ({
        let _arr: Ptr<Option<Value<Box<[Edge]>>>> = (*graph.upgrade().deref()).edges.as_pointer();
        let _end: i32 = ((*graph.upgrade().deref()).E - 1);
        quicksort_1(_arr, 0, _end)
    });
    let set: Value<DisjointSet> = Rc::new(RefCell::new(DisjointSet {
        rank: Some(Rc::new(RefCell::new(
            (0..((*graph.upgrade().deref()).V as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ))),
        parent: Some(Rc::new(RefCell::new(
            (0..((*graph.upgrade().deref()).V as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        ))),
        n: (*graph.upgrade().deref()).V,
    }));
    ({ (*set.borrow()).makeSet() });
    let total_weight: Value<f64> = Rc::new(RefCell::new(0_f64));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*graph.upgrade().deref()).E
    } {
        let x: Value<i32> = Rc::new(RefCell::new(
            (*graph.upgrade().deref()).edges.as_ref().unwrap().borrow()
                [((*i.borrow()) as usize) as usize]
                .u,
        ));
        let y: Value<i32> = Rc::new(RefCell::new(
            (*graph.upgrade().deref()).edges.as_ref().unwrap().borrow()
                [((*i.borrow()) as usize) as usize]
                .v,
        ));
        let w: Value<f64> = Rc::new(RefCell::new(
            (*graph.upgrade().deref()).edges.as_ref().unwrap().borrow()
                [((*i.borrow()) as usize) as usize]
                .weight,
        ));
        if (({ (*set.borrow()).find((*x.borrow())) }) != ({ (*set.borrow()).find((*y.borrow())) }))
        {
            ({ (*set.borrow()).merge((*x.borrow()), (*y.borrow())) });
            (*total_weight.borrow_mut()) += (*w.borrow());
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return (*total_weight.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let V: Value<i32> = Rc::new(RefCell::new(4));
    let E: Value<i32> = Rc::new(RefCell::new(5));
    let graph: Value<Graph> = Rc::new(RefCell::new(Graph {
        edges: Some(Rc::new(RefCell::new(
            (0..((*E.borrow()) as usize))
                .map(|_| <Edge>::default())
                .collect::<Box<[_]>>(),
        ))),
        V: (*V.borrow()),
        E: (*E.borrow()),
    }));
    (*graph.borrow()).edges.as_ref().unwrap().borrow_mut()[(0_usize) as usize] = Edge {
        u: 0,
        v: 1,
        weight: 10_f64,
    };
    (*graph.borrow()).edges.as_ref().unwrap().borrow_mut()[(1_usize) as usize] = Edge {
        u: 1,
        v: 3,
        weight: 15_f64,
    };
    (*graph.borrow()).edges.as_ref().unwrap().borrow_mut()[(2_usize) as usize] = Edge {
        u: 2,
        v: 3,
        weight: 4_f64,
    };
    (*graph.borrow()).edges.as_ref().unwrap().borrow_mut()[(3_usize) as usize] = Edge {
        u: 2,
        v: 0,
        weight: 6_f64,
    };
    (*graph.borrow()).edges.as_ref().unwrap().borrow_mut()[(4_usize) as usize] = Edge {
        u: 0,
        v: 3,
        weight: 5_f64,
    };
    let total_weight: Value<f64> = Rc::new(RefCell::new(({ MSTKruskal_2(graph.as_pointer()) })));
    return ((*total_weight.borrow()) as i32);
}
