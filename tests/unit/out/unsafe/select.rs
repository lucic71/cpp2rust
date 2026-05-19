extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fd_set_bit_0(mut fd: i32, mut set: *mut fd_set) {
    let mut p: *mut u8 = (set as *mut u8);
    (*p.offset(((fd) / (8)) as isize)) = (((*p.offset(((fd) / (8)) as isize)) as i32)
        | ((((1) << ((fd) % (8))) as u8) as i32)) as u8;
}
pub unsafe fn fd_isset_bit_1(mut fd: i32, mut set: *const fd_set) -> i32 {
    let mut p: *const u8 = (set as *const u8);
    return ((((*p.offset(((fd) / (8)) as isize)) as i32) >> ((fd) % (8))) & (1));
}
pub unsafe fn test_select_2() {
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
    let mut rfds: fd_set = std::mem::zeroed::<fd_set>();
    {
        let byte_0 = ((&mut rfds as *mut fd_set) as *mut fd_set as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<fd_set>() as u64 {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut rfds as *mut fd_set) as *mut fd_set as *mut ::libc::c_void)
    };
    (unsafe {
        let _fd: i32 = fds[(0) as usize];
        let _set: *mut fd_set = (&mut rfds as *mut fd_set);
        fd_set_bit_0(_fd, _set)
    });
    let mut tv: timeval = std::mem::zeroed::<timeval>();
    tv.tv_sec = 0_i64;
    tv.tv_usec = 100000_i64;
    assert!(
        ((((libc::select(
            ((fds[(0) as usize]) + (1)),
            (&mut rfds as *mut fd_set),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            (&mut tv as *mut timeval)
        )) == (1)) as i32)
            != 0)
    );
    assert!(
        ((unsafe {
            let _fd: i32 = fds[(0) as usize];
            let _set: *const fd_set = (&mut rfds as *mut fd_set).cast_const();
            fd_isset_bit_1(_fd, _set)
        }) != 0)
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
    (unsafe { test_select_2() });
    return 0;
}
