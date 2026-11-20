extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static moves_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Elem {
    #[offset(0)]
    pub v: i32,
}
impl Elem {
    pub fn new(this: Ptr<Elem>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn copy_from(this: Ptr<Elem>, o: Ptr<Elem>) {
        field!(this, v).write(o.with(|__s| __s.v));
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn move_from(this: Ptr<Elem>, o: Ptr<Elem>) {
        field!(this, v).write(o.with(|__s| __s.v));
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        let __src: Value<Elem> = Rc::new(RefCell::new(Elem { v: self.v.clone() }));
        let __dst: Value<Elem> = Rc::new(RefCell::new(<Elem>::uninit()));
        Elem::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<Elem>> = Rc::new(RefCell::new(Vec::new()));
    v.with_mut(|__b0| {
        if 4_usize > (*__b0).capacity() as usize {
            let len_0 = (*__b0).len();
            (*__b0).reserve_exact(4_usize - len_0 as usize);
        }
    });
    let a: Value<Elem> = Rc::new(RefCell::new(<Elem>::uninit()));
    Elem::new(a.as_pointer(), { 1 });
    let b: Value<Elem> = Rc::new(RefCell::new(<Elem>::uninit()));
    Elem::new(b.as_pointer(), { 2 });
    {
        let mut __tmp_0 = { <RandomAccessIterator<Elem>>::from(v.decay().end()) };
        (*v.borrow_mut()).insert(
            __tmp_0.index(),
            Rc::try_unwrap({
                let __v: Value<Elem> = Rc::new(RefCell::new(<Elem>::uninit()));
                Elem::move_from(__v.as_pointer(), { a.as_pointer() });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        );
        (__tmp_0).clone()
    };
    {
        let mut __tmp_1 = { <RandomAccessIterator<Elem>>::from(v.decay().end()) };
        let mut __tmp_2 = { (*b.borrow()).clone() };
        (*v.borrow_mut()).insert(__tmp_1.index(), __tmp_2);
        (__tmp_1).clone()
    };
    let mut ok: bool = (((copies_0.with(|rc| *rc.borrow()) == 1)
        && (moves_1.with(|rc| *rc.borrow()) == 1))
        && ({ (*v.borrow())[0_usize].v } == 1))
        && ({ (*v.borrow())[1_usize].v } == 2);
    let mut pa: Ptr<Elem> = (a.as_pointer());
    printf!(
        b"%d %d %d\n",
        copies_0.with(|rc| *rc.borrow()),
        moves_1.with(|rc| *rc.borrow()),
        pa.with(|__s| __s.v)
    );
    return if ok { 0 } else { 1 };
}
pub trait ElemImpl {
    fn copy_assign(&self, _a0: Ptr<Elem>) -> Ptr<Elem>;
}
impl ElemImpl for Ptr<Elem> {
    fn copy_assign(&self, _a0: Ptr<Elem>) -> Ptr<Elem> {
        field!((*self), v).write({ _a0.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
