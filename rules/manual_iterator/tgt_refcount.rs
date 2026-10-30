// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1<T1: CppIterator<T2>, T2>(a0: T1) -> Ptr<T2> {
    a0.iter_deref()
}
