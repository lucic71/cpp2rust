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
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let in_: Value<Ptr<CFile>> = Rc::new(RefCell::new(libcc2rs::popen_refcount(
        Ptr::from_string_literal(b"echo hello\0").clone(),
        Ptr::from_string_literal(b"r\0").clone(),
    )));
    assert!((((!((*in_.borrow()).is_null())) as i32) != 0));
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
                let __failed = (*in_.borrow()).with_mut(|__f| {
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
    assert!((((libcc2rs::pclose_refcount((*in_.borrow()).clone()) == 0) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"hello\n\0").to_c_string_iterator();
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
    let out: Value<Ptr<CFile>> = Rc::new(RefCell::new(libcc2rs::popen_refcount(
        Ptr::from_string_literal(b"cat > /dev/null\0").clone(),
        Ptr::from_string_literal(b"w\0").clone(),
    )));
    assert!((((!((*out.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let __bytes: Vec<u8> = Ptr::from_string_literal(b"data\n\0")
                .to_c_string_iterator()
                .collect();
            match (*out.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
                true => 0,
                false => -1,
            }
        } >= 0) as i32)
            != 0)
    );
    assert!((((libcc2rs::pclose_refcount((*out.borrow()).clone()) == 0) as i32) != 0));
    print!("{}", (buf.as_pointer() as Ptr::<u8>));
    return 0;
}
