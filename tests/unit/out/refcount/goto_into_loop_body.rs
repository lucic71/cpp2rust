extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn scan_0(s: Ptr<u8>, start_inside: i32) -> i32 {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    let start_inside: Value<i32> = Rc::new(RefCell::new(start_inside));
    let depth: Value<i32> = <Value<i32>>::default();
    let seen: Value<i32> = <Value<i32>>::default();
    let i: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *depth.borrow_mut() = 0;
            *seen.borrow_mut() = 0;
            *i.borrow_mut() = 0;
            if !((*start_inside.borrow()) != 0) {
                goto!('__f0_join);
            }
        }
        '__f1_then: {
            goto!('inside);
        }
        '__f0_join: {}
        '__f2_cond: {
            if !(((*s.borrow()).offset(((*i.borrow()) as isize)).read()) != 0) {
                goto!('__f3_exit);
            }
        }
        '__f4_body: {
            if !((((((*s.borrow()).offset(((*i.borrow()) as isize)).read()) as i32) == ('(' as i32))
                as i32)
                != 0)
            {
                goto!('__f5_join);
            }
        }
        '__f6_then: {
            (*i.borrow_mut()).postfix_inc();
        }
        'inside: {
            (*depth.borrow_mut()).postfix_inc();
            (*seen.borrow_mut()).postfix_inc();
            if !((((*depth.borrow()) > 3) as i32) != 0) {
                goto!('__f7_join);
            }
        }
        '__f8_then: {
            goto!('__f3_exit);
        }
        '__f7_join: {
            goto!('__f2_cond);
        }
        '__f5_join: {
            (*i.borrow_mut()).postfix_inc();
            goto!('__f2_cond);
        }
        '__f3_exit: {
            return (((*depth.borrow()) * 10) + (*seen.borrow()));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ scan_0(Ptr::from_string_literal(b""), 0) }) == 0) as i32) != 0));
    assert!((((({ scan_0(Ptr::from_string_literal(b"(()"), 0) }) == 22) as i32) != 0));
    assert!((((({ scan_0(Ptr::from_string_literal(b"ab(cd"), 0) }) == 11) as i32) != 0));
    assert!((((({ scan_0(Ptr::from_string_literal(b""), 1) }) == 11) as i32) != 0));
    assert!((((({ scan_0(Ptr::from_string_literal(b"(((((("), 0) }) == 44) as i32) != 0));
    return 0;
}
