extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_foo_0(mut p: ()) -> () {
    return (p.reinterpret_cast::<()>().read());
}
pub fn foo_1(mut fn_: FnPtr<fn(()) -> ()>, mut pi: Ptr<()>) -> () {
    return ({ fn_.call((pi as Ptr<()>).to_any()) });
}
pub fn twice_2(mut x: ()) -> () {
    return (x).wrapping_mul(2_());
}
pub fn twice_in_place_3(x: Ptr<()>) -> () {
    x.write({ (x.read()).wrapping_mul(2_()) });
    return (x.read());
}
pub fn ret_size_4(mut v: ()) -> () {
    return (v + 1_());
}
pub fn call_fn_5(mut f: FnPtr<fn(()) -> ()>, mut v: ()) -> () {
    return ({ f.call(v) }).wrapping_mul(2_());
}
pub fn identity_hash_6(v: ()) -> () {
    let v: Value<()> = Rc::new(RefCell::new(v));
    return (*v.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HashHolder_unsigned_long__ptr__bool__ {
    #[offset(0)]
    #[byte_size(8)]
    pub h: FnPtr<fn(()) -> ()>,
}
impl HashHolder_unsigned_long__ptr__bool__ {
    pub fn new(h: Ptr<FnPtr<fn(()) -> ()>>) -> Self {
        Self { h: (h.read()) }
    }
}
thread_local!(
    pub static total_7: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn add_to_total_8(mut x: ()) {
    total_7.with(|rc| *rc.borrow_mut() += x);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut fn_: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::null();
    unimplemented!();
    unimplemented!();
    fn_ = FnPtr::<fn(()) -> ()>::new(my_foo_0);
    unimplemented!();
    unimplemented!();
    let a: Value<()> = Rc::new(RefCell::new(10_()));
    unimplemented!();
    let mut ul_fn: FnPtr<fn(()) -> ()> = (FnPtr::<fn(()) -> ()>::new(twice_2));
    let mut n: () = 21_();
    let mut r: () = ({ ul_fn.call(n) });
    unimplemented!();
    let mut ul_ref_fn: FnPtr<fn(Ptr<()>) -> ()> =
        (FnPtr::<fn(Ptr<()>) -> ()>::new(twice_in_place_3));
    let m: Value<()> = Rc::new(RefCell::new(21_()));
    let mut q: () = ({ ul_ref_fn.call(m.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut hh: HashHolder_unsigned_long__ptr__bool__ = {
        let __tmp_0: Value<FnPtr<fn(()) -> ()>> =
            Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(identity_hash_6)));
        HashHolder_unsigned_long__ptr__bool__::new({ __tmp_0.as_pointer() })
    };
    unimplemented!();
    let mut handler: () = FnPtr::<fn(())>::new(add_to_total_8);
    ({ handler.call(2_()) });
    ({ handler.call(3_()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = total_7.with(|_| ());
}
