// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::rc::Rc;

// The storage of an object before its constructor runs: a valid value with no
// meaning, whose creation and drop have no side effects.
pub trait Uninit {
    fn uninit() -> Self;
}

macro_rules! impl_uninit {
    ($($ty:ty => $value:expr),* $(,)?) => {$(
        impl Uninit for $ty {
            #[inline]
            fn uninit() -> Self {
                $value
            }
        }
    )*};
}

impl_uninit!(
    () => (),
    bool => false,
    u8 => 0,
    i8 => 0,
    u16 => 0,
    i16 => 0,
    u32 => 0,
    i32 => 0,
    u64 => 0,
    i64 => 0,
    u128 => 0,
    i128 => 0,
    usize => 0,
    isize => 0,
    f32 => 0.0,
    f64 => 0.0,
    std::cmp::Ordering => std::cmp::Ordering::Equal,
);

impl<T> Uninit for Option<T> {
    #[inline]
    fn uninit() -> Self {
        None
    }
}

impl<T> Uninit for Vec<T> {
    #[inline]
    fn uninit() -> Self {
        Vec::new()
    }
}

impl<T: Uninit> Uninit for Rc<RefCell<T>> {
    #[inline]
    fn uninit() -> Self {
        Rc::new(RefCell::new(T::uninit()))
    }
}

impl<T: Uninit, const N: usize> Uninit for [T; N] {
    #[inline]
    fn uninit() -> Self {
        std::array::from_fn(|_| T::uninit())
    }
}

impl<A: Uninit, B: Uninit> Uninit for (A, B) {
    #[inline]
    fn uninit() -> Self {
        (A::uninit(), B::uninit())
    }
}
