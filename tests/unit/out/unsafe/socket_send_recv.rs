extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_send_recv_0() {
    let mut sv: [i32; 2] = [0_i32; 2];
    assert!(((((libc::socketpair(1, libc::SOCK_STREAM, 0, sv.as_mut_ptr())) == (0)) as i32) != 0));
    let mut msg: *const libc::c_char = (c"ping".as_ptr().cast_mut()).cast_const();
    assert!(
        ((((libc::send(
            sv[(0) as usize],
            (msg as *const libc::c_char as *const ::libc::c_void),
            4_usize,
            0
        )) == (4_isize)) as i32)
            != 0)
    );
    let mut buf: [libc::c_char; 8] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    assert!(
        ((((libc::recv(
            sv[(1) as usize],
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            8_usize,
            0
        )) == (4_isize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"ping".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
                4_usize as usize,
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
    assert!(
        ((((libc::send(
            sv[(1) as usize],
            (c"pong!".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            5_usize,
            0
        )) == (5_isize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::recv(
            sv[(0) as usize],
            (buf.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void),
            8_usize,
            0
        )) == (5_isize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const libc::c_char as *const ::libc::c_void) as *const u8,
                5_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                (c"pong!".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void)
                    as *const u8,
                5_usize as usize,
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
}
pub unsafe fn test_send_recv_scalar_1() {
    let mut sv: [i32; 2] = [0_i32; 2];
    assert!(((((libc::socketpair(1, libc::SOCK_STREAM, 0, sv.as_mut_ptr())) == (0)) as i32) != 0));
    let mut value: i32 = 42;
    assert!(
        ((((libc::send(
            sv[(0) as usize],
            ((&mut value as *mut i32) as *const i32 as *const ::libc::c_void),
            ::std::mem::size_of::<i32>(),
            0
        ) as usize)
            == (::std::mem::size_of::<i32>())) as i32)
            != 0)
    );
    let mut received: i32 = 0;
    assert!(
        ((((libc::recv(
            sv[(1) as usize],
            ((&mut received as *mut i32) as *mut i32 as *mut ::libc::c_void),
            ::std::mem::size_of::<i32>(),
            0
        ) as usize)
            == (::std::mem::size_of::<i32>())) as i32)
            != 0)
    );
    assert!(((((received) == (42)) as i32) != 0));
}
pub unsafe fn test_send_bad_fd_2() {
    (*libcc2rs::cpp2rust_errno_unsafe()) = 0;
    assert!(
        ((((libc::send(
            -1_i32,
            (c"x".as_ptr().cast_mut() as *const libc::c_char as *const ::libc::c_void),
            1_usize,
            0
        )) == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!(((((*libcc2rs::cpp2rust_errno_unsafe()) == (9)) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_send_recv_0() });
    (unsafe { test_send_recv_scalar_1() });
    (unsafe { test_send_bad_fd_2() });
    return 0;
}
