extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_socket_0() {
    let mut s: i32 = libc::socket(2, libc::SOCK_STREAM, 0);
    assert!(((((s) >= (0)) as i32) != 0));
    assert!(((((libc::close(s)) == (0)) as i32) != 0));
}
pub unsafe fn test_setsockopt_getsockopt_1() {
    let mut s: i32 = libc::socket(2, libc::SOCK_STREAM, 0);
    assert!(((((s) >= (0)) as i32) != 0));
    let mut set: i32 = 1;
    assert!(
        ((((libc::setsockopt(
            s,
            1,
            2,
            ((&mut set as *mut i32) as *const i32 as *const ::libc::c_void),
            (::std::mem::size_of::<i32>() as u64 as u32)
        )) == (0)) as i32)
            != 0)
    );
    let mut got: i32 = 0;
    let mut len: u32 = (::std::mem::size_of::<i32>() as u64 as u32);
    assert!(
        ((((libc::getsockopt(
            s,
            1,
            2,
            ((&mut got as *mut i32) as *mut i32 as *mut ::libc::c_void),
            (&mut len as *mut u32)
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((got) != (0)) as i32) != 0));
    assert!(((((libc::close(s)) == (0)) as i32) != 0));
}
pub unsafe fn test_send_recv_2() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::socketpair(1, libc::SOCK_STREAM, 0, fds.as_mut_ptr())) == (0)) as i32) != 0));
    let mut msg: *const u8 = (b"hello\0".as_ptr().cast_mut()).cast_const();
    assert!(
        ((((libc::send(
            fds[(0) as usize],
            (msg as *const u8 as *const ::libc::c_void),
            5_u64 as usize,
            0
        ) as i64)
            == (5_i64)) as i32)
            != 0)
    );
    let mut buf: [u8; 8] = [0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8];
    assert!(
        ((((libc::recv(
            fds[(1) as usize],
            (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
            8_u64 as usize,
            0
        ) as i64)
            == (5_i64)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                5_u64 as usize,
            );
            let sb = core::slice::from_raw_parts(
                (msg as *const u8 as *const ::libc::c_void) as *const u8,
                5_u64 as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_socket_0() });
    (unsafe { test_setsockopt_getsockopt_1() });
    (unsafe { test_send_recv_2() });
    return 0;
}
