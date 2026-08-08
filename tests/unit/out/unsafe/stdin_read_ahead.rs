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
    let mut path: *const libc::c_char =
        (c"cpp2rust_read_ahead.tmp".as_ptr().cast_mut()).cast_const();
    let mut buf: [libc::c_char; 64] = [(0 as libc::c_char); 64];
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"w".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    libc::fputs(
        (c"line1\nline2\nline3\n".as_ptr().cast_mut()).cast_const(),
        fp,
    );
    libcc2rs::fclose_unsafe(fp);
    assert!(
        (((!((libc::freopen(
            path,
            (c"r".as_ptr().cast_mut()).cast_const(),
            libcc2rs::stdin_unsafe()
        ))
        .is_null())) as i32)
            != 0)
    );
    assert!(
        (((!((libc::fgets(
            buf.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 64]>() as i32),
            libcc2rs::stdin_unsafe()
        ))
        .is_null())) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"line1\n".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    let mut pipe: *mut ::libc::FILE = libc::popen(
        (c"cat".as_ptr().cast_mut()).cast_const(),
        (c"r".as_ptr().cast_mut()).cast_const(),
    );
    assert!((((!((pipe).is_null())) as i32) != 0));
    let mut n: usize = libcc2rs::fread_unsafe(
        ((buf.as_mut_ptr() as *mut libc::c_char) as *mut ::libc::c_void),
        1_usize,
        (::std::mem::size_of::<[libc::c_char; 64]>() as usize).wrapping_sub(1_usize),
        pipe,
    );
    assert!(((((libcc2rs::pclose_unsafe(pipe)) == (0)) as i32) != 0));
    assert!(((((n) == (0_usize)) as i32) != 0));
    fp = libc::fopen(path, (c"r".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(((((libc::fgetc(fp)) == ('l' as i32)) as i32) != 0));
    assert!(((((libc::ftell(fp) as i64) == (1_i64)) as i32) != 0));
    assert!(((((libc::fseek(fp, 5_i64 as ::libc::c_long, ::libc::SEEK_CUR)) == (0)) as i32) != 0));
    assert!(((((libc::ftell(fp) as i64) == (6_i64)) as i32) != 0));
    assert!(((((libc::fgetc(fp)) == ('l' as i32)) as i32) != 0));
    assert!(((((libc::ftell(fp) as i64) == (7_i64)) as i32) != 0));
    libcc2rs::fclose_unsafe(fp);
    assert!(((((libcc2rs::unlink_unsafe(path)) == (0)) as i32) != 0));
    return 0;
}
