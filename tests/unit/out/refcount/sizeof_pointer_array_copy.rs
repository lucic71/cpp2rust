extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static names_0: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::from_string_literal(b"alpha\0"),
        Ptr::from_string_literal(b"beta\0"),
        Ptr::from_string_literal(b"gamma\0"),
        Ptr::<u8>::null(),
    ])));
);
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let count: Value<usize> = Rc::new(RefCell::new(0_usize));
    let walk: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(Ptr::<Ptr<u8>>::null()));
    (*walk.borrow_mut()) = (names_0.with(Value::clone).as_pointer() as Ptr<Ptr<u8>>);
    'loop_: while !((*walk.borrow()).read()).is_null() {
        (*count.borrow_mut()).prefix_inc();
        (*walk.borrow_mut()).postfix_inc();
    }
    assert!(((((*count.borrow()) == 3_usize) as i32) != 0));
    assert!((((32usize == (8usize as usize).wrapping_mul(4_usize)) as i32) != 0));
    let copy: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(
            ((8usize as u64).wrapping_mul((((*count.borrow()).wrapping_add(1_usize)) as u64))
                as usize),
        )
        .reinterpret_cast::<Ptr<u8>>(),
    ));
    assert!((((!((*copy.borrow()).is_null())) as i32) != 0));
    {
        (*copy.borrow()).clone().to_any().memcpy(
            &((names_0.with(Value::clone).as_pointer() as Ptr<Ptr<u8>>) as Ptr<Ptr<u8>>).to_any(),
            ((8usize as u64).wrapping_mul(((*count.borrow()) as u64)) as usize) as usize,
        );
        (*copy.borrow()).clone().to_any().clone()
    };
    (*copy.borrow())
        .offset(((*count.borrow()) as isize))
        .write(Ptr::<u8>::null());
    assert!(
        ((({
            let mut __it1 = ((*copy.borrow()).offset(((0) as isize)).read()).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"alpha\0").to_c_string_iterator();
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
            let mut __it1 = ((*copy.borrow()).offset(((1) as isize)).read()).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"beta\0").to_c_string_iterator();
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
            let mut __it1 = ((*copy.borrow()).offset(((2) as isize)).read()).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"gamma\0").to_c_string_iterator();
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
    assert!((((((*copy.borrow()).offset(((3) as isize)).read()).is_null()) as i32) != 0));
    (*count.borrow_mut()) = 0_usize;
    (*walk.borrow_mut()) = (*copy.borrow()).clone();
    'loop_: while !((*walk.borrow()).read()).is_null() {
        (*count.borrow_mut()).prefix_inc();
        (*walk.borrow_mut()).postfix_inc();
    }
    assert!(((((*count.borrow()) == 3_usize) as i32) != 0));
    libcc2rs::free_refcount(((*copy.borrow()).clone() as Ptr<Ptr<u8>>).to_any().clone());
    return 0;
}
