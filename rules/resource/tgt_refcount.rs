// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Rusage {
    Default::default()
}

fn f1(a0: i32, a1: Ptr<libcc2rs::Rusage>) -> i32 {
    let __who = match a0 {
        ::libc::RUSAGE_SELF => nix::sys::resource::UsageWho::RUSAGE_SELF,
        ::libc::RUSAGE_CHILDREN => nix::sys::resource::UsageWho::RUSAGE_CHILDREN,
        _ => panic!("getrusage: unsupported who value"),
    };
    match nix::sys::resource::getrusage(__who) {
        Ok(__u) => {
            a1.with_mut(|__r| *__r = Rusage::from_libc(__u.as_ref()));
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
