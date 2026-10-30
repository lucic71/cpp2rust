extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(mut x: ()) -> () {
    return (x * 2_());
}
pub fn triple_it_1(mut x: ()) -> () {
    return (x * 3_());
}
thread_local!(
    pub static g_op_2: Value<FnPtr<fn(()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::null()));
);
pub fn set_op_3(mut fn_: FnPtr<fn(()) -> ()>) {
    g_op_2.with(|rc| *rc.borrow_mut() = (fn_).clone());
}
pub fn call_op_4(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    if !(*g_op_2.with(Value::clone).borrow()).is_null() {
        return ({ (*g_op_2.with(Value::clone).borrow()).call((*x.borrow())) });
    }
    return (*x.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    ({ set_op_3(FnPtr::<fn(()) -> ()>::new(double_it_0)) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ set_op_3(FnPtr::<fn(()) -> ()>::new(triple_it_1)) });
    unimplemented!();
    unimplemented!();
    ({ set_op_3(FnPtr::<fn(()) -> ()>::null()) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = g_op_2.with(|_| ());
}
