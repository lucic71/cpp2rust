extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_poll_timeout_0() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    let mut pfd: pollfd = std::mem::zeroed::<pollfd>();
    pfd.fd = fds[(0) as usize];
    pfd.events = 1_i16;
    pfd.revents = 0_i16;
    assert!(
        ((((libc::poll((&mut pfd as *mut pollfd), 1_u64 as ::libc::nfds_t, 10)) == (0)) as i32)
            != 0)
    );
    assert!(((((pfd.revents as i32) == (0)) as i32) != 0));
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_poll_readable_1() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            fds[(1) as usize],
            (b"x\0".as_ptr().cast_mut() as *const u8 as *const ::libc::c_void),
            1_u64 as usize
        ) as i64)
            == (1_i64)) as i32)
            != 0)
    );
    let mut pfd: pollfd = std::mem::zeroed::<pollfd>();
    pfd.fd = fds[(0) as usize];
    pfd.events = 1_i16;
    pfd.revents = 0_i16;
    assert!(
        ((((libc::poll((&mut pfd as *mut pollfd), 1_u64 as ::libc::nfds_t, 100)) == (1)) as i32)
            != 0)
    );
    assert!((((((pfd.revents as i32) & (1)) != (0)) as i32) != 0));
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_poll_multiple_2() {
    let mut p1: [i32; 2] = [0_i32; 2];
    let mut p2: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(p1.as_mut_ptr())) == (0)) as i32) != 0));
    assert!(((((libc::pipe(p2.as_mut_ptr())) == (0)) as i32) != 0));
    assert!(
        ((((libc::write(
            p2[(1) as usize],
            (b"y\0".as_ptr().cast_mut() as *const u8 as *const ::libc::c_void),
            1_u64 as usize
        ) as i64)
            == (1_i64)) as i32)
            != 0)
    );
    let mut pfds: [pollfd; 2] = [std::mem::zeroed::<pollfd>(); 2];
    pfds[(0) as usize].fd = p1[(0) as usize];
    pfds[(0) as usize].events = 1_i16;
    pfds[(0) as usize].revents = 0_i16;
    pfds[(1) as usize].fd = p2[(0) as usize];
    pfds[(1) as usize].events = 1_i16;
    pfds[(1) as usize].revents = 0_i16;
    assert!(((((libc::poll(pfds.as_mut_ptr(), 2_u64 as ::libc::nfds_t, 100)) == (1)) as i32) != 0));
    assert!(((((pfds[(0) as usize].revents as i32) == (0)) as i32) != 0));
    assert!((((((pfds[(1) as usize].revents as i32) & (1)) != (0)) as i32) != 0));
    assert!(((((libc::close(p1[(0) as usize])) == (0)) as i32) != 0));
    assert!(((((libc::close(p1[(1) as usize])) == (0)) as i32) != 0));
    assert!(((((libc::close(p2[(0) as usize])) == (0)) as i32) != 0));
    assert!(((((libc::close(p2[(1) as usize])) == (0)) as i32) != 0));
}
pub unsafe fn test_poll_hup_3() {
    let mut fds: [i32; 2] = [0_i32; 2];
    assert!(((((libc::pipe(fds.as_mut_ptr())) == (0)) as i32) != 0));
    assert!(((((libc::close(fds[(1) as usize])) == (0)) as i32) != 0));
    let mut pfd: pollfd = std::mem::zeroed::<pollfd>();
    pfd.fd = fds[(0) as usize];
    pfd.events = 1_i16;
    pfd.revents = 0_i16;
    assert!(
        ((((libc::poll((&mut pfd as *mut pollfd), 1_u64 as ::libc::nfds_t, 100)) >= (1)) as i32)
            != 0)
    );
    assert!((((((pfd.revents as i32) & (16)) != (0)) as i32) != 0));
    assert!(((((libc::close(fds[(0) as usize])) == (0)) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_poll_timeout_0() });
    (unsafe { test_poll_readable_1() });
    (unsafe { test_poll_multiple_2() });
    (unsafe { test_poll_hup_3() });
    return 0;
}
