extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn emit_0(out: Ptr<CFile>, fmt: Ptr<u8>, __args: &[VaArg]) -> i32 {
    let out: Value<Ptr<CFile>> = Rc::new(RefCell::new(out));
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let rc: Value<i32> = Rc::new(RefCell::new({
        let __fmt: String = (*fmt.borrow())
            .to_c_string_iterator()
            .map(|b| b as char)
            .collect();
        let __bytes: Vec<u8> = libcc2rs::format_c(&__fmt, (*ap.borrow()).remaining())
            .chars()
            .map(|c| c as u32 as u8)
            .collect();
        match (*out.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => __bytes.len() as i32,
            false => -1,
        }
    }));
    return (*rc.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let high: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"\x81\xff\xc4\0")));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..32).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        ((({
            let __fmt: String = Ptr::from_string_literal(b"[%s]%c\0")
                .to_c_string_iterator()
                .map(|b| b as char)
                .collect();
            let __b: Vec<u8> =
                libcc2rs::format_c(&__fmt, &[((*high.borrow()).clone()).into(), (228).into()])
                    .chars()
                    .map(|c| c as u32 as u8)
                    .collect();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 6) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(
                &Ptr::from_string_literal(b"[\x81\xff\xc4]\xe4\0").to_any(),
                6_usize
            )
            == 0) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(6) as usize] as i32) == 0) as i32) != 0));
    assert!(
        ((({
            let __fmt: String = Ptr::from_string_literal(b"%.*s\0")
                .to_c_string_iterator()
                .map(|b| b as char)
                .collect();
            let __b: Vec<u8> =
                libcc2rs::format_c(&__fmt, &[(3).into(), ((*high.borrow()).clone()).into()])
                    .chars()
                    .map(|c| c as u32 as u8)
                    .collect();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 3) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(
                &Ptr::from_string_literal(b"\x81\xff\xc4\0").to_any(),
                3_usize
            )
            == 0) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(3) as usize] as i32) == 0) as i32) != 0));
    assert!(
        ((({
            let __fmt: String = Ptr::from_string_literal(b"[%.*s]\0")
                .to_c_string_iterator()
                .map(|b| b as char)
                .collect();
            let __b: Vec<u8> =
                libcc2rs::format_c(&__fmt, &[(2).into(), ((*high.borrow()).clone()).into()])
                    .chars()
                    .map(|c| c as u32 as u8)
                    .collect();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 4) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"[\x81\xff]\0").to_any(), 4_usize)
            == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __fmt: String = Ptr::from_string_literal(b"%.2s\0")
                .to_c_string_iterator()
                .map(|b| b as char)
                .collect();
            let __b: Vec<u8> = libcc2rs::format_c(&__fmt, &[((*high.borrow()).clone()).into()])
                .chars()
                .map(|c| c as u32 as u8)
                .collect();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 2) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"\x81\xff\0").to_any(), 2_usize)
            == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __fmt: String = Ptr::from_string_literal(b"%.16s\0")
                .to_c_string_iterator()
                .map(|b| b as char)
                .collect();
            let __b: Vec<u8> = libcc2rs::format_c(&__fmt, &[((*high.borrow()).clone()).into()])
                .chars()
                .map(|c| c as u32 as u8)
                .collect();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 3) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(
                &Ptr::from_string_literal(b"\x81\xff\xc4\0").to_any(),
                3_usize
            )
            == 0) as i32)
            != 0)
    );
    let unterminated: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ((b'\x81' as i32) as u8),
        ((b'\xff' as i32) as u8),
        ((b'\xc4' as i32) as u8),
    ])));
    assert!(
        ((({
            let __fmt: String = Ptr::from_string_literal(b"%.*s\0")
                .to_c_string_iterator()
                .map(|b| b as char)
                .collect();
            let __b: Vec<u8> = libcc2rs::format_c(
                &__fmt,
                &[(3).into(), (unterminated.as_pointer() as Ptr<u8>).into()],
            )
            .chars()
            .map(|c| c as u32 as u8)
            .collect();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 3) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(
                &Ptr::from_string_literal(b"\x81\xff\xc4\0").to_any(),
                3_usize
            )
            == 0) as i32)
            != 0)
    );
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_high_bytes.tmp\0",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb\0").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::<CFile>::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(
        (((({
            let _va0 = (*high.borrow()).clone();
            let _va3 = (*high.borrow()).clone();
            emit_0(
                (*fp.borrow()).clone(),
                Ptr::from_string_literal(b"%s%c%.*s\n\0"),
                &[(_va0).into(), (128).into(), (2).into(), (_va3).into()],
            )
        }) == 7) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    (*fp.borrow_mut()) = match CFile::open(
        &(*path.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"rb\0").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::<CFile>::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    let rd: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
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
            let __a0 = ((rd.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone();
            let __a1 = 1_usize;
            let __a2 = 16usize;
            let __a3 = (*fp.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        } == 7_usize) as i32)
            != 0)
    );
    assert!(
        (((((rd.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(
                &Ptr::from_string_literal(b"\x81\xff\xc4\x80\x81\xff\n\0").to_any(),
                7_usize
            )
            == 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    assert!((((libcc2rs::unlink_refcount((*path.borrow()).clone()) == 0) as i32) != 0));
    return 0;
}
