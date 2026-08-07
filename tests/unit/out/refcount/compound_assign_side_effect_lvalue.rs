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
        (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, 8usize as usize);
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    };
    (*buf.borrow_mut())[(0) as usize] = 1_u8;
    (*buf.borrow_mut())[(1) as usize] = 2_u8;
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new((buf.as_pointer() as Ptr<u8>)));
    {
        let __lhs = (*p.borrow_mut()).postfix_inc();
        let rhs_0 = ((((__lhs.read()) as i32) | 128) as u8);
        __lhs.write(rhs_0)
    };
    assert!(
        ((({
            let _lhs = (*p.borrow()).clone();
            _lhs == (buf.as_pointer() as Ptr<u8>).offset(((1) as isize))
        }) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(0) as usize] as i32) == 129) as i32) != 0));
    assert!((((((*buf.borrow())[(1) as usize] as i32) == 2) as i32) != 0));
    let r: Value<Ptr<u8>> = Rc::new(RefCell::new((buf.as_pointer() as Ptr<u8>)));
    {
        let __lhs = (*r.borrow_mut()).prefix_inc();
        let rhs_0 = ((((__lhs.read()) as i32) | 16) as u8);
        __lhs.write(rhs_0)
    };
    assert!(
        ((({
            let _lhs = (*r.borrow()).clone();
            _lhs == (buf.as_pointer() as Ptr<u8>).offset(((1) as isize))
        }) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(1) as usize] as i32) == 18) as i32) != 0));
    let words: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32, 4_u32])));
    let w: Value<Ptr<u32>> = Rc::new(RefCell::new((words.as_pointer() as Ptr<u32>)));
    {
        let __lhs = (*w.borrow_mut()).postfix_inc();
        let rhs_0 = __lhs.with(|__v| (*__v).wrapping_add((10_u32 as u32)).clone());
        __lhs.write(rhs_0)
    };
    assert!(
        ((({
            let _lhs = (*w.borrow()).clone();
            _lhs == (words.as_pointer() as Ptr<u32>).offset(((1) as isize))
        }) as i32)
            != 0)
    );
    assert!(((((*words.borrow())[(0) as usize] == 11_u32) as i32) != 0));
    assert!(((((*words.borrow())[(1) as usize] == 2_u32) as i32) != 0));
    let ptrs: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| Ptr::<u8>::null())
            .collect::<Box<[Ptr<u8>]>>(),
    ));
    (*ptrs.borrow_mut())[(0) as usize] = (buf.as_pointer() as Ptr<u8>);
    (*ptrs.borrow_mut())[(1) as usize] = (buf.as_pointer() as Ptr<u8>);
    let pp: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new((ptrs.as_pointer() as Ptr<Ptr<u8>>)));
    {
        let _ptr = (*pp.borrow_mut()).postfix_inc().clone();
        {
            let __rhs = (_ptr.read()) + 3;
            _ptr.write(__rhs)
        }
    };
    assert!(
        ((({
            let _lhs = (*pp.borrow()).clone();
            _lhs == (ptrs.as_pointer() as Ptr<Ptr<u8>>).offset(((1) as isize))
        }) as i32)
            != 0)
    );
    assert!(
        ((((*ptrs.borrow())[(0) as usize] == (buf.as_pointer() as Ptr::<u8>).offset(((3) as isize)))
            as i32)
            != 0)
    );
    assert!(((((*ptrs.borrow())[(1) as usize] == (buf.as_pointer() as Ptr::<u8>)) as i32) != 0));
    let q: Value<Ptr<u8>> = Rc::new(RefCell::new((buf.as_pointer() as Ptr<u8>)));
    let v: Value<i32> = Rc::new(RefCell::new(
        (({
            let __lhs = (*q.borrow_mut()).postfix_inc();
            let rhs_0 = ((((__lhs.read()) as i32) | 64) as u8);
            __lhs.write(rhs_0);
            (__lhs.read())
        }) as i32),
    ));
    assert!(
        ((({
            let _lhs = (*q.borrow()).clone();
            _lhs == (buf.as_pointer() as Ptr<u8>).offset(((1) as isize))
        }) as i32)
            != 0)
    );
    assert!(((((*v.borrow()) == 193) as i32) != 0));
    assert!((((((*buf.borrow())[(0) as usize] as i32) == 193) as i32) != 0));
    return 0;
}
