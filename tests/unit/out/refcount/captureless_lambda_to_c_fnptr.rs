extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 2, 8, 1, 3])));
    {
        let __base = ((arr.as_pointer() as Ptr<i32>) as Ptr<i32>)
            .to_any()
            .reinterpret_cast::<u8>();
        for __i in 0..5_usize {
            let mut __min = __i;
            for __j in (__i + 1)..5_usize {
                if (|a: AnyPtr, b: AnyPtr| {
                    let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
                    let b: Value<AnyPtr> = Rc::new(RefCell::new(b));
                    return {
                        let _lhs = ((*b.borrow()).reinterpret_cast::<i32>().read());
                        _lhs - ((*a.borrow()).reinterpret_cast::<i32>().read())
                    };
                })
                .call(
                    __base.offset(__j * ::std::mem::size_of::<i32>()).to_any(),
                    __base.offset(__min * ::std::mem::size_of::<i32>()).to_any(),
                ) < 0
                {
                    __min = __j;
                }
            }
            if __min != __i {
                for __b in 0..::std::mem::size_of::<i32>() {
                    let __x = __base
                        .offset(__i * ::std::mem::size_of::<i32>() + __b)
                        .read();
                    let __y = __base
                        .offset(__min * ::std::mem::size_of::<i32>() + __b)
                        .read();
                    __base
                        .offset(__i * ::std::mem::size_of::<i32>() + __b)
                        .write(__y);
                    __base
                        .offset(__min * ::std::mem::size_of::<i32>() + __b)
                        .write(__x);
                }
            }
        }
    };
    assert!(((*arr.borrow())[(0) as usize] == 8));
    return 0;
}
