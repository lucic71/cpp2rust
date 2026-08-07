extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn dispatch_0(kind: i32, v: i32) -> i32 {
    let kind: Value<i32> = Rc::new(RefCell::new(kind));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let acc: Value<i32> = <Value<i32>>::default();
    let scaled: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *acc.borrow_mut() = 0;
            *scaled.borrow_mut() = 0;
            if !((((*v.borrow()) < 0) as i32) != 0) {
                goto!('__f0_join);
            }
        }
        '__f1_then: {
            {
                let __rhs = -(*v.borrow());
                (*v.borrow_mut()) = __rhs
            };
            goto!('negative_entry);
        }
        '__f0_join: {
            match (*kind.borrow()) {
                __v if __v == 1 => {
                    goto!('__f3_case);
                }
                __v if __v == 2 => {
                    goto!('__f4_case);
                }
                _ => {
                    goto!('__default_1);
                }
            }
        }
        '__f3_case: {
            (*acc.borrow_mut()) = ((*v.borrow()) + 1);
            goto!('__f2_swexit);
        }
        '__f4_case: {
            (*scaled.borrow_mut()) = ((*v.borrow()) * 2);
        }
        'negative_entry: {
            (*acc.borrow_mut()) = ((*scaled.borrow()) + (*v.borrow()));
            goto!('__f2_swexit);
        }
        '__default_1: {
            (*acc.borrow_mut()) = 999;
            goto!('__f2_swexit);
        }
        '__f2_swexit: {
            return (*acc.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn step_1(p: Ptr<u8>) -> i32 {
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(p));
    let op: Value<i32> = <Value<i32>>::default();
    let acc: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *op.borrow_mut() = 0;
            *acc.borrow_mut() = 0;
            if !((((((*p.borrow()).read()) as i32) == ('!' as i32)) as i32) != 0) {
                goto!('__f0_join);
            }
        }
        '__f1_then: {
            (*p.borrow_mut()).postfix_inc();
            goto!('forced);
        }
        '__f0_join: {
            match {
                (*op.borrow_mut()) = (((*p.borrow_mut()).postfix_inc().read()) as i32);
                (*op.borrow())
            } {
                __v if __v == ('a' as i32) => {
                    goto!('__f3_case);
                }
                __v if __v == ('b' as i32) => {
                    goto!('__f4_case);
                }
                _ => {
                    goto!('__default_3);
                }
            }
        }
        '__f3_case: {
            (*acc.borrow_mut()) = 1;
            goto!('__f2_swexit);
        }
        '__f4_case: {
            (*acc.borrow_mut()) = 2;
        }
        'forced: {
            (*acc.borrow_mut()) += 10;
            goto!('__f2_swexit);
        }
        '__default_3: {
            (*acc.borrow_mut()) = 100;
            goto!('__f2_swexit);
        }
        '__f2_swexit: {
            return ((*acc.borrow()) + (*op.borrow()));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!((((({ step_1(Ptr::from_string_literal(b"a\0")) }) == (1 + ('a' as i32))) as i32) != 0));
    assert!(
        (((({ step_1(Ptr::from_string_literal(b"b\0")) }) == (12 + ('b' as i32))) as i32) != 0)
    );
    assert!(
        (((({ step_1(Ptr::from_string_literal(b"z\0")) }) == (100 + ('z' as i32))) as i32) != 0)
    );
    assert!((((({ step_1(Ptr::from_string_literal(b"!x\0")) }) == 10) as i32) != 0));
    assert!((((({ dispatch_0(1, 5) }) == 6) as i32) != 0));
    assert!((((({ dispatch_0(2, 5) }) == 15) as i32) != 0));
    assert!((((({ dispatch_0(7, 5) }) == 999) as i32) != 0));
    assert!((((({ dispatch_0(7, -5_i32) }) == 5) as i32) != 0));
    assert!((((({ dispatch_0(1, -3_i32) }) == 3) as i32) != 0));
    return 0;
}
