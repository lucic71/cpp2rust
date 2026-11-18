// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::{Ref, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;

use crate::rc::PtrKind;
use crate::{
    ByteRepr, CppRange, ElemStorage, Ptr, RandomAccessIterator, SliceParent, SliceRoot,
    UnsafeRandomAccessIterator,
};

pub trait DequeElems<P>: Clone {
    fn elem_at(&self, idx: isize) -> P;
    fn same_deque(&self, other: &Self) -> bool;
}

impl<T> DequeElems<*mut T> for *mut VecDeque<T> {
    fn elem_at(&self, idx: isize) -> *mut T {
        unsafe { &mut (&mut **self)[idx as usize] as *mut T }
    }

    fn same_deque(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl<T> CppRange for VecDeque<T> {
    type Iter = UnsafeRandomAccessIterator<T>;
    fn begin(&self) -> Self::Iter {
        UnsafeRandomAccessIterator::Deque(self as *const Self as *mut Self, 0)
    }
    fn end(&self) -> Self::Iter {
        UnsafeRandomAccessIterator::Deque(self as *const Self as *mut Self, self.len() as isize)
    }
}

#[derive(Clone)]
pub struct Deque<T> {
    elems: VecDeque<T>,
    base: isize,
}

pub struct DequeRef<T>(Rc<dyn SliceParent<T>>);

impl<T> Clone for DequeRef<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> DequeElems<Ptr<T>> for DequeRef<T> {
    fn elem_at(&self, idx: isize) -> Ptr<T> {
        Ptr {
            offset: idx as usize,
            kind: PtrKind::Nested(Rc::clone(&self.0)),
        }
    }

    fn same_deque(&self, other: &Self) -> bool {
        self.0.address() == other.0.address()
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self {
            elems: VecDeque::new(),
            base: 0,
        }
    }
}

impl<T: 'static> ByteRepr for Deque<T> {}

#[cold]
#[inline(never)]
#[track_caller]
fn out_of_bounds() -> ! {
    panic!("ub: out of bounds deque access")
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_fn(n: usize, mut f: impl FnMut() -> T) -> Self {
        Self {
            elems: (0..n).map(|_| f()).collect(),
            base: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.elems.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }

    fn index(&self, pos: usize) -> usize {
        (pos as isize).wrapping_sub(self.base) as usize
    }

    fn at_pos(&self, pos: usize) -> &T {
        self.elems
            .get(self.index(pos))
            .unwrap_or_else(|| out_of_bounds())
    }

    fn at_pos_mut(&mut self, pos: usize) -> &mut T {
        let idx = self.index(pos);
        self.elems.get_mut(idx).unwrap_or_else(|| out_of_bounds())
    }

    pub fn push_back(&mut self, value: T) {
        self.elems.push_back(value)
    }

    pub fn push_front(&mut self, value: T) {
        self.elems.push_front(value);
        self.base -= 1;
    }

    pub fn pop_back(&mut self) {
        drop(
            self.elems
                .pop_back()
                .expect("ub: pop_back of an empty deque"),
        );
    }

    pub fn pop_front(&mut self) {
        drop(
            self.elems
                .pop_front()
                .expect("ub: pop_front of an empty deque"),
        );
        self.base += 1;
    }

    pub fn insert(&mut self, pos: usize, value: T) {
        let idx = self.index(pos);
        self.elems.insert(idx, value)
    }

    pub fn erase(&mut self, pos: usize) {
        let idx = self.index(pos);
        drop(
            self.elems
                .remove(idx)
                .expect("ub: erase of an invalid deque iterator"),
        );
    }

    pub fn clear(&mut self) {
        self.base += self.elems.len() as isize;
        self.elems.clear()
    }
}

struct InDeque<T>(Ptr<Deque<T>>);

impl<T: 'static> SliceParent<T> for InDeque<T> {
    fn root(&self) -> SliceRoot {
        self.0.slice_root()
    }

    fn elem_storage(&self) -> ElemStorage {
        let root = self.root();
        let address = self.borrow_slice(&root).as_ptr() as usize;
        ElemStorage::Own(address, self.0.borrow_in(&root).elems.capacity())
    }

    fn borrow_slice<'a>(&self, root: &'a SliceRoot) -> Ref<'a, [T]> {
        if !self.0.borrow_in(root).elems.as_slices().1.is_empty() {
            self.0.borrow_in_mut(root).elems.make_contiguous();
        }
        Ref::map(self.0.borrow_in(root), |d| d.elems.as_slices().0)
    }

    fn borrow_slice_mut<'a>(&self, root: &'a SliceRoot) -> RefMut<'a, [T]> {
        RefMut::map(self.0.borrow_in_mut(root), |d| d.elems.make_contiguous())
    }

    fn address(&self) -> usize {
        self.0.address()
    }

    fn start(&self, root: &SliceRoot) -> usize {
        self.0.borrow_in(root).base as usize
    }

    fn borrow_elem<'a>(&self, root: &'a SliceRoot, idx: usize) -> Ref<'a, T> {
        Ref::map(self.0.borrow_in(root), |d| d.at_pos(idx))
    }

    fn borrow_elem_mut<'a>(&self, root: &'a SliceRoot, idx: usize) -> RefMut<'a, T> {
        RefMut::map(self.0.borrow_in_mut(root), |d| d.at_pos_mut(idx))
    }

    fn with_elem(&self, idx: usize, f: &mut dyn FnMut(&T)) {
        self.0.with(|d| f(d.at_pos(idx)))
    }

    fn with_elem_mut(&self, idx: usize, f: &mut dyn FnMut(&mut T)) {
        self.0.with_mut(|d| f(d.at_pos_mut(idx)))
    }
}

impl<T: 'static> Ptr<Deque<T>> {
    fn elems_parent(&self) -> Rc<dyn SliceParent<T>> {
        Rc::new(InDeque(self.clone()))
    }

    fn elem_ptr(&self, pos: isize) -> Ptr<T> {
        Ptr {
            offset: pos as usize,
            kind: PtrKind::Nested(self.elems_parent()),
        }
    }

    pub fn get(&self, idx: usize) -> Ptr<T> {
        let base = self.with(|d| {
            if idx >= d.len() {
                out_of_bounds()
            }
            d.base
        });
        self.elem_ptr(base.wrapping_add(idx as isize))
    }

    pub fn at(&self, idx: usize) -> Ptr<T> {
        if idx >= self.with(|d| d.len()) {
            panic!("out of bounds access")
        }
        self.get(idx)
    }

    pub fn front(&self) -> Ptr<T> {
        self.get(0)
    }

    pub fn back(&self) -> Ptr<T> {
        self.get(self.with(|d| d.len()).wrapping_sub(1))
    }

    pub fn begin(&self) -> RandomAccessIterator<T> {
        let base = self.with(|d| d.base);
        RandomAccessIterator::Deque(DequeRef(self.elems_parent()), base)
    }

    pub fn end(&self) -> RandomAccessIterator<T> {
        let end = self.with(|d| d.base.wrapping_add(d.len() as isize));
        RandomAccessIterator::Deque(DequeRef(self.elems_parent()), end)
    }
}
