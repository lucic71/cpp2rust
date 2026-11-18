// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
pub use std::collections::VecDeque;
use std::rc::{Rc, Weak};

use crate::deque::DequeBuf;
use crate::rc::PtrKind;
use crate::{CppRange, Deque, Ptr, RandomAccessIterator, UnsafeRandomAccessIterator};

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

#[cold]
#[inline(never)]
#[track_caller]
fn out_of_bounds() -> ! {
    panic!("ub: out of bounds deque access")
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
