// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Ifaddrs {
    Default::default()
}

fn f1(a0: Ptr<Ptr<Ifaddrs>>) -> i32 {
    let __out = a0.clone();
    match nix::ifaddrs::getifaddrs() {
        Ok(__ifas) => {
            let __list: Vec<nix::ifaddrs::InterfaceAddress> = __ifas.collect();
            let mut __next = Ptr::<Ifaddrs>::null();
            for __ifa in __list.iter().rev() {
                let mut __node = Ifaddrs::from_interface_address(__ifa);
                __node.ifa_next = __next.clone();
                __next = Ptr::alloc(__node);
            }
            __out.write(__next);
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f2(a0: Ptr<Ifaddrs>) {
    let mut __cur = a0.clone();
    while !__cur.is_null() {
        let __next = __cur.with(|__i| {
            let __name = &__i.ifa_name;
            if !__name.is_null() {
                __name.delete_array();
            }
            let __addr = &__i.ifa_addr;
            if !__addr.is_null() {
                __addr.delete();
            }
            let __mask = &__i.ifa_netmask;
            if !__mask.is_null() {
                __mask.delete();
            }
            __i.ifa_next.clone()
        });
        __cur.delete();
        __cur = __next;
    }
}
