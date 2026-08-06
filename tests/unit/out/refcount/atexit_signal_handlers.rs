extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static got_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn on_signal_1(sig: i32) {
    let sig: Value<i32> = Rc::new(RefCell::new(sig));
    (*got_0.with(Value::clone).borrow_mut()) = (*sig.borrow());
}
pub fn first_exit_2() {
    println!("first");
}
pub fn second_exit_3() {
    println!("second");
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    assert!(
        ((((libcc2rs::signal_refcount(10, FnPtr::<fn(i32)>::new(on_signal_1).clone())).is_null())
            as i32)
            != 0)
    );
    assert!(
        (((match nix::sys::signal::Signal::try_from(10) {
            Ok(__sig) => match nix::sys::signal::raise(__sig) {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            },
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(((((*got_0.with(Value::clone).borrow()) == 10) as i32) != 0));
    (*got_0.with(Value::clone).borrow_mut()) = 0;
    let prev: Value<FnPtr<fn(i32)>> = Rc::new(RefCell::new(libcc2rs::signal_refcount(
        10,
        (<FnPtr<fn(i32)>>::from_int((1) as usize)).clone(),
    )));
    assert!(
        ((({
            let _lhs = (*prev.borrow()).clone();
            _lhs == FnPtr::<fn(i32)>::new(on_signal_1)
        }) as i32)
            != 0)
    );
    assert!(
        (((match nix::sys::signal::Signal::try_from(10) {
            Ok(__sig) => match nix::sys::signal::raise(__sig) {
                Ok(()) => 0,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            },
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(((((*got_0.with(Value::clone).borrow()) == 0) as i32) != 0));
    assert!(
        (((libcc2rs::atexit_refcount(FnPtr::<fn()>::new(first_exit_2).clone()) == 0) as i32) != 0)
    );
    assert!(
        (((libcc2rs::atexit_refcount(FnPtr::<fn()>::new(second_exit_3).clone()) == 0) as i32) != 0)
    );
    println!("main");
    return 0;
}
