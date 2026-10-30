// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *mut *mut libc::ifaddrs) -> i32 {
    libc::getifaddrs(a0)
}

unsafe fn f2(a0: *mut libc::ifaddrs) {
    libc::freeifaddrs(a0)
}

fn t1() -> libc::ifaddrs {
    unsafe { std::mem::zeroed() }
}
