// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use super::{in6_addr, in_addr};
use crate::{ByteRepr, Ptr, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct sockaddr {
    pub sa_family: Value<u16>,
    pub sa_data: Value<Box<[u8]>>,
}

pub struct sockaddr_in {
    pub sin_family: Value<u16>,
    pub sin_port: Value<u16>,
    pub sin_addr: Value<in_addr>,
    pub sin_zero: Value<Box<[u8]>>,
}

pub struct sockaddr_in6 {
    pub sin6_family: Value<u16>,
    pub sin6_port: Value<u16>,
    pub sin6_flowinfo: Value<u32>,
    pub sin6_addr: Value<in6_addr>,
    pub sin6_scope_id: Value<u32>,
}

pub struct sockaddr_un {
    pub sun_family: Value<u16>,
    pub sun_path: Value<Box<[u8]>>,
}

pub struct sockaddr_storage {
    pub ss_family: Value<u16>,
    pub __pad: Value<Box<[u8]>>,
}

impl Default for sockaddr {
    fn default() -> Self {
        Self {
            sa_family: Rc::new(RefCell::new(0)),
            sa_data: Rc::new(RefCell::new(vec![0u8; 14].into_boxed_slice())),
        }
    }
}

impl Default for sockaddr_in {
    fn default() -> Self {
        Self {
            sin_family: Rc::new(RefCell::new(0)),
            sin_port: Rc::new(RefCell::new(0)),
            sin_addr: Rc::new(RefCell::new(in_addr::default())),
            sin_zero: Rc::new(RefCell::new(vec![0u8; 8].into_boxed_slice())),
        }
    }
}

impl Default for sockaddr_in6 {
    fn default() -> Self {
        Self {
            sin6_family: Rc::new(RefCell::new(0)),
            sin6_port: Rc::new(RefCell::new(0)),
            sin6_flowinfo: Rc::new(RefCell::new(0)),
            sin6_addr: Rc::new(RefCell::new(in6_addr::default())),
            sin6_scope_id: Rc::new(RefCell::new(0)),
        }
    }
}

impl Default for sockaddr_un {
    fn default() -> Self {
        Self {
            sun_family: Rc::new(RefCell::new(0)),
            sun_path: Rc::new(RefCell::new(vec![0u8; 108].into_boxed_slice())),
        }
    }
}

impl Default for sockaddr_storage {
    fn default() -> Self {
        Self {
            ss_family: Rc::new(RefCell::new(0)),
            __pad: Rc::new(RefCell::new(vec![0u8; 126].into_boxed_slice())),
        }
    }
}

impl Clone for sockaddr {
    fn clone(&self) -> Self {
        Self {
            sa_family: Rc::new(RefCell::new(*self.sa_family.borrow())),
            sa_data: Rc::new(RefCell::new(self.sa_data.borrow().clone())),
        }
    }
}

impl Clone for sockaddr_in {
    fn clone(&self) -> Self {
        Self {
            sin_family: Rc::new(RefCell::new(*self.sin_family.borrow())),
            sin_port: Rc::new(RefCell::new(*self.sin_port.borrow())),
            sin_addr: Rc::new(RefCell::new(self.sin_addr.borrow().clone())),
            sin_zero: Rc::new(RefCell::new(self.sin_zero.borrow().clone())),
        }
    }
}

impl Clone for sockaddr_in6 {
    fn clone(&self) -> Self {
        Self {
            sin6_family: Rc::new(RefCell::new(*self.sin6_family.borrow())),
            sin6_port: Rc::new(RefCell::new(*self.sin6_port.borrow())),
            sin6_flowinfo: Rc::new(RefCell::new(*self.sin6_flowinfo.borrow())),
            sin6_addr: Rc::new(RefCell::new(self.sin6_addr.borrow().clone())),
            sin6_scope_id: Rc::new(RefCell::new(*self.sin6_scope_id.borrow())),
        }
    }
}

impl Clone for sockaddr_un {
    fn clone(&self) -> Self {
        Self {
            sun_family: Rc::new(RefCell::new(*self.sun_family.borrow())),
            sun_path: Rc::new(RefCell::new(self.sun_path.borrow().clone())),
        }
    }
}

impl Clone for sockaddr_storage {
    fn clone(&self) -> Self {
        Self {
            ss_family: Rc::new(RefCell::new(*self.ss_family.borrow())),
            __pad: Rc::new(RefCell::new(self.__pad.borrow().clone())),
        }
    }
}

impl ByteRepr for sockaddr {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.sa_family.borrow()).to_bytes(&mut buf[0..2]);
        buf[2..16].copy_from_slice(&self.sa_data.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sa_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            sa_data: Rc::new(RefCell::new(buf[2..16].to_vec().into_boxed_slice())),
        }
    }
}

