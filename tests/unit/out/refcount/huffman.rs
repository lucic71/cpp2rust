extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MinHeapNode {
    #[offset(0)]
    pub data: (),
    #[offset(4)]
    pub freq: (),
    #[offset(8)]
    #[byte_size(8)]
    pub left: Ptr<MinHeapNode>,
    #[offset(16)]
    #[byte_size(8)]
    pub right: Ptr<MinHeapNode>,
}
pub fn Swap_0(a: Ptr<MinHeapNode>, b: Ptr<MinHeapNode>) {
    let mut t: MinHeapNode = MinHeapNode {
        data: a.with(|__s| __s.data),
        freq: a.with(|__s| __s.freq),
        left: a.with(|__s| __s.left.clone()),
        right: a.with(|__s| __s.right.clone()),
    };
    a.write({
        MinHeapNode {
            data: b.with(|__s| __s.data),
            freq: b.with(|__s| __s.freq),
            left: b.with(|__s| __s.left.clone()),
            right: b.with(|__s| __s.right.clone()),
        }
    });
    b.write({
        MinHeapNode {
            data: t.data,
            freq: t.freq,
            left: (t.left).clone(),
            right: (t.right).clone(),
        }
    });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(32)]
pub struct MinHeap {
    #[offset(0)]
    pub size: (),
    #[offset(4)]
    pub capacity: (),
    #[offset(8)]
    #[byte_size(8)]
    pub arr: (),
    #[offset(16)]
    pub next: (),
    #[offset(24)]
    #[byte_size(8)]
    pub alloc: (),
}
impl MinHeap {
    pub fn move_from(_a0: Ptr<MinHeap>) -> Self {
        Self {
            size: { (*_a0.upgrade().deref()).size },
            capacity: { (*_a0.upgrade().deref()).capacity },
            arr: unimplemented!(),
            next: { (*_a0.upgrade().deref()).next },
            alloc: unimplemented!(),
        }
    }
}
pub fn AllocMinHeap_1(mut capacity: ()) -> () {
    let mut minHeap: () = unimplemented!();
    return unimplemented!();
}
pub fn Huffman_2(data: Ptr<()>, freq: Ptr<()>, mut size: ()) -> () {
    let mut minHeap: () = ({ AllocMinHeap_1(size) });
    ({
        let _data: Ptr<()> = (data).clone();
        let _freq: Ptr<()> = (freq).clone();
        let _n: () = size;
        MinHeapImpl::Build(&(unimplemented!()), _data, _freq, _n)
    });
    'loop_: while (unimplemented!().with(|__s| (__s).size) != 1_()) {
        let mut left: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(unimplemented!())) });
        let mut right: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(unimplemented!())) });
        let mut top: Ptr<MinHeapNode> = ({
            MinHeapImpl::Alloc(
                &(unimplemented!()),
                ('$' as ()),
                ({ left.with(|__s| __s.freq) } + { right.with(|__s| __s.freq) }),
            )
        });
        field!(top, left).write((left).clone());
        field!(top, right).write((right).clone());
        ({ MinHeapImpl::Insert(&(unimplemented!()), (top).clone()) });
    }
    return unimplemented!();
}
pub fn CollectCode_3(arr: Ptr<()>, mut top: (), out: Ptr<()>, next: Ptr<()>) {
    unimplemented!().write(0_());
    let mut i: () = 0_();
    'loop_: while (i < top) {
        let __rhs = ((unimplemented!().read()) * 10_());
        unimplemented!().write(__rhs);
        let __rhs = ({ (unimplemented!().read()) } + { (unimplemented!().read()) });
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    next.with_mut(|__v| __v.prefix_inc());
}
pub fn CollectCodes_4(
    mut root: Ptr<MinHeapNode>,
    arr: Ptr<()>,
    mut top: (),
    out: Ptr<()>,
    next: Ptr<()>,
) {
    if !((root.with(|__s| __s.left.clone())).is_null()) {
        unimplemented!().write(0_());
        ({
            let _root: Ptr<MinHeapNode> = root.with(|__s| __s.left.clone());
            let _arr: Ptr<()> = (arr).clone();
            let _top: () = (top + 1_());
            let _out: Ptr<()> = (out).clone();
            let _next: Ptr<()> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if !((root.with(|__s| __s.right.clone())).is_null()) {
        unimplemented!().write(1_());
        ({
            let _root: Ptr<MinHeapNode> = root.with(|__s| __s.right.clone());
            let _arr: Ptr<()> = (arr).clone();
            let _top: () = (top + 1_());
            let _out: Ptr<()> = (out).clone();
            let _next: Ptr<()> = (next).clone();
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if ({ MinHeapNodeImpl::IsLeaf(&root) }) {
        ({
            let _arr: Ptr<()> = (arr).clone();
            let _top: () = top;
            let _out: Ptr<()> = (out).clone();
            let _next: Ptr<()> = (next).clone();
            CollectCode_3(_arr, _top, _out, _next)
        });
    }
}
pub fn HuffmanCodes_5(data: Ptr<()>, freq: Ptr<()>, mut size: ()) -> () {
    let mut minHeap: () = ({
        let _data: Ptr<()> = (data).clone();
        let _freq: Ptr<()> = (freq).clone();
        let _size: () = size;
        Huffman_2(_data, _freq, _size)
    });
    let mut root: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(unimplemented!())) });
    let arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut top: () = 0_();
    let next: Value<()> = Rc::new(RefCell::new(0_()));
    ({
        CollectCodes_4(
            (root).clone(),
            arr.as_pointer(),
            top,
            out.as_pointer(),
            next.as_pointer(),
        )
    });
    return unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut size: () = 6_();
    let mut arr1: [(); 6] = [
        ('a' as ()),
        ('b' as ()),
        ('c' as ()),
        ('d' as ()),
        ('e' as ()),
        ('f' as ()),
    ];
    let mut arr2: [(); 6] = [5_(), 9_(), 12_(), 13_(), 16_(), 45_()];
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let freq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < size) {
        let __rhs = arr1[(i) as usize];
        unimplemented!().write(__rhs);
        let __rhs = arr2[(i) as usize];
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let mut out: () = ({ HuffmanCodes_5(data.as_pointer(), freq.as_pointer(), size) });
    unimplemented!();
    return 0_();
}
pub trait MinHeapImpl {
    fn Alloc(&self, data: (), freq: ()) -> Ptr<MinHeapNode>;
    fn Heapify(&self, idx: ());
    fn ExtractMin(&self) -> Ptr<MinHeapNode>;
    fn Insert(&self, node: Ptr<MinHeapNode>);
    fn Build(&self, data: Ptr<()>, freq: Ptr<()>, n: ());
    fn move_assign(&self, _a0: Ptr<MinHeap>) -> Ptr<MinHeap>;
}
impl MinHeapImpl for Ptr<MinHeap> {
    fn Alloc(&self, mut data: (), mut freq: ()) -> Ptr<MinHeapNode> {
        unimplemented!().write(MinHeapNode {
            data: data,
            freq: freq,
            left: Ptr::<MinHeapNode>::null(),
            right: Ptr::<MinHeapNode>::null(),
        });
        return (unimplemented!());
    }
    fn Heapify(&self, mut idx: ()) {
        let mut smallest: () = idx;
        let mut left: () = ((2_() * idx) + 1_());
        let mut right: () = ((2_() * idx) + 2_());
        if (left < (*self).with(|__s| __s.size))
            && ((unimplemented!().read()).with(|__s| __s.freq)
                < (unimplemented!().read()).with(|__s| __s.freq))
        {
            smallest = left;
        }
        if (right < (*self).with(|__s| __s.size))
            && ((unimplemented!().read()).with(|__s| __s.freq)
                < (unimplemented!().read()).with(|__s| __s.freq))
        {
            smallest = right;
        }
        if (smallest != idx) {
            ({
                let _a: Ptr<MinHeapNode> = (unimplemented!().read());
                let _b: Ptr<MinHeapNode> = (unimplemented!().read());
                Swap_0(_a, _b)
            });
            ({ MinHeapImpl::Heapify(self, smallest) });
        }
    }
    fn ExtractMin(&self) -> Ptr<MinHeapNode> {
        let out: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new((unimplemented!().read())));
        field!((*self), size).with_mut(|__v| __v.prefix_dec());
        let __rhs = (unimplemented!().read());
        unimplemented!().write(__rhs);
        ({ MinHeapImpl::Heapify(self, 0_()) });
        return (*out.borrow()).clone();
    }
    fn Insert(&self, mut node: Ptr<MinHeapNode>) {
        field!((*self), size).with_mut(|__v| __v.prefix_inc());
        let mut i: () = ((*self).with(|__s| __s.size) - 1_());
        'loop_: while (i != 0_())
            && ({ node.with(|__s| __s.freq) } < { (unimplemented!().read()).with(|__s| __s.freq) })
        {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            i = { ((i - 1_()) / 2_()) };
        }
        unimplemented!().write((node).clone());
    }
    fn Build(&self, data: Ptr<()>, freq: Ptr<()>, mut n: ()) {
        let mut i: () = 0_();
        'loop_: while (i < n) {
            unimplemented!().write(
                ({
                    let _data: () = (unimplemented!().read());
                    let _freq: () = (unimplemented!().read());
                    MinHeapImpl::Alloc(self, _data, _freq)
                }),
            );
            i.prefix_inc();
        }
        let mut i: () = (((*self).with(|__s| __s.size) - 2_()) / 2_());
        'loop_: while (i >= 0_()) {
            ({ MinHeapImpl::Heapify(self, i) });
            i.prefix_dec();
        }
    }
    fn move_assign(&self, _a0: Ptr<MinHeap>) -> Ptr<MinHeap> {
        field!((*self), size).write({ { (*_a0.upgrade().deref()).size } });
        field!((*self), capacity).write({ { (*_a0.upgrade().deref()).capacity } });
        unimplemented!();
        field!((*self), next).write({ { (*_a0.upgrade().deref()).next } });
        unimplemented!();
        return (*self).clone();
    }
}
pub trait MinHeapNodeImpl {
    fn IsLeaf(&self) -> ();
}
impl MinHeapNodeImpl for Ptr<MinHeapNode> {
    fn IsLeaf(&self) -> () {
        return (((*self).with(|__s| __s.left.clone())).is_null())
            && (((*self).with(|__s| __s.right.clone())).is_null());
    }
}
pub fn __cpp2rust_init_globals() {}
