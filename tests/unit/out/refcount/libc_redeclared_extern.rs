extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fileno_0(stream: Ptr<CFile>) -> i32 {
    let stream: Value<Ptr<CFile>> = Rc::new(RefCell::new(stream));
    (*stream.borrow()).clone();
    return 42;
}
#[repr(C)]
#[derive(Clone)]
pub struct sink {
    pub in_: Ptr<CFile>,
    pub closer: FnPtr<fn(Ptr<CFile>) -> i32>,
}
impl Default for sink {
    fn default() -> Self {
        sink {
            in_: Ptr::null(),
            closer: FnPtr::<fn(Ptr<CFile>) -> i32>::null(),
        }
    }
}
impl ByteRepr for sink {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.in_.to_bytes(&mut buf[0..8]);
        self.closer.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            in_: <Ptr<CFile>>::from_bytes(&buf[0..8]),
            closer: <FnPtr<fn(Ptr<CFile>) -> i32>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ fileno_0((libcc2rs::c_stdout()).clone()) }) == 42) as i32) != 0));
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"hello")));
    assert!(((((*s.borrow()).to_c_string_iterator().count() == 5_usize) as i32) != 0));
    assert!(
        (((Ptr::from_string_literal(b"").to_c_string_iterator().count() == 0_usize) as i32) != 0)
    );
    let tty: Value<i32> = Rc::new(RefCell::new(
        match FdRegistry::with_fd(1, |__fd| nix::unistd::isatty(__fd)) {
            Ok(__tty) => __tty as i32,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                0
            }
        },
    ));
    assert!(((((*tty.borrow()) == 0) as i32) != 0));
    let k: Value<sink> = <Value<sink>>::default();
    (*k.borrow_mut()).in_ = libc::popen(
        Ptr::from_string_literal(b"exit 7"),
        Ptr::from_string_literal(b"r"),
    );
    assert!((((!(((*k.borrow()).in_).is_null())) as i32) != 0));
    (*k.borrow_mut()).closer = FnPtr::<fn(Ptr<CFile>) -> i32>::new(libcc2rs::pclose_refcount);
    assert!(
        (((({
            let _arg0: Ptr<CFile> = ((*k.borrow()).in_).clone();
            (*(*k.borrow()).closer)(_arg0)
        }) == (7 * 256)) as i32)
            != 0)
    );
    (*k.borrow_mut()).in_ = match CFile::open(
        &Ptr::from_string_literal(b"/dev/null").to_rust_string(),
        &Ptr::from_string_literal(b"r").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!(((*k.borrow()).in_).is_null())) as i32) != 0));
    (*k.borrow_mut()).closer = FnPtr::<fn(Ptr<CFile>) -> i32>::new(libcc2rs::fclose_refcount);
    assert!(
        (((({
            let _arg0: Ptr<CFile> = ((*k.borrow()).in_).clone();
            (*(*k.borrow()).closer)(_arg0)
        }) == 0) as i32)
            != 0)
    );
    return 0;
}
