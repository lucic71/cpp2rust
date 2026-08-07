extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_setenv_getenv_0() {
    assert!(
        (((match 1 != 0
            || ::std::env::var_os(Ptr::from_string_literal(b"CPP2RUST_TEST_VAR\0").to_rust_string())
                .is_none()
        {
            true => {
                unsafe {
                    ::std::env::set_var(
                        Ptr::from_string_literal(b"CPP2RUST_TEST_VAR\0").to_rust_string(),
                        Ptr::from_string_literal(b"test_value\0").to_rust_string(),
                    )
                };
                0
            }
            false => 0,
        } == 0) as i32)
            != 0)
    );
    let v: Value<Ptr<u8>> = Rc::new(RefCell::new(
        match ::std::env::var(Ptr::from_string_literal(b"CPP2RUST_TEST_VAR\0").to_rust_string()) {
            Ok(__val) => {
                let mut __bytes = __val.into_bytes();
                __bytes.push(0);
                Ptr::alloc_array(__bytes.into_boxed_slice())
            }
            Err(_) => Ptr::<u8>::null(),
        },
    ));
    assert!((((!((*v.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (*v.borrow()).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"test_value\0").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((match 1 != 0
            || ::std::env::var_os(Ptr::from_string_literal(b"CPP2RUST_TEST_VAR\0").to_rust_string())
                .is_none()
        {
            true => {
                unsafe {
                    ::std::env::set_var(
                        Ptr::from_string_literal(b"CPP2RUST_TEST_VAR\0").to_rust_string(),
                        Ptr::from_string_literal(b"replaced\0").to_rust_string(),
                    )
                };
                0
            }
            false => 0,
        } == 0) as i32)
            != 0)
    );
    (*v.borrow_mut()) =
        match ::std::env::var(Ptr::from_string_literal(b"CPP2RUST_TEST_VAR\0").to_rust_string()) {
            Ok(__val) => {
                let mut __bytes = __val.into_bytes();
                __bytes.push(0);
                Ptr::alloc_array(__bytes.into_boxed_slice())
            }
            Err(_) => Ptr::<u8>::null(),
        };
    assert!((((!((*v.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (*v.borrow()).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"replaced\0").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
}
pub fn test_realpath_1() {
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4096).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        (((!(({
            let __resolved = (buf.as_pointer() as Ptr<u8>).clone();
            match ::std::fs::canonicalize(Ptr::from_string_literal(b"/\0").to_rust_string()) {
                Ok(__p) => {
                    let mut __bytes = __p.into_os_string().into_encoded_bytes();
                    __bytes.push(0);
                    if __resolved.is_null() {
                        Ptr::alloc_array(__bytes.into_boxed_slice())
                    } else {
                        __resolved
                            .with_slice_mut(__bytes.len(), |__s| __s.copy_from_slice(&__bytes));
                        __resolved
                    }
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
                    Ptr::<u8>::null()
                }
            }
        })
        .is_null())) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"/\0").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new({
        let __resolved = Ptr::<u8>::null().clone();
        match ::std::fs::canonicalize(Ptr::from_string_literal(b"/\0").to_rust_string()) {
            Ok(__p) => {
                let mut __bytes = __p.into_os_string().into_encoded_bytes();
                __bytes.push(0);
                if __resolved.is_null() {
                    Ptr::alloc_array(__bytes.into_boxed_slice())
                } else {
                    __resolved.with_slice_mut(__bytes.len(), |__s| __s.copy_from_slice(&__bytes));
                    __resolved
                }
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
                Ptr::<u8>::null()
            }
        }
    }));
    assert!((((!((*p.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (*p.borrow()).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"/\0").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*p.borrow()).clone() as Ptr<u8>).to_any().clone());
    libcc2rs::cpp2rust_errno().write(0);
    assert!(
        (((({
            let __resolved = (buf.as_pointer() as Ptr<u8>).clone();
            match ::std::fs::canonicalize(
                Ptr::from_string_literal(b"/cpp2rust_definitely_missing\0").to_rust_string(),
            ) {
                Ok(__p) => {
                    let mut __bytes = __p.into_os_string().into_encoded_bytes();
                    __bytes.push(0);
                    if __resolved.is_null() {
                        Ptr::alloc_array(__bytes.into_boxed_slice())
                    } else {
                        __resolved
                            .with_slice_mut(__bytes.len(), |__s| __s.copy_from_slice(&__bytes));
                        __resolved
                    }
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
                    Ptr::<u8>::null()
                }
            }
        })
        .is_null()) as i32)
            != 0)
    );
    assert!(((((libcc2rs::cpp2rust_errno().read()) == libc::ENOENT) as i32) != 0));
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    ({ test_setenv_getenv_0() });
    ({ test_realpath_1() });
    return 0;
}
