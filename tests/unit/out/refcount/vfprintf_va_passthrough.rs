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
        let __s = libcc2rs::format_c(
            &(*fmt.borrow()).to_rust_string(),
            (*ap.borrow()).remaining(),
        );
        let __bytes = __s.as_bytes();
        match (*out.borrow()).with_mut(|__f| __f.write(__bytes)) == __bytes.len() {
            true => __bytes.len() as i32,
            false => -1,
        }
    }));
    return (*rc.borrow());
}
pub fn emit_after_skip_1(out: Ptr<CFile>, fmt: Ptr<u8>, __args: &[VaArg]) -> i32 {
    let out: Value<Ptr<CFile>> = Rc::new(RefCell::new(out));
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let skipped: Value<i32> = Rc::new(RefCell::new((*ap.borrow_mut()).arg::<i32>()));
    let rc: Value<i32> = Rc::new(RefCell::new({
        let __s = libcc2rs::format_c(
            &(*fmt.borrow()).to_rust_string(),
            (*ap.borrow()).remaining(),
        );
        let __bytes = __s.as_bytes();
        match (*out.borrow()).with_mut(|__f| __f.write(__bytes)) == __bytes.len() {
            true => __bytes.len() as i32,
            false => -1,
        }
    }));
    return ((*rc.borrow()) + (*skipped.borrow()));
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_vfprintf.tmp\0",
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
        (((({
            emit_0(
                (*fp.borrow()).clone(),
                Ptr::from_string_literal(b"%s=%d\n\0"),
                &[(Ptr::from_string_literal(b"count\0")).into(), (42).into()],
            )
        }) == 9) as i32)
            != 0)
    );
    assert!(
        (((({
            emit_after_skip_1(
                (*fp.borrow()).clone(),
                Ptr::from_string_literal(b"%c%d\n\0"),
                &[(100).into(), ('x' as i32).into(), (7).into()],
            )
        }) == 103) as i32)
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
        <u8>::default(),
    ])));
    assert!(
        ((({
            let __a0 = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone();
            let __a1 = 1_usize;
            let __a2 = 32_usize;
            let __a3 = (*fp.borrow()).clone();
            libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
        } == 12_usize) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(
                &Ptr::from_string_literal(b"count=42\nx7\n\0").to_any(),
                12_usize
            )
            == 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    assert!((((libcc2rs::unlink_refcount((*path.borrow()).clone()) == 0) as i32) != 0));
    return 0;
}
