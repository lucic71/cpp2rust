extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmp_0(a: AnyPtr, b: AnyPtr) -> i32 {
    let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
    let b: Value<AnyPtr> = Rc::new(RefCell::new(b));
    return {
        let mut __it1 = ((*a.borrow()).reinterpret_cast::<Ptr<u8>>().read()).to_c_string_iterator();
        let mut __it2 = ((*b.borrow()).reinterpret_cast::<Ptr<u8>>().read()).to_c_string_iterator();
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
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let items: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::from_string_literal(b"pear"),
        Ptr::from_string_literal(b"apple"),
        Ptr::from_string_literal(b"fig"),
        Ptr::from_string_literal(b"date"),
    ])));
    {
        let __base = (items.as_pointer() as Ptr<Ptr<u8>>)
            .to_any()
            .reinterpret_cast::<u8>();
        let __size = 8usize;
        let mut __x = vec![0u8; __size];
        let mut __y = vec![0u8; __size];
        for __i in 0..4_usize {
            let mut __min = __i;
            for __j in (__i + 1)..4_usize {
                if FnPtr::<fn(AnyPtr, AnyPtr) -> i32>::new(cmp_0)(
                    __base.offset(__j * __size).to_any(),
                    __base.offset(__min * __size).to_any(),
                ) < 0
                {
                    __min = __j;
                }
            }
            if __min != __i {
                __base
                    .offset(__i * __size)
                    .with_slice(__size, |__s| __x.copy_from_slice(__s));
                __base
                    .offset(__min * __size)
                    .with_slice(__size, |__s| __y.copy_from_slice(__s));
                __base
                    .offset(__i * __size)
                    .with_slice_mut(__size, |__d| __d.copy_from_slice(&__y));
                __base
                    .offset(__min * __size)
                    .with_slice_mut(__size, |__d| __d.copy_from_slice(&__x));
            }
        }
    };
    assert!(
        ((({
            let mut __it1 = (*items.borrow())[(0) as usize].to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"apple").to_c_string_iterator();
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
            let mut __it1 = (*items.borrow())[(1) as usize].to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"date").to_c_string_iterator();
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
            let mut __it1 = (*items.borrow())[(2) as usize].to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"fig").to_c_string_iterator();
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
            let mut __it1 = (*items.borrow())[(3) as usize].to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"pear").to_c_string_iterator();
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
    return 0;
}
