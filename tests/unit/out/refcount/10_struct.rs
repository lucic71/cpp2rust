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
pub struct GraphNode {
    #[offset(0)]
    pub dst: u32,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<GraphNode>,
}
impl GraphNode {
    pub fn new_1(this: Ptr<GraphNode>) {}
    pub fn copy_from(this: Ptr<GraphNode>, _a0: Ptr<GraphNode>) {
        field!(this, dst).write(_a0.with(|__s| __s.dst));
        field!(this, next).write(_a0.with(|__s| __s.next.clone()));
    }
    pub fn move_from(this: Ptr<GraphNode>, _a0: Ptr<GraphNode>) {
        field!(this, dst).write({ (*_a0.upgrade().deref()).dst });
        field!(this, next).write({ (*_a0.upgrade().deref()).next.clone() });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Graph {
    #[offset(0)]
    pub V: u32,
    #[offset(8)]
    #[byte_size(8)]
    pub adj: Ptr<Ptr<GraphNode>>,
}
impl Graph {
    pub fn new_1(this: Ptr<Graph>) {}
    pub fn copy_from(this: Ptr<Graph>, _a0: Ptr<Graph>) {
        field!(this, V).write(_a0.with(|__s| __s.V));
        field!(this, adj).write(_a0.with(|__s| __s.adj.clone()));
    }
    pub fn move_from(this: Ptr<Graph>, _a0: Ptr<Graph>) {
        field!(this, V).write({ (*_a0.upgrade().deref()).V });
        field!(this, adj).write({ (*_a0.upgrade().deref()).adj.clone() });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Partial {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<i32>,
}
impl Partial {
    pub fn new_1(this: Ptr<Partial>) {
        field!(this, p).write(Ptr::<i32>::null());
    }
    pub fn new_2(this: Ptr<Partial>, mut q: Ptr<i32>) {
        field!(this, p).write((q).clone());
    }
    pub fn copy_from(this: Ptr<Partial>, _a0: Ptr<Partial>) {
        field!(this, p).write(_a0.with(|__s| __s.p.clone()));
    }
    pub fn move_from(this: Ptr<Partial>, _a0: Ptr<Partial>) {
        field!(this, p).write({ (*_a0.upgrade().deref()).p.clone() });
    }
}
impl Default for Partial {
    fn default() -> Self {
        {
            Partial {
                p: Ptr::<i32>::null(),
            }
        }
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Declared {}
impl Declared {
    pub fn copy_from(this: Ptr<Declared>, _a0: Ptr<Declared>) {}
    pub fn move_from(this: Ptr<Declared>, _a0: Ptr<Declared>) {}
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub i: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub d: Ptr<Declared>,
}
impl S {
    pub fn new_1(this: Ptr<S>) {}
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, i).write(_a0.with(|__s| __s.i));
        field!(this, d).write(_a0.with(|__s| __s.d.clone()));
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, i).write({ (*_a0.upgrade().deref()).i });
        field!(this, d).write({ (*_a0.upgrade().deref()).d.clone() });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let g: Value<Graph> = Rc::new(RefCell::new(<Graph>::uninit()));
    {
        let __d = g.as_pointer();
        field!(__d, V).write(5_u32);
        field!(__d, adj).write(Ptr::<Ptr<GraphNode>>::null());
    };
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 1, 4])));
    let it: Value<Partial> = Rc::new(RefCell::new(<Partial>::uninit()));
    Partial::new_2(it.as_pointer(), { arr.decay() });
    if ({ { (*it.borrow()).p.clone() } } != { arr.decay() }) {
        return 1;
    }
    let def: Value<Partial> = Rc::new(RefCell::new(<Partial>::uninit()));
    Partial::new_1(def.as_pointer());
    if !(({ (*def.borrow()).p.clone() }).is_null()) {
        return 1;
    }
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, i).write(7);
        field!(__d, d).write(Ptr::<Declared>::null());
    };
    if ({ (*s.borrow()).i } != 7) || (!(({ (*s.borrow()).d.clone() }).is_null())) {
        return 1;
    }
    return 0;
}
pub trait GraphImpl {
    fn push(&self, src: u32, dst: u32);
}
impl GraphImpl for Ptr<Graph> {
    fn push(&self, mut src: u32, mut dst: u32) {
        let __rhs = {
            let __p = Ptr::alloc(<GraphNode>::uninit());
            {
                let __d = __p.clone();
                field!(__d, dst).write(dst);
                field!(__d, next).write((elem!((*self).with(|__s| __s.adj.clone()), src).read()));
            };
            __p
        };
        elem!((*self).with(|__s| __s.adj.clone()), src).write(__rhs);
        let __rhs = {
            let __p = Ptr::alloc(<GraphNode>::uninit());
            {
                let __d = __p.clone();
                field!(__d, dst).write(src);
                field!(__d, next).write((elem!((*self).with(|__s| __s.adj.clone()), dst).read()));
            };
            __p
        };
        elem!((*self).with(|__s| __s.adj.clone()), dst).write(__rhs);
    }
}
pub trait PartialImpl {
    fn get(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn next_4(&self) -> Ptr<Partial> {
        unimplemented!()
    }
    fn next_5(&self, __ret: Ptr<Partial>, mut _a0: i32) {
        unimplemented!()
    }
}
impl PartialImpl for Ptr<Partial> {}
pub fn __cpp2rust_init_globals() {}
