extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn emit_0(
    mut out: *mut ::libc::FILE,
    mut fmt: *const libc::c_char,
    __args: &[VaArg],
) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut rc: i32 = {
        let __f: String = ::std::ffi::CStr::from_ptr(fmt)
            .to_bytes()
            .iter()
            .map(|&b| b as char)
            .collect();
        let __s: Vec<u8> = libcc2rs::format_c(&__f, ap.remaining())
            .chars()
            .map(|c| c as u32 as u8)
            .collect();
        libc::fwrite(__s.as_ptr() as *const libc::c_void, 1, __s.len(), out) as i32
    };
    return rc;
}
pub unsafe fn emit_after_skip_1(
    mut out: *mut ::libc::FILE,
    mut fmt: *const libc::c_char,
    __args: &[VaArg],
) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut skipped: i32 = ap.arg::<i32>();
    let mut rc: i32 = {
        let __f: String = ::std::ffi::CStr::from_ptr(fmt)
            .to_bytes()
            .iter()
            .map(|&b| b as char)
            .collect();
        let __s: Vec<u8> = libcc2rs::format_c(&__f, ap.remaining())
            .chars()
            .map(|c| c as u32 as u8)
            .collect();
        libc::fwrite(__s.as_ptr() as *const libc::c_void, 1, __s.len(), out) as i32
    };
    return ((rc) + (skipped));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut path: *const libc::c_char = (c"cpp2rust_vfprintf.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(
        ((((unsafe {
            emit_0(
                fp,
                (c"%s=%d\n".as_ptr().cast_mut()).cast_const(),
                &[(c"count".as_ptr().cast_mut()).into(), (42).into()],
            )
        }) == (9)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            emit_after_skip_1(
                fp,
                (c"%c%d\n".as_ptr().cast_mut()).cast_const(),
                &[(100).into(), ('x' as i32).into(), (7).into()],
            )
        }) == (103)) as i32)
            != 0)
    );
    assert!(((((libcc2rs::fclose_unsafe(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(path, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut buf: [libc::c_char; 32] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
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
        ((((libcc2rs::fread_unsafe(
            ((buf.as_mut_ptr() as *mut libc::c_char) as *mut ::libc::c_void),
            1_usize,
            32_usize,
            fp
        )) == (12_usize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((buf.as_mut_ptr() as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                12_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((c"count=42\nx7\n".as_ptr().cast_mut() as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                12_usize as usize,
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
    assert!(((((libcc2rs::fclose_unsafe(fp)) == (0)) as i32) != 0));
    assert!(((((libcc2rs::unlink_unsafe(path)) == (0)) as i32) != 0));
    return 0;
}
