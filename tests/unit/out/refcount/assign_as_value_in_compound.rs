extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Clone, Default)]
pub struct item {
    pub flags: u8,
}
impl ByteRepr for item {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.flags.to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            flags: <u8>::from_bytes(&buf[0..1]),
        }
    }
}
pub fn merge_0(a: Ptr<item>, n: i32) -> u8 {
    let a: Value<Ptr<item>> = Rc::new(RefCell::new(a));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let all: Value<u8> = Rc::new(RefCell::new(0_u8));
    let i: Value<i32> = <Value<i32>>::default();
    (*i.borrow_mut()) = ((*n.borrow()) - 1);
    'loop_: while ((((*i.borrow()) > 0) as i32) != 0) {
        {
            let rhs_0 = ((((*all.borrow()) as i32)
                | (({
                    let __rhs = (*a.borrow())
                        .offset((((*i.borrow()) - 1) as isize))
                        .with(|__v| __v.flags);
                    (*a.borrow())
                        .offset(((*i.borrow()) as isize))
                        .with_mut(|__v| __v.flags = __rhs);
                    (*a.borrow())
                        .offset(((*i.borrow()) as isize))
                        .with(|__v| __v.flags)
                }) as i32)) as u8);
            (*all.borrow_mut()) = rhs_0
        };
        (*i.borrow_mut()).postfix_dec();
    }
    return (*all.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let a: Value<Box<[item]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <item>::default()).collect::<Box<[item]>>(),
    ));
    (*a.borrow_mut())[(0) as usize].flags = 1_u8;
    (*a.borrow_mut())[(1) as usize].flags = 2_u8;
    (*a.borrow_mut())[(2) as usize].flags = 4_u8;
    assert!(((((({ merge_0((a.as_pointer() as Ptr<item>), 3) }) as i32) == 3) as i32) != 0));
    assert!((((((*a.borrow())[(0) as usize].flags as i32) == 1) as i32) != 0));
    assert!((((((*a.borrow())[(1) as usize].flags as i32) == 1) as i32) != 0));
    assert!((((((*a.borrow())[(2) as usize].flags as i32) == 2) as i32) != 0));
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let y: Value<i32> = Rc::new(RefCell::new(5));
    let z: Value<i32> = Rc::new(RefCell::new(0));
    (*z.borrow_mut()) += {
        (*x.borrow_mut()) = (*y.borrow());
        (*x.borrow())
    };
    assert!(((((*z.borrow()) == 5) as i32) != 0));
    assert!(((((*x.borrow()) == 5) as i32) != 0));
    let c: Value<u8> = Rc::new(RefCell::new(1_u8));
    let v: Value<i32> = Rc::new(RefCell::new(
        (({
            let rhs_0 = ((((*c.borrow()) as i32) << 3) as u8);
            (*c.borrow_mut()) = rhs_0;
            (*c.borrow())
        }) as i32),
    ));
    assert!(((((*v.borrow()) == 8) as i32) != 0));
    assert!((((((*c.borrow()) as i32) == 8) as i32) != 0));
    let steps: Value<i32> = Rc::new(RefCell::new(0));
    (*c.borrow_mut()) = 1_u8;
    let mut __do_while = true;
    'loop_: while __do_while
        || (((((({
            let rhs_0 = ((((*c.borrow()) as i32) << 1) as u8);
            (*c.borrow_mut()) = rhs_0;
            (*c.borrow())
        }) as i32)
            & 64)
            != 64) as i32)
            != 0)
    {
        __do_while = false;
        (*steps.borrow_mut()).postfix_inc();
    }
    assert!(((((*steps.borrow()) == 6) as i32) != 0));
    assert!((((((*c.borrow()) as i32) == 64) as i32) != 0));
    return 0;
}
