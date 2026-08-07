extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_stat_0() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_stat_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello".as_ptr().cast_mut()).cast_const(), fp);
    assert!(((((libcc2rs::fclose_unsafe(fp)) == (0)) as i32) != 0));
    let mut st: libcc2rs::Stat = Default::default();
    assert!(
        ((((libcc2rs::stat_unsafe(path, (&raw mut st as *mut libcc2rs::Stat))) == (0)) as i32)
            != 0)
    );
    assert!(((((st.st_size) == (5_i64)) as i32) != 0));
    assert!(((((st.st_mtim.tv_sec) > (0_i64)) as i32) != 0));
    libcc2rs::unlink_unsafe(path);
}
pub unsafe fn test_fstat_1() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_fstat_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hello world".as_ptr().cast_mut()).cast_const(), fp);
    libc::fflush(fp);
    let mut fd: i32 = libc::fileno(fp);
    let mut st: libcc2rs::Stat = Default::default();
    assert!(
        ((((libcc2rs::fstat_unsafe(fd, (&raw mut st as *mut libcc2rs::Stat))) == (0)) as i32) != 0)
    );
    assert!(((((st.st_size) == (11_i64)) as i32) != 0));
    assert!(((((st.st_mtim.tv_sec) > (0_i64)) as i32) != 0));
    assert!(((((libcc2rs::fclose_unsafe(fp)) == (0)) as i32) != 0));
    libcc2rs::unlink_unsafe(path);
}
pub unsafe fn timespec_to_ms_2(mut tv: *const libcc2rs::Timespec) -> i64 {
    return ((((*tv).tv_sec as i64) * (1000_i64)) + (((*tv).tv_nsec) / (1000000_i64)));
}
pub unsafe fn test_timespec_members_3() {
    let mut path: *const libc::c_char =
        (c"cpp2rust_stat_ts_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs((c"hi".as_ptr().cast_mut()).cast_const(), fp);
    assert!(((((libcc2rs::fclose_unsafe(fp)) == (0)) as i32) != 0));
    let mut st: libcc2rs::Stat = Default::default();
    assert!(
        ((((libcc2rs::stat_unsafe(path, (&raw mut st as *mut libcc2rs::Stat))) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            timespec_to_ms_2((&raw mut st.st_atim as *mut libcc2rs::Timespec).cast_const())
        }) >= ((st.st_atim.tv_sec) * (1000_i64))) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            timespec_to_ms_2((&raw mut st.st_mtim as *mut libcc2rs::Timespec).cast_const())
        }) >= ((st.st_mtim.tv_sec) * (1000_i64))) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            timespec_to_ms_2((&raw mut st.st_ctim as *mut libcc2rs::Timespec).cast_const())
        }) >= ((st.st_ctim.tv_sec) * (1000_i64))) as i32)
            != 0)
    );
    assert!(((((st.st_mtim.tv_sec) == (st.st_mtim.tv_sec)) as i32) != 0));
    assert!(((((st.st_mtim.tv_nsec) >= (0_i64)) as i32) != 0));
    let mut copy: libcc2rs::Timespec = st.st_mtim;
    assert!(((((copy.tv_sec) == (st.st_mtim.tv_sec)) as i32) != 0));
    libcc2rs::unlink_unsafe(path);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_stat_0() });
    (unsafe { test_fstat_1() });
    (unsafe { test_timespec_members_3() });
    return 0;
}
