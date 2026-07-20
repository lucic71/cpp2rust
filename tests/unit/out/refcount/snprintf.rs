extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..32).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        (((({
            let __s = format!("x={} y={}", -3_i32, 7_u32);
            let __b = __s.as_bytes();
            let __size = (::std::mem::size_of::<[u8; 32]>()) as usize;
            if __size > 0 {
                let __n = ::std::cmp::min(__b.len(), __size - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        }) == 8) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"x=-3 y=7").to_c_string_iterator();
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
        (((({
            let __s = format!("{}", Ptr::from_string_literal(b"hello"));
            let __b = __s.as_bytes();
            let __size = (4_usize) as usize;
            if __size > 0 {
                let __n = ::std::cmp::min(__b.len(), __size - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        }) == 5) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"hel").to_c_string_iterator();
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
        (((({
            let __s = format!("{:05}|{:02x}", 42, 255);
            let __b = __s.as_bytes();
            let __size = (::std::mem::size_of::<[u8; 32]>()) as usize;
            if __size > 0 {
                let __n = ::std::cmp::min(__b.len(), __size - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        }) == 8) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"00042|ff").to_c_string_iterator();
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
        (((({
            let __s = format!("{}%", 65 as u8 as char);
            let __b = __s.as_bytes();
            let __size = (::std::mem::size_of::<[u8; 32]>()) as usize;
            if __size > 0 {
                let __n = ::std::cmp::min(__b.len(), __size - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        }) == 2) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"A%").to_c_string_iterator();
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
        (((({
            let __s = format!("{} {} {}", -1_i64, 1_u64, (9 as usize));
            let __b = __s.as_bytes();
            let __size = (::std::mem::size_of::<[u8; 32]>()) as usize;
            if __size > 0 {
                let __n = ::std::cmp::min(__b.len(), __size - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        }) == 6) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"-1 1 9").to_c_string_iterator();
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
    (*buf.borrow_mut())[(0) as usize] = (('Z' as i32) as u8);
    assert!(
        (((({
            let __s = format!("{}", 123);
            let __b = __s.as_bytes();
            let __size = (0_usize) as usize;
            if __size > 0 {
                let __n = ::std::cmp::min(__b.len(), __size - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        }) == 3) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(0) as usize] as i32) == ('Z' as i32)) as i32) != 0));
    return 0;
}
