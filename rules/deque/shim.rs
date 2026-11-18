// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

use crate::{
    AsPointer, ByteRepr, CppRange, Ptr, RandomAccessIterator, UnsafeRandomAccessIterator, Value,
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

pub struct Deque<T> {
    elems: Value<VecDeque<Value<T>>>,
}

pub struct DequeRef<T>(Weak<RefCell<VecDeque<Value<T>>>>);

impl<T> Clone for DequeRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> DequeElems<Ptr<T>> for DequeRef<T> {
    fn elem_at(&self, idx: isize) -> Ptr<T> {
        let elems = self
            .0
            .upgrade()
            .expect("ub: iterator into a destroyed deque");
        let elems = elems.borrow();
        usize::try_from(idx)
            .ok()
            .and_then(|idx| elems.get(idx))
            .expect("ub: dereference of an invalid deque iterator")
            .as_pointer()
    }

    fn same_deque(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self {
            elems: Rc::new(RefCell::new(VecDeque::new())),
        }
    }
}

impl<T: 'static> ByteRepr for Deque<T> {}

impl<T: Clone> Clone for Deque<T> {
    fn clone(&self) -> Self {
        let elems = self
            .elems
            .borrow()
            .iter()
            .map(|elem| Rc::new(RefCell::new(elem.borrow().clone())))
            .collect();
        Self {
            elems: Rc::new(RefCell::new(elems)),
        }
    }
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_fn(n: usize, mut f: impl FnMut() -> T) -> Self {
        let elems = (0..n).map(|_| Rc::new(RefCell::new(f()))).collect();
        Self {
            elems: Rc::new(RefCell::new(elems)),
        }
    }

    pub fn len(&self) -> usize {
        self.elems.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.elems.borrow().is_empty()
    }

    pub fn get(&self, idx: usize) -> Ptr<T> {
        self.elems
            .borrow()
            .get(idx)
            .expect("ub: out of bounds deque access")
            .as_pointer()
    }

    pub fn at(&self, idx: usize) -> Ptr<T> {
        if idx >= self.len() {
            panic!("out of bounds access")
        }
        self.get(idx)
    }

    pub fn front(&self) -> Ptr<T> {
        self.get(0)
    }

    pub fn back(&self) -> Ptr<T> {
        self.get(self.len().wrapping_sub(1))
    }

    pub fn push_back(&self, value: T) {
        self.elems
            .borrow_mut()
            .push_back(Rc::new(RefCell::new(value)))
    }

    pub fn push_front(&self, value: T) {
        self.elems
            .borrow_mut()
            .push_front(Rc::new(RefCell::new(value)))
    }

    pub fn pop_back(&self) {
        let elem = self.elems.borrow_mut().pop_back();
        drop(elem.expect("ub: pop_back of an empty deque"));
    }

    pub fn pop_front(&self) {
        let elem = self.elems.borrow_mut().pop_front();
        drop(elem.expect("ub: pop_front of an empty deque"));
    }

    pub fn insert(&self, idx: usize, value: T) {
        self.elems
            .borrow_mut()
            .insert(idx, Rc::new(RefCell::new(value)))
    }

    pub fn erase(&self, idx: usize) {
        let elem = self.elems.borrow_mut().remove(idx);
        drop(elem.expect("ub: erase of an invalid deque iterator"));
    }

    pub fn clear(&self) {
        let elems = std::mem::take(&mut *self.elems.borrow_mut());
        drop(elems);
    }

    pub fn begin(&self) -> RandomAccessIterator<T> {
        RandomAccessIterator::Deque(DequeRef(Rc::downgrade(&self.elems)), 0)
    }

    pub fn end(&self) -> RandomAccessIterator<T> {
        RandomAccessIterator::Deque(DequeRef(Rc::downgrade(&self.elems)), self.len() as isize)
    }
}
