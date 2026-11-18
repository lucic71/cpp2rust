// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::{Ref, RefCell};
pub use std::collections::VecDeque;
use std::rc::{Rc, Weak};

use crate::rc::PtrKind;
use crate::reinterpret::ByteRepr;
use crate::{CppRange, Ptr, RandomAccessIterator, UnsafeRandomAccessIterator};

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

pub struct DequeRef<T>(Weak<RefCell<DequeBuf<T>>>);

impl<T> Clone for DequeRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> DequeElems<Ptr<T>> for DequeRef<T> {
    fn elem_at(&self, idx: isize) -> Ptr<T> {
        Ptr {
            offset: idx as usize,
            kind: PtrKind::Deque(self.0.clone()),
        }
    }

    fn same_deque(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl<T: 'static> Ptr<Deque<T>> {
    #[inline]
    pub fn get(&self, idx: usize) -> Ptr<T> {
        let (buf, pos) = self.with(|d| {
            let b = d.buf.borrow();
            if idx >= b.elems.len() {
                out_of_bounds()
            }
            (Rc::downgrade(&d.buf), b.pos(idx))
        });
        Ptr {
            offset: pos,
            kind: PtrKind::Deque(buf),
        }
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
        let (buf, base) = self.with(|d| (Rc::downgrade(&d.buf), d.buf.borrow().base));
        RandomAccessIterator::Deque(DequeRef(buf), base)
    }

    pub fn end(&self) -> RandomAccessIterator<T> {
        let (buf, end) = self.with(|d| (Rc::downgrade(&d.buf), d.buf.borrow().end()));
        RandomAccessIterator::Deque(DequeRef(buf), end as isize)
    }
}
