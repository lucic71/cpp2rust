// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::{Seek, Write};

unsafe fn f7<T1>(a0: *mut T1, a1: *mut T1) {
    cpp_swap_unsafe!(a0, a1)
}
