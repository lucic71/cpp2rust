extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Edge {
    #[offset(0)]
    pub u: (),
    #[offset(4)]
    pub v: (),
    #[offset(8)]
    pub weight: (),
}
pub fn partition_0(arr: Ptr<()>, mut start: (), mut end: ()) -> () {
    let pivot: Ptr<Edge> = unimplemented!();
    let mut count: () = 0_();
    let mut i: () = (start + 1_());
    'loop_: while (i <= end) {
        if ({ unimplemented!().with(|__s| __s.weight) } <= { pivot.with(|__s| __s.weight) }) {
            count.postfix_inc();
        }
        i.prefix_inc();
    }
    let pidx: Value<()> = Rc::new(RefCell::new((start + count)));
    let mut tmp: Edge = Edge {
        u: unimplemented!().with(|__s| __s.u),
        v: unimplemented!().with(|__s| __s.v),
        weight: unimplemented!().with(|__s| __s.weight),
    };
    let __rhs = Edge {
        u: unimplemented!().with(|__s| __s.u),
        v: unimplemented!().with(|__s| __s.v),
        weight: unimplemented!().with(|__s| __s.weight),
    };
    unimplemented!().write(__rhs);
    unimplemented!().write(Edge {
        u: tmp.u,
        v: tmp.v,
        weight: tmp.weight,
    });
    let mut i: () = start;
    let mut j: () = end;
    'loop_: while (i < (*pidx.borrow())) && (j > (*pidx.borrow())) {
        'loop_: while ({ unimplemented!().with(|__s| __s.weight) } <= {
            pivot.with(|__s| __s.weight)
        }) {
            i.prefix_inc();
        }
        'loop_: while ({ unimplemented!().with(|__s| __s.weight) } > {
            pivot.with(|__s| __s.weight)
        }) {
            j.prefix_dec();
        }
        if (i < (*pidx.borrow())) && (j > (*pidx.borrow())) {
            tmp = Edge {
                u: unimplemented!().with(|__s| __s.u),
                v: unimplemented!().with(|__s| __s.v),
                weight: unimplemented!().with(|__s| __s.weight),
            };
            let __rhs = Edge {
                u: unimplemented!().with(|__s| __s.u),
                v: unimplemented!().with(|__s| __s.v),
                weight: unimplemented!().with(|__s| __s.weight),
            };
            unimplemented!().write(__rhs);
            unimplemented!().write(Edge {
                u: tmp.u,
                v: tmp.v,
                weight: tmp.weight,
            });
            i.postfix_inc();
            j.postfix_dec();
        }
    }
    return (*pidx.borrow());
}
pub fn quicksort_1(arr: Ptr<()>, mut start: (), mut end: ()) {
    if (start >= end) {
        return;
    }
    let mut p: () = ({
        let _arr: Ptr<()> = (arr).clone();
        let _start: () = start;
        let _end: () = end;
        partition_0(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<()> = (arr).clone();
        let _start: () = start;
        let _end: () = (p - 1_());
        quicksort_1(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<()> = (arr).clone();
        let _start: () = (p + 1_());
        let _end: () = end;
        quicksort_1(_arr, _start, _end)
    });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct DisjointSet {
    #[offset(0)]
    #[byte_size(8)]
    pub rank: (),
    #[offset(8)]
    #[byte_size(8)]
    pub parent: (),
    #[offset(16)]
    pub n: (),
}
impl DisjointSet {
    pub fn move_from(_a0: Ptr<DisjointSet>) -> Self {
        Self {
            rank: unimplemented!(),
            parent: unimplemented!(),
            n: { (*_a0.upgrade().deref()).n },
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct Graph {
    #[offset(0)]
    #[byte_size(8)]
    pub edges: (),
    #[offset(8)]
    pub V: (),
    #[offset(12)]
    pub E: (),
}
impl Graph {
    pub fn move_from(_a0: Ptr<Graph>) -> Self {
        Self {
            edges: unimplemented!(),
            V: { (*_a0.upgrade().deref()).V },
            E: { (*_a0.upgrade().deref()).E },
        }
    }
}
pub fn MSTKruskal_2(graph: Ptr<Graph>) -> () {
    ({
        let _arr: Ptr<()> = field_ptr!(graph, edges);
        let _end: () = (graph.with(|__s| __s.E) - 1_());
        quicksort_1(_arr, 0_(), _end)
    });
    let set: Value<DisjointSet> = Rc::new(RefCell::new(DisjointSet {
        rank: unimplemented!(),
        parent: unimplemented!(),
        n: graph.with(|__s| __s.V),
    }));
    ({ DisjointSetImpl::makeSet(&set.as_pointer()) });
    let total_weight: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while ({ i } < { graph.with(|__s| __s.E) }) {
        let mut x: () = unimplemented!().with(|__s| __s.u);
        let mut y: () = unimplemented!().with(|__s| __s.v);
        let mut w: () = unimplemented!().with(|__s| __s.weight);
        if (({ DisjointSetImpl::find(&set.as_pointer(), x) })
            != ({ DisjointSetImpl::find(&set.as_pointer(), y) }))
        {
            ({ DisjointSetImpl::merge(&set.as_pointer(), x, y) });
            (*total_weight.borrow_mut()) += w;
        }
        i.prefix_inc();
    }
    return (*total_weight.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut V: () = 4_();
    let mut E: () = 5_();
    let graph: Value<Graph> = Rc::new(RefCell::new(Graph {
        edges: unimplemented!(),
        V: V,
        E: E,
    }));
    unimplemented!().write(Edge {
        u: 0_(),
        v: 1_(),
        weight: 10_(),
    });
    unimplemented!().write(Edge {
        u: 1_(),
        v: 3_(),
        weight: 15_(),
    });
    unimplemented!().write(Edge {
        u: 2_(),
        v: 3_(),
        weight: 4_(),
    });
    unimplemented!().write(Edge {
        u: 2_(),
        v: 0_(),
        weight: 6_(),
    });
    unimplemented!().write(Edge {
        u: 0_(),
        v: 3_(),
        weight: 5_(),
    });
    let mut total_weight: () = ({ MSTKruskal_2(graph.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub trait DisjointSetImpl {
    fn makeSet(&self);
    fn find(&self, x: ()) -> ();
    fn merge(&self, x: (), y: ());
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet>;
}
impl DisjointSetImpl for Ptr<DisjointSet> {
    fn makeSet(&self) {
        let mut i: () = 0_();
        'loop_: while (i < (*self).with(|__s| __s.n)) {
            let __rhs = i;
            unimplemented!().write(__rhs);
            unimplemented!().write(1_());
            i.postfix_inc();
        }
    }
    fn find(&self, mut x: ()) -> () {
        if ((unimplemented!().read()) != x) {
            let __rhs = ({
                let _x: () = (unimplemented!().read());
                DisjointSetImpl::find(self, _x)
            });
            unimplemented!().write(__rhs);
        }
        return (unimplemented!().read());
    }
    fn merge(&self, mut x: (), mut y: ()) {
        let mut xset: () = ({ DisjointSetImpl::find(self, x) });
        let mut yset: () = ({ DisjointSetImpl::find(self, y) });
        if (xset == yset) {
            return;
        }
        if ((unimplemented!().read()) < (unimplemented!().read())) {
            unimplemented!().write(yset);
        } else if ((unimplemented!().read()) > (unimplemented!().read())) {
            unimplemented!().write(xset);
        } else {
            unimplemented!().write(xset);
            let __rhs = ((unimplemented!().read()) + 1_());
            unimplemented!().write(__rhs);
        }
    }
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet> {
        unimplemented!();
        unimplemented!();
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        return (*self).clone();
    }
}
pub trait GraphImpl {
    fn move_assign(&self, _a0: Ptr<Graph>) -> Ptr<Graph>;
}
impl GraphImpl for Ptr<Graph> {
    fn move_assign(&self, _a0: Ptr<Graph>) -> Ptr<Graph> {
        unimplemented!();
        field!((*self), V).write({ { (*_a0.upgrade().deref()).V } });
        field!((*self), E).write({ { (*_a0.upgrade().deref()).E } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
