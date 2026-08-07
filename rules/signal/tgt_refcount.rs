// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f3(a0: i32, a1: FnPtr<fn(i32)>) -> FnPtr<fn(i32)> {
    libcc2rs::signal_refcount(a0, a1.clone())
}

fn f2(a0: i32) -> i32 {
    match nix::sys::signal::Signal::try_from(a0) {
        Ok(__sig) => match nix::sys::signal::raise(__sig) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        },
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f4(a0: i32, a1: i32) -> i32 {
    panic!("kill: signalling other processes is not supported in the refcount model")
}
