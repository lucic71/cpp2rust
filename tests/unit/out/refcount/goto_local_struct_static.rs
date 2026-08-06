extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn lookup_0(c: u8, fallback: i32) -> i32 {
    let c: Value<u8> = Rc::new(RefCell::new(c));
    let fallback: Value<i32> = Rc::new(RefCell::new(fallback));
    #[repr(C)]
    #[derive(Clone, Default)]
    pub struct Choice {
        pub key: u8,
        pub op: i32,
    }
    impl ByteRepr for Choice {
        fn byte_size() -> usize {
            8
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            self.key.to_bytes(&mut buf[0..1]);
            self.op.to_bytes(&mut buf[4..8]);
        }
        fn from_bytes(buf: &[u8]) -> Self {
            Self {
                key: <u8>::from_bytes(&buf[0..1]),
                op: <i32>::from_bytes(&buf[4..8]),
            }
        }
    }
    thread_local!(
        static aChoice_1: Value<Box<[Choice]>> = Rc::new(RefCell::new(Box::new([
            Choice {
                key: (('a' as i32) as u8),
                op: 11,
            },
            Choice {
                key: (('b' as i32) as u8),
                op: 22,
            },
        ])));
    );
    let i: Value<i32> = <Value<i32>>::default();
    let r: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *r.borrow_mut() = (*fallback.borrow());
            (*i.borrow_mut()) = 0;
            'loop_: while ((((*i.borrow()) < 2) as i32) != 0) {
                if (((((*c.borrow()) as i32)
                    == ((*aChoice_1.with(Value::clone).borrow())[(*i.borrow()) as usize].key
                        as i32)) as i32)
                    != 0)
                {
                    (*r.borrow_mut()) =
                        (*aChoice_1.with(Value::clone).borrow())[(*i.borrow()) as usize].op;
                    goto!('done);
                }
                (*i.borrow_mut()).postfix_inc();
            }
        }
        'done: {
            return (*r.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn classify_2(mode: i32, v: i32) -> i32 {
    let mode: Value<i32> = Rc::new(RefCell::new(mode));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    #[repr(C)]
    #[derive(Clone, Default)]
    pub struct Weight {
        pub lo: i32,
        pub hi: i32,
    }
    impl ByteRepr for Weight {
        fn byte_size() -> usize {
            8
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            self.lo.to_bytes(&mut buf[0..4]);
            self.hi.to_bytes(&mut buf[4..8]);
        }
        fn from_bytes(buf: &[u8]) -> Self {
            Self {
                lo: <i32>::from_bytes(&buf[0..4]),
                hi: <i32>::from_bytes(&buf[4..8]),
            }
        }
    }
    thread_local!(
        static aWeight_3: Value<Box<[Weight]>> = Rc::new(RefCell::new(Box::new([
            Weight { lo: 1, hi: 2 },
            Weight { lo: 3, hi: 4 },
        ])));
    );
    let r: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *r.borrow_mut() = 0;
            if !((((*v.borrow()) > 0) as i32) != 0) {
                goto!('__f1_else);
            }
        }
        '__f2_then: {}
        'positive: {
            (*r.borrow_mut()) =
                ((*aWeight_3.with(Value::clone).borrow())[(0) as usize].lo + (*v.borrow()));
            if !((((*mode.borrow()) == 1) as i32) != 0) {
                goto!('__f3_join);
            }
        }
        '__f4_then: {
            goto!('negative);
        }
        '__f3_join: {
            goto!('__f0_join);
        }
        '__f1_else: {
            if !((((*mode.borrow()) == 2) as i32) != 0) {
                goto!('__f5_join);
            }
        }
        '__f6_then: {
            goto!('positive);
        }
        '__f5_join: {}
        'negative: {
            (*r.borrow_mut()) =
                ((*aWeight_3.with(Value::clone).borrow())[(1) as usize].hi - (*v.borrow()));
        }
        '__f0_join: {
            return (*r.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ lookup_0((('a' as i32) as u8), -1_i32) }) == 11) as i32) != 0));
    assert!((((({ lookup_0((('b' as i32) as u8), -1_i32) }) == 22) as i32) != 0));
    assert!((((({ lookup_0((('z' as i32) as u8), -1_i32) }) == -1_i32) as i32) != 0));
    assert!((((({ classify_2(0, 5) }) == 6) as i32) != 0));
    assert!((((({ classify_2(1, 5) }) == -1_i32) as i32) != 0));
    assert!((((({ classify_2(0, -3_i32) }) == 7) as i32) != 0));
    assert!((((({ classify_2(2, -3_i32) }) == -2_i32) as i32) != 0));
    return 0;
}
