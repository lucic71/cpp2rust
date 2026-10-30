// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f10<T2, T3>(a0: (Value<T2>, Value<T3>)) -> Ptr<T2> {
    a0.0.as_pointer()
}

fn f11<T2, T3>(a0: (Value<T2>, Value<T3>)) -> Ptr<T2> {
    a0.0.as_pointer()
}

fn f12<T2, T3>(a0: (Value<T2>, Value<T3>)) -> Ptr<T2> {
    a0.0.as_pointer()
}

fn f13<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (Rc::new(RefCell::new(a0)), Rc::new(RefCell::new(a1)))
}
