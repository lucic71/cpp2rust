extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_stat_0() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_stat_test.tmp\0",
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
    {
        let __bytes: Vec<u8> = Ptr::from_string_literal(b"hello\0")
            .to_c_string_iterator()
            .collect();
        match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((libcc2rs::stat_refcount((*path.borrow()).clone(), (st.as_pointer()).clone()) == 0)
            as i32)
            != 0)
    );
    assert!(((((*st.borrow()).st_size == 5_i64) as i32) != 0));
    assert!(((((*st.borrow()).st_mtim.tv_sec > 0_i64) as i32) != 0));
    libcc2rs::unlink_refcount((*path.borrow()).clone());
}
pub fn test_fstat_1() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_fstat_test.tmp\0",
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
    {
        let __bytes: Vec<u8> = Ptr::from_string_literal(b"hello world\0")
            .to_c_string_iterator()
            .collect();
        match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    0;
    let fd: Value<i32> = Rc::new(RefCell::new((*fp.borrow()).with(|__f| __f.fd)));
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((libcc2rs::fstat_refcount((*fd.borrow()), (st.as_pointer()).clone()) == 0) as i32) != 0)
    );
    assert!(((((*st.borrow()).st_size == 11_i64) as i32) != 0));
    assert!(((((*st.borrow()).st_mtim.tv_sec > 0_i64) as i32) != 0));
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    libcc2rs::unlink_refcount((*path.borrow()).clone());
}
pub fn timespec_to_ms_2(tv: Ptr<libcc2rs::Timespec>) -> i64 {
    let tv: Value<Ptr<libcc2rs::Timespec>> = Rc::new(RefCell::new(tv));
    return {
        let _lhs = (((*tv.borrow()).with(|__v| (*__v).tv_sec) as i64) * 1000_i64);
        _lhs + ((*tv.borrow()).with(|__v| (*__v).tv_nsec) / 1000000_i64)
    };
}
pub fn test_timespec_members_3() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_stat_ts_test.tmp\0",
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
    {
        let __bytes: Vec<u8> = Ptr::from_string_literal(b"hi\0")
            .to_c_string_iterator()
            .collect();
        match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    assert!((((libcc2rs::fclose_refcount((*fp.borrow()).clone()) == 0) as i32) != 0));
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((libcc2rs::stat_refcount((*path.borrow()).clone(), (st.as_pointer()).clone()) == 0)
            as i32)
            != 0)
    );
    assert!(
        (((({
            timespec_to_ms_2(
                (st.as_pointer().field_ptr(
                    72,
                    |__v: &libcc2rs::Stat| ::std::slice::from_ref(&__v.st_atim),
                    |__v: &mut libcc2rs::Stat| ::std::slice::from_mut(&mut __v.st_atim),
                )),
            )
        }) >= ((*st.borrow()).st_atim.tv_sec * 1000_i64)) as i32)
            != 0)
    );
    assert!(
        (((({
            timespec_to_ms_2(
                (st.as_pointer().field_ptr(
                    88,
                    |__v: &libcc2rs::Stat| ::std::slice::from_ref(&__v.st_mtim),
                    |__v: &mut libcc2rs::Stat| ::std::slice::from_mut(&mut __v.st_mtim),
                )),
            )
        }) >= ((*st.borrow()).st_mtim.tv_sec * 1000_i64)) as i32)
            != 0)
    );
    assert!(
        (((({
            timespec_to_ms_2(
                (st.as_pointer().field_ptr(
                    104,
                    |__v: &libcc2rs::Stat| ::std::slice::from_ref(&__v.st_ctim),
                    |__v: &mut libcc2rs::Stat| ::std::slice::from_mut(&mut __v.st_ctim),
                )),
            )
        }) >= ((*st.borrow()).st_ctim.tv_sec * 1000_i64)) as i32)
            != 0)
    );
    assert!(((((*st.borrow()).st_mtim.tv_sec == (*st.borrow()).st_mtim.tv_sec) as i32) != 0));
    assert!(((((*st.borrow()).st_mtim.tv_nsec >= 0_i64) as i32) != 0));
    let copy: Value<libcc2rs::Timespec> = Rc::new(RefCell::new((*st.borrow()).st_mtim));
    assert!(((((*copy.borrow()).tv_sec == (*st.borrow()).st_mtim.tv_sec) as i32) != 0));
    libcc2rs::unlink_refcount((*path.borrow()).clone());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    ({ test_stat_0() });
    ({ test_fstat_1() });
    ({ test_timespec_members_3() });
    return 0;
}
