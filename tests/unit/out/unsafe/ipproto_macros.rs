extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut tcp: i32 = (libc::IPPROTO_TCP as i32);
    let mut udp: i32 = (libc::IPPROTO_UDP as i32);
    let mut ip: i32 = (libc::IPPROTO_IP as i32);
    return (((tcp) + (udp)) + (ip));
}
