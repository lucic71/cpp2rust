extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_time_0() {
    assert!(((((libc::time(std::ptr::null_mut())) > (0_i64)) as i32) != 0));
}
pub unsafe fn test_gettimeofday_1() {
    let mut tv: timeval = std::mem::zeroed::<timeval>();
    tv.tv_sec = 0_i64;
    tv.tv_usec = 0_i64;
    assert!(
        ((((libc::gettimeofday(
            (&mut tv as *mut timeval),
            (0 as *mut ::libc::c_void) as *mut ::libc::timezone
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((tv.tv_sec) > (0_i64)) as i32) != 0));
    assert!(((((tv.tv_usec) >= (0_i64)) as i32) != 0));
    assert!(((((tv.tv_usec) < (1000000_i64)) as i32) != 0));
}
pub unsafe fn test_clock_gettime_2() {
    let mut ts: timespec = std::mem::zeroed::<timespec>();
    ts.tv_sec = 0_i64;
    ts.tv_nsec = 0_i64;
    assert!(((((libc::clock_gettime(1, (&mut ts as *mut timespec))) == (0)) as i32) != 0));
    assert!(((((ts.tv_sec) > (0_i64)) as i32) != 0));
    assert!(((((ts.tv_nsec) >= (0_i64)) as i32) != 0));
    assert!(((((ts.tv_nsec) < (1000000000_i64)) as i32) != 0));
}
pub unsafe fn test_localtime_r_3() {
    let mut t: i64 = 0_i64;
    let mut tm: tm = std::mem::zeroed::<tm>();
    assert!(
        (((!((libc::localtime_r((&mut t as *mut i64).cast_const(), (&mut tm as *mut tm)))
            .is_null())) as i32)
            != 0)
    );
    assert!(((((tm.tm_year) == (70)) as i32) != 0));
}
pub unsafe fn test_gmtime_r_4() {
    let mut t: i64 = 0_i64;
    let mut tm: tm = std::mem::zeroed::<tm>();
    assert!(
        (((!((libc::gmtime_r((&mut t as *mut i64).cast_const(), (&mut tm as *mut tm))).is_null()))
            as i32)
            != 0)
    );
    assert!(((((tm.tm_year) == (70)) as i32) != 0));
    assert!(((((tm.tm_mon) == (0)) as i32) != 0));
    assert!(((((tm.tm_mday) == (1)) as i32) != 0));
    assert!(((((tm.tm_hour) == (0)) as i32) != 0));
}
pub unsafe fn test_strftime_5() {
    let mut t: i64 = 0_i64;
    let mut tm: tm = std::mem::zeroed::<tm>();
    libc::gmtime_r((&mut t as *mut i64).cast_const(), (&mut tm as *mut tm));
    let mut buf: [u8; 64] = [0_u8; 64];
    assert!(
        ((((libc::strftime(
            buf.as_mut_ptr() as *mut i8,
            ::std::mem::size_of::<[u8; 64]>() as u64 as usize,
            (b"%Y-%m-%d\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (&mut tm as *mut tm).cast_const()
        ) as u64)
            == (10_u64)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const() as *const i8,
            (b"1970-01-01\0".as_ptr().cast_mut()).cast_const() as *const i8
        )) == (0)) as i32)
            != 0)
    );
}
pub unsafe fn test_utimes_6() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_utimes_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::std::fs::File =
        match std::ffi::CStr::from_ptr((b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8)
            .to_str()
            .expect("invalid c-string")
        {
            v if v == "rb" => std::fs::OpenOptions::new()
                .read(true)
                .open(
                    std::ffi::CStr::from_ptr(path as *const i8)
                        .to_str()
                        .expect("invalid c-string"),
                )
                .ok()
                .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
            v if v == "wb" => std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(
                    std::ffi::CStr::from_ptr(path as *const i8)
                        .to_str()
                        .expect("invalid c-string"),
                )
                .ok()
                .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
            _ => panic!("unsupported mode"),
        };
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(
        (((({
            Box::from_raw(fp);
            0
        }) == (0)) as i32)
            != 0)
    );
    let mut tv: [timeval; 2] = [std::mem::zeroed::<timeval>(); 2];
    tv[(0) as usize].tv_sec = 1000_i64;
    tv[(0) as usize].tv_usec = 0_i64;
    tv[(1) as usize].tv_sec = 2000_i64;
    tv[(1) as usize].tv_usec = 0_i64;
    assert!(
        ((((libc::utimes(path as *const i8, (tv.as_mut_ptr()).cast_const())) == (0)) as i32) != 0)
    );
    libc::unlink(path as *const i8);
    assert!(
        ((((libc::utimes(path as *const i8, (tv.as_mut_ptr()).cast_const())) == (-1_i32)) as i32)
            != 0)
    );
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_time_0() });
    (unsafe { test_gettimeofday_1() });
    (unsafe { test_clock_gettime_2() });
    (unsafe { test_localtime_r_3() });
    (unsafe { test_gmtime_r_4() });
    (unsafe { test_strftime_5() });
    (unsafe { test_utimes_6() });
    return 0;
}
