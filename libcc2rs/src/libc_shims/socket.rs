// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use super::{In6Addr, InAddr};
use crate::{ByteRepr, Ptr};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sockaddr {
    pub sa_family: u16,
    pub sa_data: Box<[u8]>,
}

#[derive(Clone)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: Box<[u8]>,
}

#[derive(Default, Clone)]
pub struct SockaddrIn6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: In6Addr,
    pub sin6_scope_id: u32,
}

#[derive(Clone)]
pub struct SockaddrUn {
    pub sun_family: u16,
    pub sun_path: Box<[u8]>,
}

#[derive(Clone)]
pub struct SockaddrStorage {
    pub ss_family: u16,
    pub __pad: Box<[u8]>,
}

impl SockaddrIn {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(l: &::libc::sockaddr_in) -> Self {
        Self {
            sin_family: l.sin_family as u16,
            sin_port: l.sin_port,
            sin_addr: InAddr {
                s_addr: l.sin_addr.s_addr,
            },
            sin_zero: l
                .sin_zero
                .iter()
                .map(|&b| b as u8)
                .collect::<Vec<u8>>()
                .into_boxed_slice(),
        }
    }

    pub fn from_ipv4(addr: &::std::net::Ipv4Addr, port: u16) -> Self {
        let mut s = Self::default();
        s.sin_family = ::libc::AF_INET as u16;
        s.sin_port = port.to_be();
        s.sin_addr.s_addr = u32::from(*addr).to_be();
        s
    }

    #[cfg(target_os = "linux")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in {
        let mut sin_zero = [0u8; 8];
        sin_zero.copy_from_slice(&self.sin_zero);
        ::libc::sockaddr_in {
            sin_family: self.sin_family,
            sin_port: self.sin_port,
            sin_addr: ::libc::in_addr {
                s_addr: self.sin_addr.s_addr,
            },
            sin_zero,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in {
        let mut sin_zero = [0i8; 8];
        for (dst, src) in sin_zero.iter_mut().zip(self.sin_zero.iter()) {
            *dst = *src as i8;
        }
        ::libc::sockaddr_in {
            sin_len: ::std::mem::size_of::<::libc::sockaddr_in>() as u8,
            sin_family: self.sin_family as u8,
            sin_port: self.sin_port,
            sin_addr: ::libc::in_addr {
                s_addr: self.sin_addr.s_addr,
            },
            sin_zero,
        }
    }
}

impl SockaddrIn6 {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(l: &::libc::sockaddr_in6) -> Self {
        Self {
            sin6_family: l.sin6_family as u16,
            sin6_port: l.sin6_port,
            sin6_flowinfo: l.sin6_flowinfo,
            sin6_addr: In6Addr {
                s6_addr: Rc::new(RefCell::new(
                    l.sin6_addr.s6_addr.to_vec().into_boxed_slice(),
                )),
            },
            sin6_scope_id: l.sin6_scope_id,
        }
    }

    pub fn from_ipv6(addr: &::std::net::Ipv6Addr, port: u16) -> Self {
        let mut s = Self::default();
        s.sin6_family = ::libc::AF_INET6 as u16;
        s.sin6_port = port.to_be();
        s.sin6_addr
            .s6_addr
            .borrow_mut()
            .copy_from_slice(&addr.octets());
        s
    }

