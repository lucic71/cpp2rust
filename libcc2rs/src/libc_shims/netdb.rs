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

impl ByteRepr for Addrinfo {}

impl ByteRepr for ::libc::addrinfo {}
