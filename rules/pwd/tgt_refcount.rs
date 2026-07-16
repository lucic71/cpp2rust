// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Passwd {
    Default::default()
}

fn f1(a0: u32) -> Ptr<Passwd> {
    match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(a0)) {
        Ok(Some(__u)) => Ptr::alloc(Passwd::from_user(&__u)),
        _ => Ptr::null(),
    }
}
