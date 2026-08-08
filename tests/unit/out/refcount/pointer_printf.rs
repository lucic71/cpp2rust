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
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    {
        print!("{:?} {}", (*p.borrow()), ((*p.borrow()).read()));
        let _ = ::std::io::Write::flush(&mut ::std::io::stdout());
    };
    return 0;
}
