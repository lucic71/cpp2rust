extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_send_recv_0() {
    let sv: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        ((({
            use nix::sys::socket::{AddressFamily, SockFlag, SockProtocol, SockType};
            let __out = (sv.as_pointer() as Ptr<i32>).clone();
            let __flag_bits = ::libc::SOCK_CLOEXEC | ::libc::SOCK_NONBLOCK;
            match (
                AddressFamily::from_i32(1),
                SockType::try_from(libc::SOCK_STREAM & !__flag_bits),
            ) {
                (Some(__f), Ok(__t)) => {
                    let __flags = SockFlag::from_bits_truncate(libc::SOCK_STREAM & __flag_bits);
                    let __proto: Option<SockProtocol> = None;
                    match nix::sys::socket::socketpair(__f, __t, __proto, __flags) {
                        Ok((__a, __b)) => {
                            __out.write(::std::os::fd::IntoRawFd::into_raw_fd(__a));
                            __out
                                .offset(1)
                                .write(::std::os::fd::IntoRawFd::into_raw_fd(__b));
                            0
                        }
                        Err(__e) => {
                            libcc2rs::cpp2rust_errno().write(__e as i32);
                            -1
                        }
                    }
                }
                _ => {
                    libcc2rs::cpp2rust_errno().write(::libc::EINVAL);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    let msg: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"ping")));
    assert!(
        ((({
            let __buf = ((*msg.borrow()).clone() as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice(4_usize, |__s| {
                nix::sys::socket::send(
                    (*sv.borrow())[(0) as usize],
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 4_isize) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    assert!(
        ((({
            let __buf = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice_mut(8_usize, |__s| {
                nix::sys::socket::recv(
                    (*sv.borrow())[(1) as usize],
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 4_isize) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"ping").to_any(), 4_usize)
            == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __buf = Ptr::from_string_literal(b"pong!")
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice(5_usize, |__s| {
                nix::sys::socket::send(
                    (*sv.borrow())[(1) as usize],
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 5_isize) as i32)
            != 0)
    );
    assert!(
        ((({
            let __buf = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice_mut(8_usize, |__s| {
                nix::sys::socket::recv(
                    (*sv.borrow())[(0) as usize],
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 5_isize) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"pong!").to_any(), 5_usize)
            == 0) as i32)
            != 0)
    );
}
pub fn test_send_recv_scalar_1() {
    let sv: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        ((({
            use nix::sys::socket::{AddressFamily, SockFlag, SockProtocol, SockType};
            let __out = (sv.as_pointer() as Ptr<i32>).clone();
            let __flag_bits = ::libc::SOCK_CLOEXEC | ::libc::SOCK_NONBLOCK;
            match (
                AddressFamily::from_i32(1),
                SockType::try_from(libc::SOCK_STREAM & !__flag_bits),
            ) {
                (Some(__f), Ok(__t)) => {
                    let __flags = SockFlag::from_bits_truncate(libc::SOCK_STREAM & __flag_bits);
                    let __proto: Option<SockProtocol> = None;
                    match nix::sys::socket::socketpair(__f, __t, __proto, __flags) {
                        Ok((__a, __b)) => {
                            __out.write(::std::os::fd::IntoRawFd::into_raw_fd(__a));
                            __out
                                .offset(1)
                                .write(::std::os::fd::IntoRawFd::into_raw_fd(__b));
                            0
                        }
                        Err(__e) => {
                            libcc2rs::cpp2rust_errno().write(__e as i32);
                            -1
                        }
                    }
                }
                _ => {
                    libcc2rs::cpp2rust_errno().write(::libc::EINVAL);
                    -1
                }
            }
        } == 0) as i32)
            != 0)
    );
    let value: Value<i32> = Rc::new(RefCell::new(42));
    assert!(
        (((({
            let __buf = ((value.as_pointer()) as Ptr<i32>)
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice(::std::mem::size_of::<i32>(), |__s| {
                nix::sys::socket::send(
                    (*sv.borrow())[(0) as usize],
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } as usize)
            == ::std::mem::size_of::<i32>()) as i32)
            != 0)
    );
    let received: Value<i32> = Rc::new(RefCell::new(0));
    assert!(
        (((({
            let __buf = ((received.as_pointer()) as Ptr<i32>)
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice_mut(::std::mem::size_of::<i32>(), |__s| {
                nix::sys::socket::recv(
                    (*sv.borrow())[(1) as usize],
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } as usize)
            == ::std::mem::size_of::<i32>()) as i32)
            != 0)
    );
    assert!(((((*received.borrow()) == 42) as i32) != 0));
}
pub fn test_send_bad_fd_2() {
    libcc2rs::cpp2rust_errno().write(0);
    assert!(
        ((({
            let __buf = Ptr::from_string_literal(b"x")
                .to_any()
                .reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    -1_i32,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!(((((libcc2rs::cpp2rust_errno().read()) == 9) as i32) != 0));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_send_recv_0() });
    ({ test_send_recv_scalar_1() });
    ({ test_send_bad_fd_2() });
    return 0;
}
