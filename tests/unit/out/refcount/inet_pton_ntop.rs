extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..16).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        (((if libc::AF_INET == libc::AF_INET {
            match Ptr::from_string_literal(b"1.2.3.4")
                .to_rust_string()
                .parse::<std::net::Ipv4Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..4 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else if libc::AF_INET == libc::AF_INET6 {
            match Ptr::from_string_literal(b"1.2.3.4")
                .to_rust_string()
                .parse::<std::net::Ipv6Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..16 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else {
            -1
        } == 1) as i32)
            != 0)
    );
    assert!(
        ((((((((((((((*buf.borrow())[(0) as usize] as i32) == 1) as i32) != 0)
            && (((((*buf.borrow())[(1) as usize] as i32) == 2) as i32) != 0))
            as i32)
            != 0)
            && (((((*buf.borrow())[(2) as usize] as i32) == 3) as i32) != 0)) as i32)
            != 0)
            && (((((*buf.borrow())[(3) as usize] as i32) == 4) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        (((if libc::AF_INET == libc::AF_INET {
            match Ptr::from_string_literal(b"999.1.1.1")
                .to_rust_string()
                .parse::<std::net::Ipv4Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..4 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else if libc::AF_INET == libc::AF_INET6 {
            match Ptr::from_string_literal(b"999.1.1.1")
                .to_rust_string()
                .parse::<std::net::Ipv6Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..16 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else {
            -1
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((if libc::AF_INET == libc::AF_INET {
            match Ptr::from_string_literal(b"not an ip")
                .to_rust_string()
                .parse::<std::net::Ipv4Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..4 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else if libc::AF_INET == libc::AF_INET6 {
            match Ptr::from_string_literal(b"not an ip")
                .to_rust_string()
                .parse::<std::net::Ipv6Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..16 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else {
            -1
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((if libc::AF_INET6 == libc::AF_INET {
            match Ptr::from_string_literal(b"::1")
                .to_rust_string()
                .parse::<std::net::Ipv4Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..4 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else if libc::AF_INET6 == libc::AF_INET6 {
            match Ptr::from_string_literal(b"::1")
                .to_rust_string()
                .parse::<std::net::Ipv6Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..16 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else {
            -1
        } == 1) as i32)
            != 0)
    );
    assert!(
        ((((((((*buf.borrow())[(0) as usize] as i32) == 0) as i32) != 0)
            && (((((*buf.borrow())[(15) as usize] as i32) == 1) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        (((if libc::AF_INET6 == libc::AF_INET {
            match Ptr::from_string_literal(b"2001:db8::5")
                .to_rust_string()
                .parse::<std::net::Ipv4Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..4 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else if libc::AF_INET6 == libc::AF_INET6 {
            match Ptr::from_string_literal(b"2001:db8::5")
                .to_rust_string()
                .parse::<std::net::Ipv6Addr>()
            {
                Ok(__ip) => {
                    let __octets = __ip.octets();
                    for __i in 0..16 {
                        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .write(__octets[__i]);
                    }
                    1
                }
                Err(_) => 0,
            }
        } else {
            -1
        } == 1) as i32)
            != 0)
    );
    assert!(
        (((((((((((*buf.borrow())[(0) as usize] as i32) == 32) as i32) != 0)
            && (((((*buf.borrow())[(1) as usize] as i32) == 1) as i32) != 0)) as i32)
            != 0)
            && (((((*buf.borrow())[(15) as usize] as i32) == 5) as i32) != 0)) as i32)
            != 0)
    );
    let text: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let four: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([10_u8, 0_u8, 0_u8, 1_u8])));
    assert!(
        ((({
            let mut __it1 = {
                let __text = if libc::AF_INET == libc::AF_INET {
                    let mut __b = [0u8; 4];
                    for __i in 0..4 {
                        __b[__i] = ((four.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .read();
                    }
                    Some(std::net::Ipv4Addr::from(__b).to_string())
                } else if libc::AF_INET == libc::AF_INET6 {
                    let mut __b = [0u8; 16];
                    for __i in 0..16 {
                        __b[__i] = ((four.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .read();
                    }
                    Some(std::net::Ipv6Addr::from(__b).to_string())
                } else {
                    None
                };
                match __text {
                    Some(__s)
                        if (__s.len() as u32) < (::std::mem::size_of::<[u8; 64]>() as u32) =>
                    {
                        for __i in 0..__s.len() {
                            (text.as_pointer() as Ptr<u8>)
                                .offset(__i)
                                .write(__s.as_bytes()[__i]);
                        }
                        (text.as_pointer() as Ptr<u8>).offset(__s.len()).write(0);
                        (text.as_pointer() as Ptr<u8>).clone()
                    }
                    _ => Ptr::null(),
                }
            }
            .to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"10.0.0.1").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    let sixteen: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    (*sixteen.borrow_mut())[(15) as usize] = 1_u8;
    assert!(
        ((({
            let mut __it1 = {
                let __text = if libc::AF_INET6 == libc::AF_INET {
                    let mut __b = [0u8; 4];
                    for __i in 0..4 {
                        __b[__i] = ((sixteen.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .read();
                    }
                    Some(std::net::Ipv4Addr::from(__b).to_string())
                } else if libc::AF_INET6 == libc::AF_INET6 {
                    let mut __b = [0u8; 16];
                    for __i in 0..16 {
                        __b[__i] = ((sixteen.as_pointer() as Ptr<u8>) as Ptr<u8>)
                            .to_any()
                            .reinterpret_cast::<u8>()
                            .offset(__i)
                            .read();
                    }
                    Some(std::net::Ipv6Addr::from(__b).to_string())
                } else {
                    None
                };
                match __text {
                    Some(__s)
                        if (__s.len() as u32) < (::std::mem::size_of::<[u8; 64]>() as u32) =>
                    {
                        for __i in 0..__s.len() {
                            (text.as_pointer() as Ptr<u8>)
                                .offset(__i)
                                .write(__s.as_bytes()[__i]);
                        }
                        (text.as_pointer() as Ptr<u8>).offset(__s.len()).write(0);
                        (text.as_pointer() as Ptr<u8>).clone()
                    }
                    _ => Ptr::null(),
                }
            }
            .to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"::1").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((({
            let __text = if libc::AF_INET == libc::AF_INET {
                let mut __b = [0u8; 4];
                for __i in 0..4 {
                    __b[__i] = ((four.as_pointer() as Ptr<u8>) as Ptr<u8>)
                        .to_any()
                        .reinterpret_cast::<u8>()
                        .offset(__i)
                        .read();
                }
                Some(std::net::Ipv4Addr::from(__b).to_string())
            } else if libc::AF_INET == libc::AF_INET6 {
                let mut __b = [0u8; 16];
                for __i in 0..16 {
                    __b[__i] = ((four.as_pointer() as Ptr<u8>) as Ptr<u8>)
                        .to_any()
                        .reinterpret_cast::<u8>()
                        .offset(__i)
                        .read();
                }
                Some(std::net::Ipv6Addr::from(__b).to_string())
            } else {
                None
            };
            match __text {
                Some(__s) if (__s.len() as u32) < 4_u32 => {
                    for __i in 0..__s.len() {
                        (text.as_pointer() as Ptr<u8>)
                            .offset(__i)
                            .write(__s.as_bytes()[__i]);
                    }
                    (text.as_pointer() as Ptr<u8>).offset(__s.len()).write(0);
                    (text.as_pointer() as Ptr<u8>).clone()
                }
                _ => Ptr::null(),
            }
        })
        .is_null()) as i32)
            != 0)
    );
    return 0;
}