    #[cfg(target_os = "linux")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in6 {
        let mut s6_addr = [0u8; 16];
        s6_addr.copy_from_slice(&self.sin6_addr.s6_addr.borrow());
        ::libc::sockaddr_in6 {
            sin6_family: self.sin6_family,
            sin6_port: self.sin6_port,
            sin6_flowinfo: self.sin6_flowinfo,
            sin6_addr: ::libc::in6_addr { s6_addr },
            sin6_scope_id: self.sin6_scope_id,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in6 {
        let mut s6_addr = [0u8; 16];
        s6_addr.copy_from_slice(&self.sin6_addr.s6_addr.borrow());
        ::libc::sockaddr_in6 {
            sin6_len: ::std::mem::size_of::<::libc::sockaddr_in6>() as u8,
            sin6_family: self.sin6_family as u8,
            sin6_port: self.sin6_port,
            sin6_flowinfo: self.sin6_flowinfo,
            sin6_addr: ::libc::in6_addr { s6_addr },
            sin6_scope_id: self.sin6_scope_id,
        }
    }
}

impl Default for Sockaddr {
    fn default() -> Self {
        Self {
            sa_family: 0,
            sa_data: vec![0u8; 14].into_boxed_slice(),
        }
    }
}

impl Default for SockaddrIn {
    fn default() -> Self {
        Self {
            sin_family: 0,
            sin_port: 0,
            sin_addr: InAddr::default(),
            sin_zero: vec![0u8; 8].into_boxed_slice(),
        }
    }
}

impl Default for SockaddrUn {
    fn default() -> Self {
        Self {
            sun_family: 0,
            sun_path: vec![0u8; 108].into_boxed_slice(),
        }
    }
}

impl Default for SockaddrStorage {
    fn default() -> Self {
        Self {
            ss_family: 0,
            __pad: vec![0u8; 126].into_boxed_slice(),
        }
    }
}

impl ByteRepr for Sockaddr {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.sa_family.to_bytes(&mut buf[0..2]);
        buf[2..16].copy_from_slice(&self.sa_data);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sa_family: <u16>::from_bytes(&buf[0..2]),
            sa_data: buf[2..16].to_vec().into_boxed_slice(),
        }
    }
}

impl ByteRepr for SockaddrIn {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.sin_family.to_bytes(&mut buf[0..2]);
        self.sin_port.to_bytes(&mut buf[2..4]);
        self.sin_addr.to_bytes(&mut buf[4..8]);
        buf[8..16].copy_from_slice(&self.sin_zero);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sin_family: <u16>::from_bytes(&buf[0..2]),
            sin_port: <u16>::from_bytes(&buf[2..4]),
            sin_addr: <InAddr>::from_bytes(&buf[4..8]),
            sin_zero: buf[8..16].to_vec().into_boxed_slice(),
        }
    }
}

impl ByteRepr for SockaddrIn6 {
    fn byte_size() -> usize {
        28
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.sin6_family.to_bytes(&mut buf[0..2]);
        self.sin6_port.to_bytes(&mut buf[2..4]);
        self.sin6_flowinfo.to_bytes(&mut buf[4..8]);
        self.sin6_addr.to_bytes(&mut buf[8..24]);
        self.sin6_scope_id.to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sin6_family: <u16>::from_bytes(&buf[0..2]),
            sin6_port: <u16>::from_bytes(&buf[2..4]),
            sin6_flowinfo: <u32>::from_bytes(&buf[4..8]),
            sin6_addr: <In6Addr>::from_bytes(&buf[8..24]),
            sin6_scope_id: <u32>::from_bytes(&buf[24..28]),
        }
    }
}

impl ByteRepr for SockaddrUn {
    fn byte_size() -> usize {
        110
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.sun_family.to_bytes(&mut buf[0..2]);
        buf[2..110].copy_from_slice(&self.sun_path);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sun_family: <u16>::from_bytes(&buf[0..2]),
            sun_path: buf[2..110].to_vec().into_boxed_slice(),
        }
    }
}

impl ByteRepr for SockaddrStorage {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.ss_family.to_bytes(&mut buf[0..2]);
        buf[2..128].copy_from_slice(&self.__pad);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ss_family: <u16>::from_bytes(&buf[0..2]),
            __pad: buf[2..128].to_vec().into_boxed_slice(),
        }
    }
}

impl ByteRepr for ::libc::sockaddr {}
impl ByteRepr for ::libc::sockaddr_in {}
impl ByteRepr for ::libc::sockaddr_in6 {}
impl ByteRepr for ::libc::sockaddr_un {}
impl ByteRepr for ::libc::sockaddr_storage {}

