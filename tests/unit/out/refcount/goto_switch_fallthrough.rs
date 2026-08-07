extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sm_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ret: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *ret.borrow_mut() = 0;
            switch!(match (*n.borrow()) {
                __v if __v == 0 => {
                    (*ret.borrow_mut()) += 1;
                }
                __v if __v == 1 => {
                    (*ret.borrow_mut()) += 10;
                    goto!('out);
                }
                _ => {
                    (*ret.borrow_mut()) += 100;
                    break;
                }
            });
            (*ret.borrow_mut()) += 1000;
        }
        'out: {
            return (*ret.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn scan_1(p: Ptr<u8>) -> i32 {
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(p));
    let c: Value<i32> = <Value<i32>>::default();
    let ret: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *c.borrow_mut() = 0;
            *ret.borrow_mut() = 0;
            switch!(match {
                (*c.borrow_mut()) = (((*p.borrow_mut()).postfix_inc().read()) as i32);
                (*c.borrow())
            } {
                __v if __v == ('a' as i32) => {
                    (*ret.borrow_mut()) = 1;
                }
                __v if __v == ('b' as i32) => {
                    (*ret.borrow_mut()) += 10;
                    goto!('out);
                }
                _ => {
                    (*ret.borrow_mut()) = 100;
                    break;
                }
            });
            (*ret.borrow_mut()) += 1000;
        }
        'out: {
            return ((*ret.borrow()) + (*c.borrow()));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ sm_0(0) }) == 11) as i32) != 0));
    assert!((((({ sm_0(1) }) == 10) as i32) != 0));
    assert!((((({ sm_0(9) }) == 1100) as i32) != 0));
    assert!(
        (((({ scan_1(Ptr::from_string_literal(b"a\0")) }) == (11 + ('a' as i32))) as i32) != 0)
    );
    assert!(
        (((({ scan_1(Ptr::from_string_literal(b"b\0")) }) == (10 + ('b' as i32))) as i32) != 0)
    );
    assert!(
        (((({ scan_1(Ptr::from_string_literal(b"z\0")) }) == (1100 + ('z' as i32))) as i32) != 0)
    );
    return 0;
}
