// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f5(a0: i32, a1: Ptr<u8>, a2: AnyPtr) -> i32 {
    if a0 == libc::AF_INET {
        match a1.to_rust_string().parse::<std::net::Ipv4Addr>() {
            Ok(__ip) => {
                let __octets = __ip.octets();
                for __i in 0..4 {
                    a2.reinterpret_cast::<u8>().offset(__i).write(__octets[__i]);
                }
                1
            }
            Err(_) => 0,
        }
    } else if a0 == libc::AF_INET6 {
        match a1.to_rust_string().parse::<std::net::Ipv6Addr>() {
            Ok(__ip) => {
                let __octets = __ip.octets();
                for __i in 0..16 {
                    a2.reinterpret_cast::<u8>().offset(__i).write(__octets[__i]);
                }
                1
            }
            Err(_) => 0,
        }
    } else {
        -1
    }
}

fn f6(a0: i32, a1: AnyPtr, a2: Ptr<u8>, a3: u32) -> Ptr<u8> {
    let __text = if a0 == libc::AF_INET {
        let mut __b = [0u8; 4];
        for __i in 0..4 {
            __b[__i] = a1.reinterpret_cast::<u8>().offset(__i).read();
        }
        Some(std::net::Ipv4Addr::from(__b).to_string())
    } else if a0 == libc::AF_INET6 {
        let mut __b = [0u8; 16];
        for __i in 0..16 {
            __b[__i] = a1.reinterpret_cast::<u8>().offset(__i).read();
        }
        Some(std::net::Ipv6Addr::from(__b).to_string())
    } else {
        None
    };
    match __text {
        Some(__s) if (__s.len() as u32) < a3 => {
            for __i in 0..__s.len() {
                a2.offset(__i).write(__s.as_bytes()[__i]);
            }
            a2.offset(__s.len()).write(0);
            a2.clone()
        }
        _ => Ptr::null(),
    }
}
