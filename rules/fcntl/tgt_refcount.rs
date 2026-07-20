// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: i32, a1: i32, va: &[VaArg]) -> i32 {
    panic!(
        "fcntl is not supported in the refcount model (fd={}, cmd={}, varargs={})",
        a0,
        a1,
        va.len()
    )
}

fn f2(a0: Ptr<u8>, a1: i32, va: &[VaArg]) -> i32 {
    let __mode = match va.first() {
        Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
        None => nix::sys::stat::Mode::empty(),
    };
    match nix::fcntl::open(
        a0.to_rust_string().as_str(),
        nix::fcntl::OFlag::from_bits_retain(a1),
        __mode,
    ) {
        Ok(__ofd) => FdRegistry::register(__ofd),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
