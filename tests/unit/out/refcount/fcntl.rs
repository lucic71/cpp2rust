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
    let fds: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        (((match nix::unistd::pipe() {
            Ok((__r, __w)) => {
                let __fds = (fds.as_pointer() as Ptr<i32>).clone();
                __fds.write(FdRegistry::register(__r));
                __fds.offset(1).write(FdRegistry::register(__w));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    let flags: Value<i32> = Rc::new(RefCell::new(libcc2rs::fcntl_refcount(
        (*fds.borrow())[(0) as usize],
        3,
        &[(0).into()],
    )));
    assert!(((((*flags.borrow()) >= 0) as i32) != 0));
    assert!((((((*flags.borrow()) & ::libc::O_NONBLOCK) == 0) as i32) != 0));
    assert!(
        (((libcc2rs::fcntl_refcount(
            (*fds.borrow())[(0) as usize],
            4,
            &[((*flags.borrow()) | ::libc::O_NONBLOCK).into(),]
        ) == 0) as i32)
            != 0)
    );
    (*flags.borrow_mut()) =
        libcc2rs::fcntl_refcount((*fds.borrow())[(0) as usize], 3, &[(0).into()]);
    assert!((((((*flags.borrow()) & ::libc::O_NONBLOCK) != 0) as i32) != 0));
    let b: Value<u8> = <Value<u8>>::default();
    assert!(
        (((libcc2rs::read_refcount(
            (*fds.borrow())[(0) as usize],
            ((b.as_pointer()) as Ptr::<u8>).to_any().clone(),
            1_usize
        ) == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!(
        ((((libcc2rs::fcntl_refcount((*fds.borrow())[(0) as usize], 1, &[(0).into(),]) & 1) == 0)
            as i32)
            != 0)
    );
    assert!(
        (((libcc2rs::fcntl_refcount((*fds.borrow())[(0) as usize], 2, &[(1).into(),]) == 0)
            as i32)
            != 0)
    );
    assert!(
        ((((libcc2rs::fcntl_refcount((*fds.borrow())[(0) as usize], 1, &[(0).into(),]) & 1) != 0)
            as i32)
            != 0)
    );
    assert!((((libcc2rs::close_refcount((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    assert!((((libcc2rs::close_refcount((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
    return 0;
}
