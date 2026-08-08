extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"cpp2rust_read_ahead.tmp\0",
    )));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"w\0").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::<CFile>::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    {
        let __bytes: Vec<u8> = Ptr::from_string_literal(b"line1\nline2\nline3\n\0")
            .to_c_string_iterator()
            .collect();
        match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    libcc2rs::fclose_refcount((*fp.borrow()).clone());
    assert!(
        (((!(({
            let __stream = libcc2rs::c_stdin().clone();
            let __old = __stream.with(|__f| __f.fd);
            match __old {
                0..=2 => {}
                __fd => {
                    FdRegistry::close(__fd);
                }
            }
            match CFile::open(
                &(*path.borrow()).to_rust_string(),
                &Ptr::from_string_literal(b"r\0").to_rust_string(),
            ) {
                Some(__f) => {
                    __stream.write(__f);
                    __stream
                }
                None => Ptr::<CFile>::null(),
            }
        })
        .is_null())) as i32)
            != 0)
    );
    assert!(
        (((!(({
            let __buf = (buf.as_pointer() as Ptr<u8>).clone();
            let __n = (64usize as i32);
            if __n <= 0 {
                Ptr::<u8>::null()
            } else {
                let __max = (__n - 1) as usize;
                let mut __dst = __buf.clone();
                let mut __count: usize = 0;
                let __failed = libcc2rs::c_stdin().with_mut(|__f| {
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
                    Ptr::<u8>::null()
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
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"line1\n\0").to_c_string_iterator();
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
    let pipe: Value<Ptr<CFile>> = Rc::new(RefCell::new(libcc2rs::popen_refcount(
        Ptr::from_string_literal(b"cat\0").clone(),
        Ptr::from_string_literal(b"r\0").clone(),
    )));
    assert!((((!((*pipe.borrow()).is_null())) as i32) != 0));
    let n: Value<usize> = Rc::new(RefCell::new({
        let __a0 = ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone();
        let __a1 = 1_usize;
        let __a2 = (64usize as usize).wrapping_sub(1_usize);
        let __a3 = (*pipe.borrow()).clone();
        libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
    }));
    assert!((((libcc2rs::pclose_refcount((*pipe.borrow()).clone()) == 0) as i32) != 0));
    assert!(((((*n.borrow()) == 0_usize) as i32) != 0));
    (*fp.borrow_mut()) = match CFile::open(
        &(*path.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"r\0").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::<CFile>::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(((((*fp.borrow()).with_mut(|__f| __f.getc()) == ('l' as i32)) as i32) != 0));
    assert!(((((*fp.borrow()).with(|__f| __f.tell()) == 1_i64) as i32) != 0));
    assert!(
        (((match (*fp.borrow()).with_mut(|__v: &mut CFile| __v.seek(5_i64, ::libc::SEEK_CUR)) {
            -1 => -1,
            _ => 0,
        } == 0) as i32)
            != 0)
    );
    assert!(((((*fp.borrow()).with(|__f| __f.tell()) == 6_i64) as i32) != 0));
    assert!(((((*fp.borrow()).with_mut(|__f| __f.getc()) == ('l' as i32)) as i32) != 0));
    assert!(((((*fp.borrow()).with(|__f| __f.tell()) == 7_i64) as i32) != 0));
    libcc2rs::fclose_refcount((*fp.borrow()).clone());
    assert!((((libcc2rs::unlink_refcount((*path.borrow()).clone()) == 0) as i32) != 0));
    return 0;
}
