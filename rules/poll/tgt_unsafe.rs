// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *mut ::libc::pollfd, a1: ::libc::nfds_t, a2: i32) -> i32 {
    libc::poll(a0, a1 as ::libc::nfds_t, a2)
}

fn t1() -> ::libc::pollfd {
    unsafe { std::mem::zeroed() }
}

fn t2() -> ::libc::nfds_t {
    0
}
