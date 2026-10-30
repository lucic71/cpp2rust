extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_bytes_0(mut buf: Ptr<()>, mut len: ()) -> () {
    let sum: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while (i < len) {
        (*sum.borrow_mut()) += (elem!(buf, i).read());
        i.postfix_inc();
    }
    return (*sum.borrow());
}
thread_local!(
    pub static g_packet_1: Value<Ptr<()>> =
        Rc::new(RefCell::new(Ptr::<()>::from_string_literal(b"\x01\0")));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut a: () = ({ sum_bytes_0(Ptr::<()>::from_string_literal(b"\x01\0"), 2_()) });
    let mut b: () = ({ sum_bytes_0((*g_packet_1.with(Value::clone).borrow()).clone(), 2_()) });
    unimplemented!();
    unimplemented!();
    let mut high: () = ({ sum_bytes_0(Ptr::<()>::from_string_literal(b"\xc2\0A"), 3_()) });
    unimplemented!();
    let mut c: () = (b"\r\n.\r\n"[(0_()) as usize] + b"\r\n.\r\n"[(3_()) as usize]);
    unimplemented!();
    let mut idx: () = 1_();
    let mut d: () = b"abcd"[(idx) as usize];
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = g_packet_1.with(|_| ());
}
