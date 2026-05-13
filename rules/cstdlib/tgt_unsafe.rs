// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1() {
    std::process::abort();
}

unsafe fn f2(a0: *mut ::libc::c_void) {
    libc::free(a0)
}

unsafe fn f3(a0: u64) -> *mut ::libc::c_void {
    libc::malloc(a0 as ::libc::size_t)
}

unsafe fn f4(a0: *mut ::libc::c_void, a1: u64) -> *mut ::libc::c_void {
    libc::realloc(a0, a1 as ::libc::size_t)
}
