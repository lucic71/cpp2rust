// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

fn f7<T1: Clone + ByteRepr>(a0: Ptr<T1>, a1: Ptr<T1>) {
    cpp_swap!(a0, a1)
}
