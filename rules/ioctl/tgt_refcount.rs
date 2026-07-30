// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: i32, a1: u64, va: &[VaArg]) -> i32 {
    match a1 as ::libc::c_ulong {
        ::libc::TIOCGWINSZ => match FdRegistry::with_fd(a0, Winsize::from_fd) {
            Some(__ws) => {
                Ptr::<Winsize>::get(&va[0]).with_mut(|__dst| *__dst = __ws);
                0
            }
            None => {
                libcc2rs::cpp2rust_errno().write(::libc::ENOTTY);
                -1
            }
        },
        __request => panic!("ioctl: unsupported request {}", __request),
    }
}
