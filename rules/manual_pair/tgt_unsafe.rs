// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f15<T1, T2>(a0: (T1, T2)) -> T2 {
    a0.1
}

unsafe fn f16<T1, T2>(a0: (T1, T2)) -> T1 {
    a0.0
}
