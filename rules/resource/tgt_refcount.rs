// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t2() -> i32 {
    0
}

fn t3() -> i32 {
    0
}

fn t4() -> i32 {
    0
}

fn t5() -> libcc2rs::Rlimit {
    Default::default()
}

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

fn f4(a0: i32, a1: Ptr<libcc2rs::Rlimit>) -> i32 {
    let __res = match a0 as u32 {
        ::libc::RLIMIT_AS => Some(nix::sys::resource::Resource::RLIMIT_AS),
        ::libc::RLIMIT_CORE => Some(nix::sys::resource::Resource::RLIMIT_CORE),
        ::libc::RLIMIT_CPU => Some(nix::sys::resource::Resource::RLIMIT_CPU),
        ::libc::RLIMIT_DATA => Some(nix::sys::resource::Resource::RLIMIT_DATA),
        ::libc::RLIMIT_FSIZE => Some(nix::sys::resource::Resource::RLIMIT_FSIZE),
        ::libc::RLIMIT_NOFILE => Some(nix::sys::resource::Resource::RLIMIT_NOFILE),
        ::libc::RLIMIT_STACK => Some(nix::sys::resource::Resource::RLIMIT_STACK),
        _ => None,
    };
    match __res {
        None => {
            libcc2rs::cpp2rust_errno().write(::libc::EINVAL);
            -1
        }
        Some(__r) => match nix::sys::resource::getrlimit(__r) {
            Ok((__soft, __hard)) => {
                a1.clone().with_mut(|__v| {
                    __v.rlim_cur = __soft;
                    __v.rlim_max = __hard;
                });
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        },
    }
}

fn f5(a0: i32, a1: Ptr<libcc2rs::Rlimit>) -> i32 {
    let __res = match a0 as u32 {
        ::libc::RLIMIT_AS => Some(nix::sys::resource::Resource::RLIMIT_AS),
        ::libc::RLIMIT_CORE => Some(nix::sys::resource::Resource::RLIMIT_CORE),
        ::libc::RLIMIT_CPU => Some(nix::sys::resource::Resource::RLIMIT_CPU),
        ::libc::RLIMIT_DATA => Some(nix::sys::resource::Resource::RLIMIT_DATA),
        ::libc::RLIMIT_FSIZE => Some(nix::sys::resource::Resource::RLIMIT_FSIZE),
        ::libc::RLIMIT_NOFILE => Some(nix::sys::resource::Resource::RLIMIT_NOFILE),
        ::libc::RLIMIT_STACK => Some(nix::sys::resource::Resource::RLIMIT_STACK),
        _ => None,
    };
    let __lim = a1.clone().read();
    match __res {
        None => {
            libcc2rs::cpp2rust_errno().write(::libc::EINVAL);
            -1
        }
        Some(__r) => match nix::sys::resource::setrlimit(__r, __lim.rlim_cur, __lim.rlim_max) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        },
    }
}
