extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moves_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static made_2: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct Key {
    #[offset(0)]
    pub k: (),
}
impl Key {
    pub fn new(mut k: ()) -> Self {
        Self { k: k }
    }
    pub fn copy_from(o: Ptr<Key>) -> Self {
        let __this: Key = Self {
            k: o.with(|__s| __s.k),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(o: Ptr<Key>) -> Self {
        let __this: Key = Self {
            k: o.with(|__s| __s.k),
        };
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for Key {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        KeyImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Key {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(KeyImpl::operator_lt(__lhs, __rhs.clone()))
            && !(KeyImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
impl Clone for Key {
    fn clone(&self) -> Self {
        let __src: Value<Key> = Rc::new(RefCell::new(Key { k: self.k.clone() }));
        Key::copy_from(__src.as_pointer())
    }
}
pub fn make_key_3(mut k: ()) -> Key {
    (*made_2.with(Value::clone).borrow_mut()).prefix_inc();
    return Key::new({ k });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut ok: () = true;
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut a: Key = Key::new({ 1_() });
    let mut b: Key = Key::new({ 2_() });
    let mut c: Key = Key::new({ 3_() });
    unimplemented!().write(10_());
    ok &= (copies_0.with(|rc| *rc.borrow()) == 1_());
    unimplemented!().write(20_());
    ok &= (copies_0.with(|rc| *rc.borrow()) == 2_());
    unimplemented!().write(11_());
    ok &= (copies_0.with(|rc| *rc.borrow()) == 2_());
    ok &= ({ (unimplemented!().read()).second } == 11_());
    ok &= ((unimplemented!().read()) == 20_());
    ok &= (copies_0.with(|rc| *rc.borrow()) == 2_());
    unimplemented!().write(40_());
    unimplemented!().write(41_());
    ok &= (((made_2.with(|rc| *rc.borrow()) == 2_()) && (moves_1.with(|rc| *rc.borrow()) == 1_()))
        && (copies_0.with(|rc| *rc.borrow()) == 2_()));
    unimplemented!().write(30_());
    unimplemented!().write(31_());
    ok &= (((moves_1.with(|rc| *rc.borrow()) == 2_())
        && (copies_0.with(|rc| *rc.borrow()) == 2_()))
        && (c.k == 3_()));
    ok &= (unimplemented!() == 4_());
    {
        unimplemented!().with_mut(|__v| *__v = *__v + 1_())
    };
    ok &= ((copies_0.with(|rc| *rc.borrow()) == 2_()) && ((unimplemented!().read()) == 12_()));
    let mut first: Ptr<Key> =
        (field_ptr!(Rc::new(RefCell::new(unimplemented!())).as_pointer(), first));
    unimplemented!().write(0_());
    ok &= (moves_1.with(|rc| *rc.borrow()) == 3_());
    ok &= ({ (field_ptr!(Rc::new(RefCell::new(unimplemented!())).as_pointer(), first)) } == {
        (first).clone()
    });
    let n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ok &= (copies_0.with(|rc| *rc.borrow()) == (2_() + 5_()));
    ok &= ((unimplemented!() == 5_()) && ((unimplemented!().read()) == 12_()));
    unimplemented!();
    ok &= (unimplemented!() == 1_());
    ok &= (unimplemented!() == 0_());
    ok &= (unimplemented!() == 1_());
    ok &= ((copies_0.with(|rc| *rc.borrow()) == 7_()) && (moves_1.with(|rc| *rc.borrow()) == 3_()));
    ok &= ((unimplemented!() == 2_()) && (unimplemented!() == 5_()));
    let mut sum: () = 0_();
    'loop_: for entry in IterRange::new(unimplemented!(), unimplemented!()) {
        let entry: Ptr<()> = unimplemented!();
        sum += ({ entry.with(|__s| __s.first.k) } * { entry.with(|__s| __s.second) });
    }
    ok &= (sum == ((3_() * 31_()) + (4_() * 41_())));
    ok &= ((copies_0.with(|rc| *rc.borrow()) == 7_()) && (moves_1.with(|rc| *rc.borrow()) == 3_()));
    println!(
        "{} {} {}",
        copies_0.with(|rc| *rc.borrow()),
        moves_1.with(|rc| *rc.borrow()),
        made_2.with(|rc| *rc.borrow())
    );
    return if ok { 0_() } else { 1_() };
}
pub trait KeyImpl {
    fn operator_lt(&self, o: Ptr<Key>) -> ();
}
impl KeyImpl for Ptr<Key> {
    fn operator_lt(&self, o: Ptr<Key>) -> () {
        return ({ (*self).with(|__s| __s.k) } < { o.with(|__s| __s.k) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
    let _ = made_2.with(|_| ());
}
