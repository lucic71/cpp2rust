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
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Counted {
    #[offset(0)]
    pub v: i32,
}
impl Counted {
    pub fn new(this: Ptr<Counted>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn copy_from(this: Ptr<Counted>, o: Ptr<Counted>) {
        field!(this, v).write(o.with(|__s| __s.v));
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted { v: self.v.clone() }));
        let __dst: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
        Counted::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign)]
#[placement]
#[byte_size(4)]
pub struct NonConst {
    #[offset(0)]
    pub mark: i32,
}
impl NonConst {
    pub fn new(this: Ptr<NonConst>) {
        field!(this, mark).write(0);
    }
    pub fn new_1(this: Ptr<NonConst>, o: Ptr<NonConst>) {
        field!(this, mark).write((o.with(|__s| __s.mark) + 1));
    }
    pub fn new_2(this: Ptr<NonConst>, o: Ptr<NonConst>) {
        field!(this, mark).write((o.with(|__s| __s.mark) + 10));
    }
}
impl Clone for NonConst {
    fn clone(&self) -> Self {
        let __src: Value<NonConst> = Rc::new(RefCell::new(NonConst {
            mark: self.mark.clone(),
        }));
        let __dst: Value<NonConst> = Rc::new(RefCell::new(<NonConst>::uninit()));
        NonConst::new_1(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl Default for NonConst {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<NonConst> = Rc::new(RefCell::new(<NonConst>::uninit()));
            NonConst::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Ignored {
    #[offset(0)]
    pub v: i32,
}
impl Ignored {
    pub fn new(this: Ptr<Ignored>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn copy_from(this: Ptr<Ignored>, _a0: Ptr<Ignored>) {
        field!(this, v).write(-1_i32);
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for Ignored {
    fn clone(&self) -> Self {
        let __src: Value<Ignored> = Rc::new(RefCell::new(Ignored { v: self.v.clone() }));
        let __dst: Value<Ignored> = Rc::new(RefCell::new(<Ignored>::uninit()));
        Ignored::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign)]
#[placement]
#[byte_size(12)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub c: Counted,
    #[offset(4)]
    #[byte_size(8)]
    #[array_len(2)]
    pub arr: Box<[Counted]>,
}
impl Holder {
    pub fn copy_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        Counted::copy_from(field_ptr!(this, c), { field_ptr!(_a0, c) });
        {
            let __d = array_field_ptr!(this, arr);
            for __i in 0..2_usize {
                Counted::copy_from(__d.offset(__i as isize), {
                    array_field_ptr!(_a0, arr).offset(__i)
                });
            }
        };
    }
}
impl Default for Holder {
    fn default() -> Self {
        {
            Holder {
                c: <Counted>::default(),
                arr: (0..2)
                    .map(|_| <Counted>::default())
                    .collect::<Box<[Counted]>>(),
            }
        }
    }
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Box_int_ {
    #[offset(0)]
    pub val: i32,
}
impl Box_int_ {
    pub fn new(this: Ptr<Box_int_>, mut v: i32) {
        field!(this, val).write(v);
    }
    pub fn copy_from(this: Ptr<Box_int_>, o: Ptr<Box_int_>) {
        field!(this, val).write(o.with(|__s| __s.val));
    }
    pub fn move_from(this: Ptr<Box_int_>, _a0: Ptr<Box_int_>) {
        field!(this, val).write({ (*_a0.upgrade().deref()).val });
    }
}
impl Clone for Box_int_ {
    fn clone(&self) -> Self {
        let __src: Value<Box_int_> = Rc::new(RefCell::new(Box_int_ {
            val: self.val.clone(),
        }));
        let __dst: Value<Box_int_> = Rc::new(RefCell::new(<Box_int_>::uninit()));
        Box_int_::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, Default)]
#[placement]
#[byte_size(4)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(4)]
    pub box_: Box_int_,
}
impl Owner {
    pub fn copy_from(this: Ptr<Owner>, _a0: Ptr<Owner>) {
        Box_int_::copy_from(field_ptr!(this, box_), { field_ptr!(_a0, box_) });
    }
    pub fn move_from(this: Ptr<Owner>, _a0: Ptr<Owner>) {
        Box_int_::move_from(field_ptr!(this, box_), { field_ptr!(_a0, box_) });
    }
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct OutOfLine {
    #[offset(0)]
    pub v: i32,
}
impl OutOfLine {
    pub fn new(this: Ptr<OutOfLine>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn copy_from(this: Ptr<OutOfLine>, o: Ptr<OutOfLine>) {
        field!(this, v).write((o.with(|__s| __s.v) + 100));
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for OutOfLine {
    fn clone(&self) -> Self {
        let __src: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { v: self.v.clone() }));
        let __dst: Value<OutOfLine> = Rc::new(RefCell::new(<OutOfLine>::uninit()));
        OutOfLine::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl OutOfLine {}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign)]
#[placement]
#[byte_size(4)]
pub struct Tracked {
    #[offset(0)]
    pub copied_from: i32,
}
impl Tracked {
    pub fn new(this: Ptr<Tracked>) {
        field!(this, copied_from).write(0);
    }
    pub fn copy_from(this: Ptr<Tracked>, o: Ptr<Tracked>) {
        field!(this, copied_from).write(0);
        field!(o, copied_from).with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        let __src: Value<Tracked> = Rc::new(RefCell::new(Tracked {
            copied_from: self.copied_from.clone(),
        }));
        let __dst: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
        Tracked::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl Default for Tracked {
    fn default() -> Self {
        { Tracked { copied_from: 0 } }
    }
}
pub fn touch_1(mut t: Ptr<Tracked>) {
    field!(t, copied_from).with_mut(|__v| {
        __v.prefix_inc();
    });
}
pub fn by_value_2(c: Value<Counted>) -> i32 {
    return { (*c.borrow()).v };
}
pub fn make_3(__ret: Ptr<Counted>, mut v: i32) {
    let c: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::new(c.as_pointer(), { v });
    Counted::copy_from(__ret.clone(), { c.as_pointer() });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::new(a.as_pointer(), { 1 });
    let b: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::copy_from(b.as_pointer(), { a.as_pointer() });
    let c: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::copy_from(c.as_pointer(), { a.as_pointer() });
    let d: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::copy_from(d.as_pointer(), { a.as_pointer() });
    assert!((copies_0.with(|rc| *rc.borrow()) == 3));
    assert!(
        ({
            (({ (*b.borrow()).v } == 1) && ({ (*c.borrow()).v } == 1)) && ({ (*d.borrow()).v } == 1)
        })
    );
    assert!(
        (({
            by_value_2({
                let __v: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
                Counted::copy_from(__v.as_pointer(), { a.as_pointer() });
                __v
            })
        }) == 1)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 4));
    let e: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    ({ make_3(e.as_pointer(), 5) });
    assert!(({ ({ (*e.borrow()).v } == 5) }));
    assert!((copies_0.with(|rc| *rc.borrow()) == 5));
    let f: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::new(f.as_pointer(), { 6 });
    assert!(({ ({ (*f.borrow()).v } == 6) }));
    assert!((copies_0.with(|rc| *rc.borrow()) == 5));
    let g: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::new(g.as_pointer(), { 7 });
    let h: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::copy_from(h.as_pointer(), { g.as_pointer() });
    assert!(({ ({ (*h.borrow()).v } == 7) }));
    assert!((copies_0.with(|rc| *rc.borrow()) == 6));
    let hold: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    {
        let __d = hold.as_pointer();
        Counted::new(field_ptr!(__d, c), { 8 });
        {
            let __d = array_field_ptr!(__d, arr);
            Counted::new(__d.offset(0), { 9 });
            Counted::new(__d.offset(1), { 10 });
        };
    };
    let hold2: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    Holder::copy_from(hold2.as_pointer(), { hold.as_pointer() });
    assert!(
        ({
            (({ (*hold2.borrow()).c.v } == 8)
                && (array_elem!(hold2.as_pointer(), arr, 0).with(|__s| __s.v) == 9))
                && (array_elem!(hold2.as_pointer(), arr, 1).with(|__s| __s.v) == 10)
        })
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 9));
    let mut vec_: Vec<Counted> = Vec::new();
    {
        let a0_clone = (*a.borrow()).clone();
        vec_.push(a0_clone)
    };
    assert!((vec_[0_usize].v == 1));
    assert!((copies_0.with(|rc| *rc.borrow()) == 10));
    let i1: Value<Ignored> = Rc::new(RefCell::new(<Ignored>::uninit()));
    Ignored::new(i1.as_pointer(), { 1 });
    let i2: Value<Ignored> = Rc::new(RefCell::new(<Ignored>::uninit()));
    Ignored::copy_from(i2.as_pointer(), { i1.as_pointer() });
    assert!(({ ({ (*i1.borrow()).v } == 1) && ({ (*i2.borrow()).v } == -1_i32) }));
    assert!((copies_0.with(|rc| *rc.borrow()) == 11));
    let o: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
    {
        let __d = o.as_pointer();
        Box_int_::new(field_ptr!(__d, box_), { 3 });
    };
    let o2: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
    Owner::move_from(o2.as_pointer(), { o.as_pointer() });
    assert!(({ ({ (*o2.borrow()).box_.val } == 3) }));
    let ol1: Value<OutOfLine> = Rc::new(RefCell::new(<OutOfLine>::uninit()));
    OutOfLine::new(ol1.as_pointer(), { 5 });
    let ol2: Value<OutOfLine> = Rc::new(RefCell::new(<OutOfLine>::uninit()));
    OutOfLine::copy_from(ol2.as_pointer(), { ol1.as_pointer() });
    assert!(({ ({ (*ol2.borrow()).v } == 105) }));
    assert!((copies_0.with(|rc| *rc.borrow()) == 12));
    let n: Value<NonConst> = Rc::new(RefCell::new(<NonConst>::uninit()));
    NonConst::new(n.as_pointer());
    let n1: Value<NonConst> = Rc::new(RefCell::new(<NonConst>::uninit()));
    NonConst::new_1(n1.as_pointer(), { n.as_pointer() });
    let cn: Value<NonConst> = Rc::new(RefCell::new(<NonConst>::uninit()));
    NonConst::new(cn.as_pointer());
    let n2: Value<NonConst> = Rc::new(RefCell::new(<NonConst>::uninit()));
    NonConst::new_2(n2.as_pointer(), { cn.as_pointer() });
    assert!(({ ({ (*n1.borrow()).mark } == 1) }));
    assert!(({ ({ (*n2.borrow()).mark } == 10) }));
    let t1: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(t1.as_pointer());
    let t2: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::copy_from(t2.as_pointer(), { t1.as_pointer() });
    ({ TrackedImpl::copy_assign(&t2.as_pointer(), t1.as_pointer()) });
    assert!(({ ({ (*t1.borrow()).copied_from } == 2) }));
    let ct: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(ct.as_pointer());
    let t3: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::copy_from(t3.as_pointer(), { ct.as_pointer() });
    ({ touch_1((ct.as_pointer())) });
    assert!(({ ({ (*ct.borrow()).copied_from } == 2) }));
    return 0;
}
pub trait CountedImpl {
    fn copy_assign(&self, _a0: Ptr<Counted>) -> Ptr<Counted>;
}
impl CountedImpl for Ptr<Counted> {
    fn copy_assign(&self, _a0: Ptr<Counted>) -> Ptr<Counted> {
        field!((*self), v).write({ _a0.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub trait HolderImpl {
    fn copy_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn copy_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        ({
            let _arg0: Ptr<Counted> = field_ptr!(_a0, c);
            CountedImpl::copy_assign(&field_ptr!((*self), c), _arg0)
        });
        {
            let mut __tmp_0 = { (array_field_ptr!((*self), arr)).to_any() };
            __tmp_0.memcpy(&(array_field_ptr!(_a0, arr)).to_any(), 8_usize);
            (__tmp_0).clone()
        };
        return (*self).clone();
    }
}
pub trait IgnoredImpl {
    fn copy_assign(&self, _a0: Ptr<Ignored>) -> Ptr<Ignored>;
}
impl IgnoredImpl for Ptr<Ignored> {
    fn copy_assign(&self, _a0: Ptr<Ignored>) -> Ptr<Ignored> {
        field!((*self), v).write({ _a0.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub trait NonConstImpl {
    fn copy_assign(&self, _a0: Ptr<NonConst>) -> Ptr<NonConst>;
}
impl NonConstImpl for Ptr<NonConst> {
    fn copy_assign(&self, _a0: Ptr<NonConst>) -> Ptr<NonConst> {
        field!((*self), mark).write({ _a0.with(|__s| __s.mark) });
        return (*self).clone();
    }
}
pub trait OutOfLineImpl {
    fn copy_assign(&self, _a0: Ptr<OutOfLine>) -> Ptr<OutOfLine>;
}
impl OutOfLineImpl for Ptr<OutOfLine> {
    fn copy_assign(&self, _a0: Ptr<OutOfLine>) -> Ptr<OutOfLine> {
        field!((*self), v).write({ _a0.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub trait TrackedImpl {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked>;
}
impl TrackedImpl for Ptr<Tracked> {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked> {
        field!(o, copied_from).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
