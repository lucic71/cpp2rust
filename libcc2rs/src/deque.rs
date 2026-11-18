// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

use crate::reinterpret::ByteRepr;

pub struct DequeBuf<T> {
    pub(crate) elems: VecDeque<T>,
    pub(crate) base: isize,
}

#[cold]
#[inline(never)]
#[track_caller]
fn out_of_bounds() -> ! {
    panic!("ub: out of bounds deque access")
}

impl<T> DequeBuf<T> {
    #[inline(always)]
    pub(crate) fn index(&self, pos: usize) -> usize {
        (pos as isize).wrapping_sub(self.base) as usize
    }

    #[inline(always)]
    pub(crate) fn pos(&self, idx: usize) -> usize {
        self.base.wrapping_add(idx as isize) as usize
    }

    #[inline(always)]
    pub(crate) fn end(&self) -> usize {
        self.pos(self.elems.len())
    }

    #[inline(always)]
    #[track_caller]
    pub(crate) fn at_pos(&self, pos: usize) -> &T {
        self.elems
            .get(self.index(pos))
            .unwrap_or_else(|| out_of_bounds())
    }

    #[inline(always)]
    #[track_caller]
    pub(crate) fn at_pos_mut(&mut self, pos: usize) -> &mut T {
        let idx = self.index(pos);
        self.elems.get_mut(idx).unwrap_or_else(|| out_of_bounds())
    }

    #[inline(always)]
    pub(crate) fn as_slice_mut(&mut self) -> &mut [T] {
        self.elems.make_contiguous()
    }

    pub(crate) fn contiguous(cell: &RefCell<Self>) -> Ref<'_, [T]> {
        if !cell.borrow().elems.as_slices().1.is_empty() {
            cell.borrow_mut().elems.make_contiguous();
        }
        Ref::map(cell.borrow(), |d| d.elems.as_slices().0)
    }
}

pub struct Deque<T> {
    pub(crate) buf: Rc<RefCell<DequeBuf<T>>>,
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::from_elems(VecDeque::new())
    }
}

impl<T: Clone> Clone for Deque<T> {
    fn clone(&self) -> Self {
        Self::from_elems(self.buf.borrow().elems.clone())
    }
}

impl<T: 'static> ByteRepr for Deque<T> {}

impl<T> Deque<T> {
    fn from_elems(elems: VecDeque<T>) -> Self {
        Self {
            buf: Rc::new(RefCell::new(DequeBuf { elems, base: 0 })),
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_fn(n: usize, mut f: impl FnMut() -> T) -> Self {
        Self::from_elems((0..n).map(|_| f()).collect())
    }

    pub fn len(&self) -> usize {
        self.buf.borrow().elems.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.borrow().elems.is_empty()
    }

    pub fn push_back(&mut self, value: T) {
        self.buf.borrow_mut().elems.push_back(value)
    }

    pub fn push_front(&mut self, value: T) {
        let mut buf = self.buf.borrow_mut();
        buf.elems.push_front(value);
        buf.base -= 1;
    }

    pub fn pop_back(&mut self) {
        let elem = self.buf.borrow_mut().elems.pop_back();
        drop(elem.expect("ub: pop_back of an empty deque"));
    }

    pub fn pop_front(&mut self) {
        let elem = {
            let mut buf = self.buf.borrow_mut();
            buf.base += 1;
            buf.elems.pop_front()
        };
        drop(elem.expect("ub: pop_front of an empty deque"));
    }

    pub fn insert(&mut self, pos: usize, value: T) {
        let mut buf = self.buf.borrow_mut();
        let idx = buf.index(pos);
        buf.elems.insert(idx, value)
    }

    pub fn erase(&mut self, pos: usize) {
        let elem = {
            let mut buf = self.buf.borrow_mut();
            let idx = buf.index(pos);
            buf.elems.remove(idx)
        };
        drop(elem.expect("ub: erase of an invalid deque iterator"));
    }

    pub fn clear(&mut self) {
        let elems = {
            let mut buf = self.buf.borrow_mut();
            buf.base += buf.elems.len() as isize;
            std::mem::take(&mut buf.elems)
        };
        drop(elems);
    }
}
