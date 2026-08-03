// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *const libc::c_char) -> u32 {
    libc::if_nametoindex(a0)
}

unsafe fn f2() -> i32 {
    ::libc::IFF_UP
}

unsafe fn f3() -> i32 {
    ::libc::IFF_LOOPBACK
}