impl ByteRepr for sockaddr_in {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.sin_family.borrow()).to_bytes(&mut buf[0..2]);
        (*self.sin_port.borrow()).to_bytes(&mut buf[2..4]);
        (*self.sin_addr.borrow()).to_bytes(&mut buf[4..8]);
        buf[8..16].copy_from_slice(&self.sin_zero.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sin_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            sin_port: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
            sin_addr: Rc::new(RefCell::new(<in_addr>::from_bytes(&buf[4..8]))),
            sin_zero: Rc::new(RefCell::new(buf[8..16].to_vec().into_boxed_slice())),
        }
    }
}

impl ByteRepr for sockaddr_in6 {
    fn byte_size() -> usize {
        28
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.sin6_family.borrow()).to_bytes(&mut buf[0..2]);
        (*self.sin6_port.borrow()).to_bytes(&mut buf[2..4]);
        (*self.sin6_flowinfo.borrow()).to_bytes(&mut buf[4..8]);
        (*self.sin6_addr.borrow()).to_bytes(&mut buf[8..24]);
        (*self.sin6_scope_id.borrow()).to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sin6_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            sin6_port: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
            sin6_flowinfo: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
            sin6_addr: Rc::new(RefCell::new(<in6_addr>::from_bytes(&buf[8..24]))),
            sin6_scope_id: Rc::new(RefCell::new(<u32>::from_bytes(&buf[24..28]))),
        }
    }
}

impl ByteRepr for sockaddr_un {
    fn byte_size() -> usize {
        110
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.sun_family.borrow()).to_bytes(&mut buf[0..2]);
        buf[2..110].copy_from_slice(&self.sun_path.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sun_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            sun_path: Rc::new(RefCell::new(buf[2..110].to_vec().into_boxed_slice())),
        }
    }
}

impl ByteRepr for sockaddr_storage {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.ss_family.borrow()).to_bytes(&mut buf[0..2]);
        buf[2..128].copy_from_slice(&self.__pad.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ss_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            __pad: Rc::new(RefCell::new(buf[2..128].to_vec().into_boxed_slice())),
        }
    }
}

impl sockaddr_in {
    pub fn to_libc(&self) -> ::libc::sockaddr_in {
        let mut sin_zero = [0u8; 8];
        sin_zero.copy_from_slice(&self.sin_zero.borrow());
        ::libc::sockaddr_in {
            sin_family: *self.sin_family.borrow(),
            sin_port: *self.sin_port.borrow(),
            sin_addr: ::libc::in_addr {
                s_addr: *self.sin_addr.borrow().s_addr.borrow(),
            },
            sin_zero,
        }
    }
    pub fn from_libc(l: &::libc::sockaddr_in) -> Self {
        Self {
            sin_family: Rc::new(RefCell::new(l.sin_family)),
            sin_port: Rc::new(RefCell::new(l.sin_port)),
            sin_addr: Rc::new(RefCell::new(in_addr {
                s_addr: Rc::new(RefCell::new(l.sin_addr.s_addr)),
            })),
            sin_zero: Rc::new(RefCell::new(l.sin_zero.to_vec().into_boxed_slice())),
        }
    }
}

