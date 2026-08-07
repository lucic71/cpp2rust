extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static buf_0: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..32).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
);
thread_local!(
    pub static n_1: Value<i32> = <Value<i32>>::default();
);
pub fn emit_2(ch: u8) {
    let ch: Value<u8> = Rc::new(RefCell::new(ch));
    (*buf_0.with(Value::clone).borrow_mut())
        [((*n_1.with(Value::clone).borrow_mut()).postfix_inc()) as usize] = (*ch.borrow());
}
pub fn step_3(c: i32, last: Ptr<i32>) {
    let c: Value<i32> = Rc::new(RefCell::new(c));
    let last: Value<Ptr<i32>> = Rc::new(RefCell::new(last));
    goto_block!({
        '__entry: {
            match (*c.borrow()) {
                __v if __v == (')' as i32) => {
                    goto!('__f1_case);
                }
                __v if __v == ('(' as i32) => {
                    goto!('__f2_case);
                }
                __v if __v == ('.' as i32) => {
                    goto!('COPY);
                }
                __v if __v == ('^' as i32) => {
                    goto!('__f3_case);
                }
                _ => {
                    goto!('__default_1);
                }
            }
        }
        '__f1_case: {
            if !(((((*last.borrow()).read()) == 0) as i32) != 0) {
                goto!('__f4_join);
            }
        }
        '__f5_then: {
            goto!('ESCAPE);
        }
        '__f4_join: {
            goto!('COPY);
        }
        '__f2_case: {
            (*last.borrow()).write(('(' as i32));
        }
        'COPY: {
            ({ emit_2(((*c.borrow()) as u8)) });
            {
                let __rhs = (*c.borrow());
                (*last.borrow()).write(__rhs)
            };
            goto!('__f0_swexit);
        }
        '__f3_case: {
            if !(((((*last.borrow()).read()) == ('(' as i32)) as i32) != 0) {
                goto!('__f6_join);
            }
        }
        '__f7_then: {
            goto!('COPY);
        }
        '__f6_join: {}
        '__default_1: {
            if !(((((((*c.borrow()) == ('x' as i32)) as i32) != 0)
                || ((((*c.borrow()) == ('y' as i32)) as i32) != 0)) as i32)
                != 0)
            {
                goto!('__f8_join);
            }
        }
        '__f9_then: {}
        'ESCAPE: {
            ({ emit_2((('\\' as i32) as u8)) });
        }
        '__f8_join: {
            ({ emit_2(((*c.borrow()) as u8)) });
            (*last.borrow()).write(255);
            goto!('__f0_swexit);
        }
        '__f0_swexit: {}
    });
}
pub fn convert_4(s: Ptr<u8>) -> Ptr<u8> {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    let last: Value<i32> = Rc::new(RefCell::new(0));
    (*n_1.with(Value::clone).borrow_mut()) = 0;
    'loop_: while (((*s.borrow()).read()) != 0) {
        ({
            step_3(
                (((*s.borrow_mut()).postfix_inc().read()) as i32),
                (last.as_pointer()),
            )
        });
    }
    (*buf_0.with(Value::clone).borrow_mut())[(*n_1.with(Value::clone).borrow()) as usize] = 0_u8;
    return (buf_0.with(Value::clone).as_pointer() as Ptr<u8>);
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!(
        ((({
            let mut __it1 =
                ({ convert_4(Ptr::from_string_literal(b")a\0")) }).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"\\)a\0").to_c_string_iterator();
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
            let mut __it1 =
                ({ convert_4(Ptr::from_string_literal(b"(.x\0")) }).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"(.\\x\0").to_c_string_iterator();
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
            let mut __it1 =
                ({ convert_4(Ptr::from_string_literal(b"(^\0")) }).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"(^\0").to_c_string_iterator();
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
            let mut __it1 =
                ({ convert_4(Ptr::from_string_literal(b"a^\0")) }).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"a^\0").to_c_string_iterator();
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
            let mut __it1 =
                ({ convert_4(Ptr::from_string_literal(b"()\0")) }).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"()\0").to_c_string_iterator();
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
            let mut __it1 =
                ({ convert_4(Ptr::from_string_literal(b"^x\0")) }).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"^\\x\0").to_c_string_iterator();
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
            let mut __it1 =
                ({ convert_4(Ptr::from_string_literal(b")(\0")) }).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"\\)(\0").to_c_string_iterator();
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
