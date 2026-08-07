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
        (0..32).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"x=%d y=%u\0").to_rust_string(),
                &[(-3_i32).into(), (7_u32).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 8) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"x=-3 y=7\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%s\0").to_rust_string(),
                &[(Ptr::from_string_literal(b"hello\0")).into()],
            );
            let __b = __s.as_bytes();
            if 4_usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 4_usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 5) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"hel\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%05d|%x|%X\0").to_rust_string(),
                &[(42).into(), (255).into(), (255).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 11) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"00042|ff|FF\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%.2f\0").to_rust_string(),
                &[(3.14159E+0).into()],
            );
            let __b = __s.as_bytes();
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
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"3.14\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%-6s|\0").to_rust_string(),
                &[(Ptr::from_string_literal(b"ab\0")).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 7) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"ab    |\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%c%%\0").to_rust_string(),
                &[(65).into()],
            );
            let __b = __s.as_bytes();
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
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"A%\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%+d % d\0").to_rust_string(),
                &[(5).into(), (5).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 5) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"+5  5\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%ld %lu %zu\0").to_rust_string(),
                &[(-1_i64).into(), (1_u64).into(), (9_usize).into()],
            );
            let __b = __s.as_bytes();
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
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"-1 1 9\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%e\0").to_rust_string(),
                &[(1.2345678E+3).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 12) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"1.234568e+03\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%g\0").to_rust_string(),
                &[(1.234567E+6).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 11) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"1.23457e+06\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%d\0").to_rust_string(),
                &[(123).into()],
            );
            let __b = __s.as_bytes();
            if 0_usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 0_usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 3) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(0) as usize] as i32) == ('Z' as i32)) as i32) != 0));
    let fmt: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    (*fmt.borrow_mut())[(0) as usize] = (('%' as i32) as u8);
    (*fmt.borrow_mut())[(1) as usize] = (('5' as i32) as u8);
    (*fmt.borrow_mut())[(2) as usize] = (('.' as i32) as u8);
    (*fmt.borrow_mut())[(3) as usize] = (('1' as i32) as u8);
    (*fmt.borrow_mut())[(4) as usize] = (('f' as i32) as u8);
    (*fmt.borrow_mut())[(5) as usize] = 0_u8;
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &(fmt.as_pointer() as Ptr<u8>).to_rust_string(),
                &[(3.26E+0).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 5) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"  3.3\0").to_c_string_iterator();
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
    let segment: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"abcdef\0\0")));
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"<%.*s>\0").to_rust_string(),
                &[(3).into(), (segment.as_pointer() as Ptr<u8>).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 5) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"<abc>\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%.*s\0").to_rust_string(),
                &[(10).into(), (segment.as_pointer() as Ptr<u8>).into()],
            );
            let __b = __s.as_bytes();
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
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"abcdef\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%.*s\0").to_rust_string(),
                &[(-1_i32).into(), (segment.as_pointer() as Ptr<u8>).into()],
            );
            let __b = __s.as_bytes();
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
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"abcdef\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%*d|%-*d|\0").to_rust_string(),
                &[(5).into(), (42).into(), (5).into(), (42).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 12) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"   42|42   |\0").to_c_string_iterator();
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
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%*d\0").to_rust_string(),
                &[(-5_i32).into(), (42).into()],
            );
            let __b = __s.as_bytes();
            if 32usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 32usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 5) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"42   \0").to_c_string_iterator();
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
    let first: Value<i32> = Rc::new(RefCell::new(0));
    let second: Value<i32> = Rc::new(RefCell::new(0));
    let third: Value<i32> = Rc::new(RefCell::new(0));
    assert!(
        (((libcc2rs::scan_c(
            &Ptr::from_string_literal(b"40,25,3\0").to_rust_string(),
            &Ptr::from_string_literal(b"%d,%d,%d\0").to_rust_string(),
            &[
                (first.as_pointer()).into(),
                (second.as_pointer()).into(),
                (third.as_pointer()).into(),
            ]
        ) == 3) as i32)
            != 0)
    );
    assert!(
        ((((((((((*first.borrow()) == 40) as i32) != 0)
            && ((((*second.borrow()) == 25) as i32) != 0)) as i32)
            != 0)
            && ((((*third.borrow()) == 3) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        (((libcc2rs::scan_c(
            &Ptr::from_string_literal(b"7,8\0").to_rust_string(),
            &Ptr::from_string_literal(b"%d,%d,%d\0").to_rust_string(),
            &[
                (first.as_pointer()).into(),
                (second.as_pointer()).into(),
                (third.as_pointer()).into(),
            ]
        ) == 2) as i32)
            != 0)
    );
    assert!(
        (((((((*first.borrow()) == 7) as i32) != 0) && ((((*second.borrow()) == 8) as i32) != 0))
            as i32)
            != 0)
    );
    assert!(
        (((libcc2rs::scan_c(
            &Ptr::from_string_literal(b"junk\0").to_rust_string(),
            &Ptr::from_string_literal(b"%d,%d,%d\0").to_rust_string(),
            &[
                (first.as_pointer()).into(),
                (second.as_pointer()).into(),
                (third.as_pointer()).into(),
            ]
        ) == 0) as i32)
            != 0)
    );
    let hex: Value<i32> = Rc::new(RefCell::new(0));
    let word: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let ch: Value<u8> = Rc::new(RefCell::new(0_u8));
    assert!(
        (((libcc2rs::scan_c(
            &Ptr::from_string_literal(b"  ff word x\0").to_rust_string(),
            &Ptr::from_string_literal(b"%x %7s %c\0").to_rust_string(),
            &[
                (hex.as_pointer()).into(),
                (word.as_pointer() as Ptr::<u8>).into(),
                (ch.as_pointer()).into(),
            ]
        ) == 3) as i32)
            != 0)
    );
    assert!(((((*hex.borrow()) == 255) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (word.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"word\0").to_c_string_iterator();
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
    assert!((((((*ch.borrow()) as i32) == ('x' as i32)) as i32) != 0));
    let big: Value<i64> = Rc::new(RefCell::new(0_i64));
    let small: Value<i16> = Rc::new(RefCell::new(0_i16));
    assert!(
        (((libcc2rs::scan_c(
            &Ptr::from_string_literal(b"123456789012 -7\0").to_rust_string(),
            &Ptr::from_string_literal(b"%ld %hd\0").to_rust_string(),
            &[(big.as_pointer()).into(), (small.as_pointer()).into(),]
        ) == 2) as i32)
            != 0)
    );
    assert!(((((*big.borrow()) == 123456789012_i64) as i32) != 0));
    assert!((((((*small.borrow()) as i32) == -7_i32) as i32) != 0));
    return 0;
}