impl sockaddr_in6 {
    pub fn to_libc(&self) -> ::libc::sockaddr_in6 {
        let mut s6 = [0u8; 16];
        s6.copy_from_slice(&self.sin6_addr.borrow().s6_addr.borrow());
        ::libc::sockaddr_in6 {
            sin6_family: *self.sin6_family.borrow(),
            sin6_port: *self.sin6_port.borrow(),
            sin6_flowinfo: *self.sin6_flowinfo.borrow(),
            sin6_addr: ::libc::in6_addr { s6_addr: s6 },
            sin6_scope_id: *self.sin6_scope_id.borrow(),
        }
    }
    pub fn from_libc(l: &::libc::sockaddr_in6) -> Self {
        Self {
            sin6_family: Rc::new(RefCell::new(l.sin6_family)),
            sin6_port: Rc::new(RefCell::new(l.sin6_port)),
            sin6_flowinfo: Rc::new(RefCell::new(l.sin6_flowinfo)),
            sin6_addr: Rc::new(RefCell::new(in6_addr {
                s6_addr: Rc::new(RefCell::new(
                    l.sin6_addr.s6_addr.to_vec().into_boxed_slice(),
                )),
            })),
            sin6_scope_id: Rc::new(RefCell::new(l.sin6_scope_id)),
        }
    }
}

pub fn decode_sockaddr(
    addr: &Ptr<sockaddr>,
    _len: u32,
) -> Option<Box<dyn nix::sys::socket::SockaddrLike>> {
    let family = addr.reinterpret_cast::<u16>().read();
    if family == ::libc::AF_INET as u16 {
        let m = addr.reinterpret_cast::<sockaddr_in>().read();
        Some(Box::new(nix::sys::socket::SockaddrIn::from(m.to_libc())))
    } else if family == ::libc::AF_INET6 as u16 {
        let m = addr.reinterpret_cast::<sockaddr_in6>().read();
        Some(Box::new(nix::sys::socket::SockaddrIn6::from(m.to_libc())))
    } else if family == ::libc::AF_UNIX as u16 {
        let m = addr.reinterpret_cast::<sockaddr_un>().read();
        let path = m.sun_path.borrow();
        let end = path.iter().position(|&c| c == 0).unwrap_or(path.len());
        let bytes: Vec<u8> = path[..end].to_vec();
        nix::sys::socket::UnixAddr::new(&bytes[..])
            .ok()
            .map(|u| Box::new(u) as Box<dyn nix::sys::socket::SockaddrLike>)
    } else {
        None
    }
}

pub fn encode_sockaddr(
    ss: &nix::sys::socket::SockaddrStorage,
    out: &Ptr<sockaddr>,
    out_len: &Ptr<u32>,
) {
    use nix::sys::socket::{AddressFamily, SockaddrLike};
    match ss.family() {
        Some(AddressFamily::Inet) => {
            let l = ::libc::sockaddr_in::from(*ss.as_sockaddr_in().unwrap());
            out.reinterpret_cast::<sockaddr_in>()
                .write(sockaddr_in::from_libc(&l));
        }
        Some(AddressFamily::Inet6) => {
            let l = ::libc::sockaddr_in6::from(*ss.as_sockaddr_in6().unwrap());
            out.reinterpret_cast::<sockaddr_in6>()
                .write(sockaddr_in6::from_libc(&l));
        }
        _ => {}
    }
    out_len.write(ss.len());
}

impl ByteRepr for ::libc::sockaddr {}
impl ByteRepr for ::libc::sockaddr_in {}
impl ByteRepr for ::libc::sockaddr_in6 {}
impl ByteRepr for ::libc::sockaddr_un {}
impl ByteRepr for ::libc::sockaddr_storage {}
