// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use super::{Sockaddr, SockaddrIn, SockaddrIn6, SockaddrStorage};
use crate::{ByteRepr, Ptr};

#[derive(Default, Clone)]
pub struct Ifaddrs {
    pub ifa_next: Ptr<Ifaddrs>,
    pub ifa_name: Ptr<u8>,
    pub ifa_flags: u32,
    pub ifa_addr: Ptr<Sockaddr>,
    pub ifa_netmask: Ptr<Sockaddr>,
}

impl Ifaddrs {
    pub fn from_interface_address(ifa: &nix::ifaddrs::InterfaceAddress) -> Self {
        fn mk_addr(ss: Option<&nix::sys::socket::SockaddrStorage>) -> Ptr<Sockaddr> {
            match ss {
                None => Ptr::null(),
                Some(a) => match (a.as_sockaddr_in(), a.as_sockaddr_in6()) {
                    (Some(v4), _) => {
                        let l = ::libc::sockaddr_in::from(*v4);
                        let st = Ptr::alloc(SockaddrStorage::default());
                        st.reinterpret_cast::<SockaddrIn>()
                            .write(SockaddrIn::from_libc(&l));
                        st.reinterpret_cast::<Sockaddr>()
                    }
                    (None, Some(v6)) => {
                        let l = ::libc::sockaddr_in6::from(*v6);
                        let st = Ptr::alloc(SockaddrStorage::default());
                        st.reinterpret_cast::<SockaddrIn6>()
                            .write(SockaddrIn6::from_libc(&l));
                        st.reinterpret_cast::<Sockaddr>()
                    }
                    (None, None) => Ptr::null(),
                },
            }
        }
        let mut node = Ifaddrs::default();
        let mut name = ifa.interface_name.clone().into_bytes();
        name.push(0);
        node.ifa_name = Ptr::alloc_array(name.into_boxed_slice());
        node.ifa_flags = ifa.flags.bits() as u32;
        node.ifa_addr = mk_addr(ifa.address.as_ref());
        node.ifa_netmask = mk_addr(ifa.netmask.as_ref());
        node
    }
}

impl ByteRepr for Ifaddrs {
    fn byte_size() -> usize {
        56
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.ifa_next.to_bytes(&mut buf[0..8]);
        self.ifa_name.to_bytes(&mut buf[8..16]);
        self.ifa_flags.to_bytes(&mut buf[16..20]);
        self.ifa_addr.to_bytes(&mut buf[24..32]);
        self.ifa_netmask.to_bytes(&mut buf[32..40]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ifa_next: <Ptr<Ifaddrs>>::from_bytes(&buf[0..8]),
            ifa_name: <Ptr<u8>>::from_bytes(&buf[8..16]),
            ifa_flags: u32::from_bytes(&buf[16..20]),
            ifa_addr: <Ptr<Sockaddr>>::from_bytes(&buf[24..32]),
            ifa_netmask: <Ptr<Sockaddr>>::from_bytes(&buf[32..40]),
        }
    }
}

impl ByteRepr for ::libc::ifaddrs {}