impl Sockaddr {
    pub fn decode(
        addr: &Ptr<Sockaddr>,
        _len: u32,
    ) -> Option<Box<dyn nix::sys::socket::SockaddrLike>> {
        let family = addr.reinterpret_cast::<u16>().read();
        if family == ::libc::AF_INET as u16 {
            let m = addr.reinterpret_cast::<SockaddrIn>().read();
            Some(Box::new(nix::sys::socket::SockaddrIn::from(m.to_libc())))
        } else if family == ::libc::AF_INET6 as u16 {
            let m = addr.reinterpret_cast::<SockaddrIn6>().read();
            Some(Box::new(nix::sys::socket::SockaddrIn6::from(m.to_libc())))
        } else if family == ::libc::AF_UNIX as u16 {
            let m = addr.reinterpret_cast::<SockaddrUn>().read();
            let path = &m.sun_path;
            let end = path.iter().position(|&c| c == 0).unwrap_or(path.len());
            nix::sys::socket::UnixAddr::new(&path[..end])
                .ok()
                .map(|u| Box::new(u) as Box<dyn nix::sys::socket::SockaddrLike>)
        } else {
            None
        }
    }

    pub fn encode(ss: &nix::sys::socket::SockaddrStorage, out: &Ptr<Sockaddr>, out_len: &Ptr<u32>) {
        use nix::sys::socket::{AddressFamily, SockaddrLike};
        match ss.family() {
            Some(AddressFamily::Inet) => {
                let l = ::libc::sockaddr_in::from(*ss.as_sockaddr_in().unwrap());
                out.reinterpret_cast::<SockaddrIn>()
                    .write(SockaddrIn::from_libc(&l));
            }
            Some(AddressFamily::Inet6) => {
                let l = ::libc::sockaddr_in6::from(*ss.as_sockaddr_in6().unwrap());
                out.reinterpret_cast::<SockaddrIn6>()
                    .write(SockaddrIn6::from_libc(&l));
            }
            _ => {}
        }
        out_len.write(ss.len());
    }
}

pub fn setsockopt_refcount(fd: i32, level: i32, optname: i32, optval: crate::AnyPtr) -> i32 {
    let res = match (level, optname) {
        (::libc::IPPROTO_TCP, ::libc::TCP_NODELAY) => {
            let v = optval.reinterpret_cast::<i32>().read() != 0;
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::TcpNoDelay, &v)
            })
        }
        (::libc::SOL_SOCKET, ::libc::SO_KEEPALIVE) => {
            let v = optval.reinterpret_cast::<i32>().read() != 0;
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::KeepAlive, &v)
            })
        }
        (::libc::IPPROTO_TCP, ::libc::TCP_KEEPINTVL) => {
            let v = optval.reinterpret_cast::<u32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(
                    &borrowed,
                    nix::sys::socket::sockopt::TcpKeepInterval,
                    &v,
                )
            })
        }
        (::libc::IPPROTO_TCP, ::libc::TCP_KEEPCNT) => {
            let v = optval.reinterpret_cast::<u32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::TcpKeepCount, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_IP, ::libc::IP_TOS) => {
            let v = optval.reinterpret_cast::<i32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::Ipv4Tos, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_IPV6, ::libc::IPV6_TCLASS) => {
            let v = optval.reinterpret_cast::<i32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::Ipv6TClass, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_TCP, ::libc::TCP_KEEPIDLE) => {
            let v = optval.reinterpret_cast::<u32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::TcpKeepIdle, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::SOL_SOCKET, ::libc::SO_BINDTODEVICE) => {
            let v = ::std::ffi::OsString::from(optval.reinterpret_cast::<u8>().to_rust_string());
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::BindToDevice, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_IP, ::libc::IP_BIND_ADDRESS_NO_PORT) => {
            let v = optval.reinterpret_cast::<i32>().read() != 0;
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(
                    &borrowed,
                    nix::sys::socket::sockopt::IpBindAddressNoPort,
                    &v,
                )
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_TCP, ::libc::TCP_FASTOPEN_CONNECT) => {
            let v = optval.reinterpret_cast::<i32>().read() != 0;
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(
                    &borrowed,
                    nix::sys::socket::sockopt::TcpFastOpenConnect,
                    &v,
                )
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::SOL_SOCKET, ::libc::SO_PRIORITY) => {
            let v = optval.reinterpret_cast::<i32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::Priority, &v)
            })
        }
        (l, o) => panic!(
            "setsockopt: unsupported option (level={}, optname={})",
            l, o
        ),
    };
    match res {
        Ok(()) => 0,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}
