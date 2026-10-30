extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub v: (),
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            v: unimplemented!(),
        }
    }
}
pub fn write_through_0(mut p: Ptr<()>) {
    p.write(42_());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut p: Ptr<()> = (v.as_pointer());
    let mut a: () = (unimplemented!().read());
    unimplemented!().write(30_());
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    unimplemented!();
    unimplemented!();
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let mut b: () = (unimplemented!().read());
    unimplemented!().write(60_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ write_through_0((unimplemented!())) });
    unimplemented!();
    return 0_();
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
