extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static next_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn bump_1(mut v: Ptr<Vec<i32>>, mut x: i32) -> i32 {
    {
        let __rhs = x;
        (v).decay()
            .offset(0_usize as isize)
            .with_mut(|__v| *__v = *__v + __rhs)
    };
    return (*next_0.with(Value::clone).borrow_mut()).postfix_inc();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; 4_usize as usize]));
    let items: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(3_usize) as usize).map(|_| 0_u32).collect::<Vec<_>>(),
    ));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (*items.borrow()).len()) {
        let __rhs = (i).wrapping_mul(2_u32);
        items.decay().offset((i as usize) as isize).write(__rhs);
        i.prefix_inc();
    }
    {
        let rhs_0 = (items.decay().offset(1_usize as isize).read()).wrapping_add(5_u32);
        items.decay().offset(1_usize as isize).write(rhs_0)
    };
    items
        .decay()
        .offset(2_usize as isize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(
        (((items.decay().offset(0_usize as isize).read()) == 0_u32)
            && ((items.decay().offset(1_usize as isize).read()) == 7_u32))
            && ((items.decay().offset(2_usize as isize).read()) == 5_u32)
    );
    v.decay()
        .offset(((v.decay().offset(0_usize as isize).read()) as usize) as isize)
        .write(3);
    let __rhs = ((v.decay().offset(1_usize as isize).read()) + 4);
    v.decay()
        .offset(((v.decay().offset(1_usize as isize).read()) as usize) as isize)
        .write(__rhs);
    assert!(
        ((v.decay().offset(1_usize as isize).read()) == 3)
            && ((v.decay().offset(3_usize as isize).read()) == 7)
    );
    let mut p: Ptr<i32> = (v.decay().offset(0_usize as isize));
    v.decay().offset(((p.read()) as usize) as isize).write(9);
    assert!(((v.decay().offset(1_usize as isize).read()) == 9));
    v.decay()
        .offset((({ bump_1((v.as_pointer()), 10) }) as usize) as isize)
        .write(2);
    assert!(((v.decay().offset(0_usize as isize).read()) == 2));
    assert!(
        (({
            let _v: Ptr<Vec<i32>> = (v.as_pointer());
            let _x: i32 = (v.decay().offset(2_usize as isize).read());
            bump_1(_v, _x)
        }) == 1)
            && ((v.decay().offset(0_usize as isize).read()) == 3)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = next_0.with(|_| ());
}
