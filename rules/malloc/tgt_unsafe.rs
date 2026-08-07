// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *mut ::libc::c_void) -> usize {
    libcc2rs::malloc_usable_size(a0)
}
