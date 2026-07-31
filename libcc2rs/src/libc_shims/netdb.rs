// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use super::Sockaddr;
use crate::{ByteRepr, Ptr};

#[derive(Default, Clone)]
pub struct Addrinfo {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: u32,
    pub ai_addr: Ptr<Sockaddr>,
    pub ai_canonname: Ptr<u8>,
    pub ai_next: Ptr<Addrinfo>,
}

impl ByteRepr for Addrinfo {
    fn byte_size() -> usize {
        48
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.ai_flags.to_bytes(&mut buf[0..4]);
        self.ai_family.to_bytes(&mut buf[4..8]);
        self.ai_socktype.to_bytes(&mut buf[8..12]);
        self.ai_protocol.to_bytes(&mut buf[12..16]);
        self.ai_addrlen.to_bytes(&mut buf[16..20]);
        self.ai_addr.to_bytes(&mut buf[24..32]);
        self.ai_canonname.to_bytes(&mut buf[32..40]);
        self.ai_next.to_bytes(&mut buf[40..48]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ai_flags: i32::from_bytes(&buf[0..4]),
            ai_family: i32::from_bytes(&buf[4..8]),
            ai_socktype: i32::from_bytes(&buf[8..12]),
            ai_protocol: i32::from_bytes(&buf[12..16]),
            ai_addrlen: u32::from_bytes(&buf[16..20]),
            ai_addr: <Ptr<Sockaddr>>::from_bytes(&buf[24..32]),
            ai_canonname: <Ptr<u8>>::from_bytes(&buf[32..40]),
            ai_next: <Ptr<Addrinfo>>::from_bytes(&buf[40..48]),
        }
    }
}

impl ByteRepr for ::libc::addrinfo {}
