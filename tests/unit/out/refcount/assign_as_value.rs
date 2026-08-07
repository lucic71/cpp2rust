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
        (0..2).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new((buf.as_pointer() as Ptr<u8>)));
    let q: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    (*q.borrow_mut()) = {
        (*p.borrow_mut()) += 1;
        (*p.borrow()).clone()
    };
    assert!(
        ((({
            let _lhs = (*q.borrow()).clone();
            _lhs == (buf.as_pointer() as Ptr<u8>).offset(((1) as isize))
        }) as i32)
            != 0)
    );
    let src: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as u8),
        (('b' as i32) as u8),
    ])));
    let dst: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('x' as i32) as u8),
        (('y' as i32) as u8),
    ])));
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new((src.as_pointer() as Ptr<u8>)));
    let d: Value<Ptr<u8>> = Rc::new(RefCell::new((dst.as_pointer() as Ptr<u8>)));
    let last: Value<u8> = Rc::new(RefCell::new({
        let __rhs = ((*s.borrow_mut()).postfix_inc().read());
        (*d.borrow_mut()).postfix_inc().write(__rhs);
        __rhs
    }));
    assert!((((((*last.borrow()) as i32) == ('a' as i32)) as i32) != 0));
    assert!(
        (((((({
            let _lhs = (*d.borrow()).clone();
            _lhs == (dst.as_pointer() as Ptr<u8>).offset(((1) as isize))
        }) as i32)
            != 0)
            && ((({
                let _lhs = (*s.borrow()).clone();
                _lhs == (src.as_pointer() as Ptr<u8>).offset(((1) as isize))
            }) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((((*dst.borrow())[(0) as usize] as i32) == ('a' as i32)) as i32) != 0)
            && (((((*dst.borrow())[(1) as usize] as i32) == ('y' as i32)) as i32) != 0))
            as i32)
            != 0)
    );
    let out: Value<u8> = <Value<u8>>::default();
    'switch: {
        let __match_cond = (({
            (*out.borrow_mut()) = (('x' as i32) as u8);
            (*out.borrow())
        }) as i32);
        match __match_cond {
            __v if __v == ('x' as i32) => {
                assert!((1 != 0));
                break 'switch;
            }
            _ => {
                assert!((0 != 0));
                break 'switch;
            }
        }
    };
    assert!((((((*out.borrow()) as i32) == ('x' as i32)) as i32) != 0));
    return 0;
}
