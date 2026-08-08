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
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut high: *const libc::c_char = (c"\x81\xff\xc4".as_ptr().cast_mut()).cast_const();
    let mut buf: [libc::c_char; 32] = [(0 as libc::c_char); 32];
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"[%s]%c".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (high),
                (228),
            )
        }) == (6)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((buf.as_mut_ptr() as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                6_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((c"[\x81\xff\xc4]\xe4".as_ptr().cast_mut() as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                6_usize as usize,
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
    assert!(((((buf[((6) as usize)] as i32) == (0)) as i32) != 0));
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%.*s".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (3),
                (high),
            )
        }) == (3)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((buf.as_mut_ptr() as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                3_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((c"\x81\xff\xc4".as_ptr().cast_mut() as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                3_usize as usize,
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
    assert!(((((buf[((3) as usize)] as i32) == (0)) as i32) != 0));
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"[%.*s]".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (2),
                (high),
            )
        }) == (4)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((buf.as_mut_ptr() as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                4_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((c"[\x81\xff]".as_ptr().cast_mut() as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
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
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%.2s".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (high),
            )
        }) == (2)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((buf.as_mut_ptr() as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                2_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((c"\x81\xff".as_ptr().cast_mut() as *const libc::c_char) as *const ::libc::c_void)
                    as *const u8,
                2_usize as usize,
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
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%.16s".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (high),
            )
        }) == (3)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((buf.as_mut_ptr() as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                3_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((c"\x81\xff\xc4".as_ptr().cast_mut() as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                3_usize as usize,
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
    let unterminated: [libc::c_char; 3] = [
        ((b'\x81' as i32) as libc::c_char),
        ((b'\xff' as i32) as libc::c_char),
        ((b'\xc4' as i32) as libc::c_char),
    ];
    assert!(
        ((((unsafe {
            libc::snprintf(
                buf.as_mut_ptr() as *mut libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 32]>() as usize,
                (c"%.*s".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
                (3),
                (unterminated.as_ptr()),
            )
        }) == (3)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((buf.as_mut_ptr() as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                3_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((c"\x81\xff\xc4".as_ptr().cast_mut() as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                3_usize as usize,
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
    let mut path: *const libc::c_char =
        (c"cpp2rust_high_bytes.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(path, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(
        ((((unsafe {
            let _va0 = high;
            let _va3 = high;
            emit_0(
                fp,
                (c"%s%c%.*s\n".as_ptr().cast_mut()).cast_const(),
                &[(_va0).into(), (128).into(), (2).into(), (_va3).into()],
            )
        }) == (7)) as i32)
            != 0)
    );
    assert!(((((libcc2rs::fclose_unsafe(fp)) == (0)) as i32) != 0));
    fp = libc::fopen(path, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut rd: [libc::c_char; 16] = [
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
            ((rd.as_mut_ptr() as *mut libc::c_char) as *mut ::libc::c_void),
            1_usize,
            ::std::mem::size_of::<[libc::c_char; 16]>(),
            fp
        )) == (7_usize)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                ((rd.as_mut_ptr() as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                7_usize as usize,
            );
            let sb = core::slice::from_raw_parts(
                ((c"\x81\xff\xc4\x80\x81\xff\n".as_ptr().cast_mut() as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                7_usize as usize,
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
