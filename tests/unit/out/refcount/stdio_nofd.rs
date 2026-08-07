extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_fputc_fputs_0() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_stdio_nofd_puts.tmp\0",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb\0").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let __c = ('A' as i32) as u8;
            match (*fp.borrow()).with_mut(|__f| __f.write(&[__c])) {
                1 => __c as i32,
                _ => -1,
            }
        } == ('A' as i32)) as i32)
            != 0)
    );
    assert!(
        ((({
            let __bytes: Vec<u8> = Ptr::from_string_literal(b"BCD\n\0")
                .to_c_string_iterator()
                .collect();
            match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        } >= 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    (*fp.borrow_mut()) = match CFile::open(
        &(*path.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"rb\0").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    assert!(
        ((({
            let __a0 = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone();
            let __a1 = 1_usize;
            let __a2 = 16_usize;
            let __a3 = (*fp.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        } == 5_usize) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"ABCD\n\0").to_any(), 5_usize)
            == 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    assert!((((libcc2rs::unlink_refcount((*path.borrow()).clone()) == 0) as i32) != 0));
}
pub fn test_puts_1() {
    assert!(
        ((({
            let mut __bytes: Vec<u8> = Ptr::from_string_literal(b"hello from puts\0")
                .to_c_string_iterator()
                .collect();
            __bytes.push(b'\n');
            match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        } >= 0) as i32)
            != 0)
    );
}
pub fn test_fgets_getc_2() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_stdio_nofd_gets.tmp\0",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb\0").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let __bytes: Vec<u8> = Ptr::from_string_literal(b"line1\nline2\n\0")
                .to_c_string_iterator()
                .collect();
            match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        } >= 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    (*fp.borrow_mut()) = match CFile::open(
        &(*path.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"rb\0").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        (((!(({
            let __buf = (buf.as_pointer() as Ptr<u8>).clone();
            let __n = 8;
            if __n <= 0 {
                Ptr::null()
            } else {
                let __max = (__n - 1) as usize;
                let mut __dst = __buf.clone();
                let mut __count: usize = 0;
                let __failed = (*fp.borrow()).with_mut(|__f| {
                    while __count < __max {
                        let __c = __f.getc();
                        if __c < 0 {
                            break;
                        }
                        __dst.write(__c as u8);
                        __dst += 1;
                        __count += 1;
                        if __c as u8 == b'\n' {
                            break;
                        }
                    }
                    __f.err
                });
                if __failed || __count == 0 {
                    Ptr::null()
                } else {
                    __dst.write(0);
                    __buf
                }
            }
        })
        .is_null())) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"line1\n\0").to_any(), 7_usize)
            == 0) as i32)
            != 0)
    );
    assert!(((((*fp.borrow()).with_mut(|__f| __f.getc()) == ('l' as i32)) as i32) != 0));
    assert!(
        (((!(({
            let __buf = (buf.as_pointer() as Ptr<u8>).clone();
            let __n = 4;
            if __n <= 0 {
                Ptr::null()
            } else {
                let __max = (__n - 1) as usize;
                let mut __dst = __buf.clone();
                let mut __count: usize = 0;
                let __failed = (*fp.borrow()).with_mut(|__f| {
                    while __count < __max {
                        let __c = __f.getc();
                        if __c < 0 {
                            break;
                        }
                        __dst.write(__c as u8);
                        __dst += 1;
                        __count += 1;
                        if __c as u8 == b'\n' {
                            break;
                        }
                    }
                    __f.err
                });
                if __failed || __count == 0 {
                    Ptr::null()
                } else {
                    __dst.write(0);
                    __buf
                }
            }
        })
        .is_null())) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"ine\0").to_any(), 4_usize)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((!(({
            let __buf = (buf.as_pointer() as Ptr<u8>).clone();
            let __n = 8;
            if __n <= 0 {
                Ptr::null()
            } else {
                let __max = (__n - 1) as usize;
                let mut __dst = __buf.clone();
                let mut __count: usize = 0;
                let __failed = (*fp.borrow()).with_mut(|__f| {
                    while __count < __max {
                        let __c = __f.getc();
                        if __c < 0 {
                            break;
                        }
                        __dst.write(__c as u8);
                        __dst += 1;
                        __count += 1;
                        if __c as u8 == b'\n' {
                            break;
                        }
                    }
                    __f.err
                });
                if __failed || __count == 0 {
                    Ptr::null()
                } else {
                    __dst.write(0);
                    __buf
                }
            }
        })
        .is_null())) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"2\n\0").to_any(), 3_usize)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((({
            let __buf = (buf.as_pointer() as Ptr<u8>).clone();
            let __n = 8;
            if __n <= 0 {
                Ptr::null()
            } else {
                let __max = (__n - 1) as usize;
                let mut __dst = __buf.clone();
                let mut __count: usize = 0;
                let __failed = (*fp.borrow()).with_mut(|__f| {
                    while __count < __max {
                        let __c = __f.getc();
                        if __c < 0 {
                            break;
                        }
                        __dst.write(__c as u8);
                        __dst += 1;
                        __count += 1;
                        if __c as u8 == b'\n' {
                            break;
                        }
                    }
                    __f.err
                });
                if __failed || __count == 0 {
                    Ptr::null()
                } else {
                    __dst.write(0);
                    __buf
                }
            }
        })
        .is_null()) as i32)
            != 0)
    );
    assert!(((((*fp.borrow()).with_mut(|__f| __f.getc()) == (-1_i32)) as i32) != 0));
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    assert!((((libcc2rs::unlink_refcount((*path.borrow()).clone()) == 0) as i32) != 0));
}
pub fn test_freopen_3() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_stdio_nofd_reopen.tmp\0",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb\0").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let __bytes: Vec<u8> = Ptr::from_string_literal(b"hello\0")
                .to_c_string_iterator()
                .collect();
            match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        } >= 0) as i32)
            != 0)
    );
    let fp2: Value<Ptr<CFile>> = Rc::new(RefCell::new({
        let __stream = (*fp.borrow()).clone();
        let __old = __stream.with(|__f| __f.fd);
        match __old {
            0..=2 => {}
            __fd => {
                FdRegistry::close(__fd);
            }
        }
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"rb\0").to_rust_string(),
        ) {
            Some(__f) => {
                __stream.write(__f);
                __stream
            }
            None => Ptr::null(),
        }
    }));
    assert!((((!((*fp2.borrow()).is_null())) as i32) != 0));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    assert!(
        ((({
            let __a0 = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone();
            let __a1 = 1_usize;
            let __a2 = 8_usize;
            let __a3 = (*fp2.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        } == 5_usize) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"hello\0").to_any(), 5_usize)
            == 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp2.borrow()).clone()) == 0) as i32) != 0));
    assert!((((libcc2rs::unlink_refcount((*path.borrow()).clone()) == 0) as i32) != 0));
}
pub fn test_fseeko_4() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_stdio_nofd_seek.tmp\0",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb\0").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let __bytes: Vec<u8> = Ptr::from_string_literal(b"hello world\0")
                .to_c_string_iterator()
                .collect();
            match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        } >= 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    (*fp.borrow_mut()) = match CFile::open(
        &(*path.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"rb\0").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        (((match (*fp.borrow()).with_mut(|__f| __f.seek(6_i64, ::libc::SEEK_SET)) {
            -1 => -1,
            _ => 0,
        } == 0) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    assert!(
        ((({
            let __a0 = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone();
            let __a1 = 1_usize;
            let __a2 = 5_usize;
            let __a3 = (*fp.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        } == 5_usize) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"world\0").to_any(), 5_usize)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((match (*fp.borrow()).with_mut(|__f| __f.seek((-5_i32 as i64), ::libc::SEEK_END)) {
            -1 => -1,
            _ => 0,
        } == 0) as i32)
            != 0)
    );
    assert!(((((*fp.borrow()).with_mut(|__f| __f.getc()) == ('w' as i32)) as i32) != 0));
    assert!(
        (((match (*fp.borrow()).with_mut(|__f| __f.seek(1_i64, ::libc::SEEK_CUR)) {
            -1 => -1,
            _ => 0,
        } == 0) as i32)
            != 0)
    );
    assert!(((((*fp.borrow()).with_mut(|__f| __f.getc()) == ('r' as i32)) as i32) != 0));
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    assert!((((libcc2rs::unlink_refcount((*path.borrow()).clone()) == 0) as i32) != 0));
}
pub fn test_rename_5() {
    let from: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_stdio_nofd_from.tmp\0",
    )));
    let to: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_stdio_nofd_to.tmp\0",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*from.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb\0").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let __bytes: Vec<u8> = Ptr::from_string_literal(b"data\0")
                .to_c_string_iterator()
                .collect();
            match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        } >= 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    assert!(
        (((match ::std::fs::rename(
            (*from.borrow()).to_rust_string(),
            (*to.borrow()).to_rust_string()
        ) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((((match CFile::open(
            &(*from.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"rb\0").to_rust_string()
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        })
        .is_null()) as i32)
            != 0)
    );
    (*fp.borrow_mut()) = match CFile::open(
        &(*to.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"rb\0").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    assert!(
        (((match ::std::fs::rename(
            (*from.borrow()).to_rust_string(),
            (*to.borrow()).to_rust_string()
        ) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
                -1
            }
        } == -1_i32) as i32)
            != 0)
    );
    assert!((((libcc2rs::unlink_refcount((*to.borrow()).clone()) == 0) as i32) != 0));
}
pub fn test_setvbuf_6() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_stdio_nofd_vbuf.tmp\0",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb\0").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!((((0 == 0) as i32) != 0));
    assert!(
        ((({
            let __bytes: Vec<u8> = Ptr::from_string_literal(b"x\0")
                .to_c_string_iterator()
                .collect();
            match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        } >= 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    assert!((((libcc2rs::unlink_refcount((*path.borrow()).clone()) == 0) as i32) != 0));
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    ({ test_fputc_fputs_0() });
    ({ test_puts_1() });
    ({ test_fgets_getc_2() });
    ({ test_freopen_3() });
    ({ test_fseeko_4() });
    ({ test_rename_5() });
    ({ test_setvbuf_6() });
    return 0;
}
