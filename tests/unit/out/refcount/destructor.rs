extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static global_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {}
    pub fn new_1(this: Ptr<S>) {}
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct Defaulted {
    #[offset(0)]
    #[byte_size(1)]
    pub s: S,
}
impl Defaulted {
    pub fn copy_from(this: Ptr<Defaulted>, _a0: Ptr<Defaulted>) {
        S::copy_from(field_ptr!(this, s), { field_ptr!(_a0, s) });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct Middle {
    #[offset(0)]
    #[byte_size(1)]
    pub s: S,
}
impl Middle {
    pub fn copy_from(this: Ptr<Middle>, _a0: Ptr<Middle>) {
        S::copy_from(field_ptr!(this, s), { field_ptr!(_a0, s) });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(1)]
    pub m: Middle,
}
impl Outer {
    pub fn copy_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        Middle::copy_from(field_ptr!(this, m), { field_ptr!(_a0, m) });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor)]
#[byte_size(3)]
pub struct ArrayMember {
    #[offset(0)]
    #[byte_size(3)]
    #[array_len(3)]
    pub items: Box<[S]>,
}
impl ArrayMember {
    pub fn copy_from(this: Ptr<ArrayMember>, _a0: Ptr<ArrayMember>) {
        {
            let __d = array_field_ptr!(this, items);
            for __i in 0..3_usize {
                S::copy_from(__d.offset(__i as isize), {
                    array_field_ptr!(_a0, items).offset(__i)
                });
            }
        };
    }
}
impl Default for ArrayMember {
    fn default() -> Self {
        {
            ArrayMember {
                items: (0..3).map(|_| <S>::default()).collect::<Box<[S]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct EmptyBody {
    #[offset(0)]
    #[byte_size(1)]
    pub s: S,
}
impl EmptyBody {
    pub fn copy_from(this: Ptr<EmptyBody>, _a0: Ptr<EmptyBody>) {
        S::copy_from(field_ptr!(this, s), { field_ptr!(_a0, s) });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct Templated_char_ {
    #[offset(0)]
    pub v: i8,
}
impl Templated_char_ {
    pub fn copy_from(this: Ptr<Templated_char_>, _a0: Ptr<Templated_char_>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Templated_int_ {
    #[offset(0)]
    pub v: i32,
}
impl Templated_int_ {
    pub fn copy_from(this: Ptr<Templated_int_>, _a0: Ptr<Templated_int_>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Copied {
    #[offset(0)]
    pub v: i32,
}
impl Copied {
    pub fn copy_from(this: Ptr<Copied>, _a0: Ptr<Copied>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
thread_local!(
    pub static order_1: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>()));
);
thread_local!(
    pub static order_count_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Tagged {
    #[offset(0)]
    pub tag: i32,
}
impl Tagged {
    pub fn copy_from(this: Ptr<Tagged>, _a0: Ptr<Tagged>) {
        field!(this, tag).write(_a0.with(|__s| __s.tag));
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(20)]
pub struct Ordered {
    #[offset(0)]
    #[byte_size(4)]
    pub first: Tagged,
    #[offset(4)]
    pub dummy1: i32,
    #[offset(8)]
    #[byte_size(4)]
    pub second: Tagged,
    #[offset(12)]
    pub dummy2: i32,
    #[offset(16)]
    #[byte_size(4)]
    pub third: Tagged,
}
impl Ordered {
    pub fn copy_from(this: Ptr<Ordered>, _a0: Ptr<Ordered>) {
        Tagged::copy_from(field_ptr!(this, first), { field_ptr!(_a0, first) });
        field!(this, dummy1).write(_a0.with(|__s| __s.dummy1));
        Tagged::copy_from(field_ptr!(this, second), { field_ptr!(_a0, second) });
        field!(this, dummy2).write(_a0.with(|__s| __s.dummy2));
        Tagged::copy_from(field_ptr!(this, third), { field_ptr!(_a0, third) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        {
            let __d = s.as_pointer();
        };
        let _dtor_s = std::mem::DropGuard::new(s.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 1));
    {
        let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        {
            let __d = s.as_pointer();
        };
        let _dtor_s = std::mem::DropGuard::new(s.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 2));
    {
        let d: Value<Defaulted> = Rc::new(RefCell::new(<Defaulted>::uninit()));
        {
            let __d = d.as_pointer();
            {
                let __d = field_ptr!(__d, s);
            };
        };
        let _dtor_d = std::mem::DropGuard::new(d.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 3));
    {
        let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
        {
            let __d = o.as_pointer();
            {
                let __d = field_ptr!(__d, m);
                {
                    let __d = field_ptr!(__d, s);
                };
            };
        };
        let _dtor_o = std::mem::DropGuard::new(o.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 4));
    {
        let am: Value<ArrayMember> = Rc::new(RefCell::new(<ArrayMember>::uninit()));
        {
            let __d = am.as_pointer();
            {
                let __d = array_field_ptr!(__d, items);
                {
                    let __d = __d.offset(0);
                };
                {
                    let __d = __d.offset(1);
                };
                {
                    let __d = __d.offset(2);
                };
            };
        };
        let _dtor_am = std::mem::DropGuard::new(am.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 7));
    {
        let e: Value<EmptyBody> = Rc::new(RefCell::new(<EmptyBody>::uninit()));
        {
            let __d = e.as_pointer();
            {
                let __d = field_ptr!(__d, s);
            };
        };
        let _dtor_e = std::mem::DropGuard::new(e.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 8));
    {
        let tc: Value<Templated_char_> = Rc::new(RefCell::new(<Templated_char_>::uninit()));
        {
            let __d = tc.as_pointer();
            field!(__d, v).write(0_i8);
        };
        let _dtor_tc = std::mem::DropGuard::new(tc.clone(), |__p| __p.as_pointer().destructor());
        let ti: Value<Templated_int_> = Rc::new(RefCell::new(<Templated_int_>::uninit()));
        {
            let __d = ti.as_pointer();
            field!(__d, v).write(0_i32);
        };
        let _dtor_ti = std::mem::DropGuard::new(ti.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 13));
    {
        let a: Value<Copied> = Rc::new(RefCell::new(<Copied>::uninit()));
        {
            let __d = a.as_pointer();
            field!(__d, v).write(5);
        };
        let _dtor_a = std::mem::DropGuard::new(a.clone(), |__p| __p.as_pointer().destructor());
        let b: Value<Copied> = Rc::new(RefCell::new(<Copied>::uninit()));
        Copied::copy_from(b.as_pointer(), { a.as_pointer() });
        let _dtor_b = std::mem::DropGuard::new(b.clone(), |__p| __p.as_pointer().destructor());
        assert!(({ ({ (*b.borrow()).v } == 5) }));
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 15));
    {
        let o: Value<Ordered> = Rc::new(RefCell::new(<Ordered>::uninit()));
        {
            let __d = o.as_pointer();
            {
                let __d = field_ptr!(__d, first);
                field!(__d, tag).write(1);
            };
            field!(__d, dummy1).write(0);
            {
                let __d = field_ptr!(__d, second);
                field!(__d, tag).write(2);
            };
            field!(__d, dummy2).write(0);
            {
                let __d = field_ptr!(__d, third);
                field!(__d, tag).write(3);
            };
        };
        let _dtor_o = std::mem::DropGuard::new(o.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((order_count_2.with(|rc| *rc.borrow()) == 3));
    assert!(
        (({
            let __idx0 = (0) as usize;
            order_1.with(|rc| rc.borrow()[__idx0])
        }) == 3)
    );
    assert!(
        (({
            let __idx0 = (1) as usize;
            order_1.with(|rc| rc.borrow()[__idx0])
        }) == 2)
    );
    assert!(
        (({
            let __idx0 = (2) as usize;
            order_1.with(|rc| rc.borrow()[__idx0])
        }) == 1)
    );
    let mut before: i32 = global_0.with(|rc| *rc.borrow());
    {
        let cs: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        S::new_1(cs.as_pointer());
        let _dtor_cs = std::mem::DropGuard::new(cs.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == (before + 1)));
    return 0;
}
pub trait ArrayMemberImpl {
    fn destructor(&self);
}
impl ArrayMemberImpl for Ptr<ArrayMember> {
    fn destructor(&self) {
        {
            let __p = array_field_ptr!(self, items);
            for __i in 0..__p.len() {
                SImpl::destructor(&__p.offset(__i as isize));
            }
        }
    }
}
pub trait CopiedImpl {
    fn destructor(&self);
}
impl CopiedImpl for Ptr<Copied> {
    fn destructor(&self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
pub trait DefaultedImpl {
    fn destructor(&self);
}
impl DefaultedImpl for Ptr<Defaulted> {
    fn destructor(&self) {
        field_ptr!(self, s).destructor();
    }
}
pub trait EmptyBodyImpl {
    fn destructor(&self);
}
impl EmptyBodyImpl for Ptr<EmptyBody> {
    fn destructor(&self) {
        field_ptr!(self, s).destructor();
    }
}
pub trait MiddleImpl {
    fn destructor(&self);
}
impl MiddleImpl for Ptr<Middle> {
    fn destructor(&self) {
        field_ptr!(self, s).destructor();
    }
}
pub trait OrderedImpl {
    fn destructor(&self);
}
impl OrderedImpl for Ptr<Ordered> {
    fn destructor(&self) {
        field_ptr!(self, third).destructor();
        field_ptr!(self, second).destructor();
        field_ptr!(self, first).destructor();
    }
}
pub trait OuterImpl {
    fn destructor(&self);
}
impl OuterImpl for Ptr<Outer> {
    fn destructor(&self) {
        field_ptr!(self, m).destructor();
    }
}
pub trait SImpl {
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
pub trait TaggedImpl {
    fn destructor(&self);
}
impl TaggedImpl for Ptr<Tagged> {
    fn destructor(&self) {
        (*order_1.with(Value::clone).borrow_mut())
            [((*order_count_2.with(Value::clone).borrow_mut()).postfix_inc()) as usize] =
            (*self).with(|__s| __s.tag);
    }
}
pub trait Templated_char_Impl {
    fn destructor(&self);
}
impl Templated_char_Impl for Ptr<Templated_char_> {
    fn destructor(&self) {
        global_0.with(|rc| {
            *rc.borrow_mut() = {
                ((global_0.with(|rc| *rc.borrow()) as usize)
                    .wrapping_add((::std::mem::size_of::<i8>() as usize))) as i32
            }
        });
    }
}
pub trait Templated_int_Impl {
    fn destructor(&self);
}
impl Templated_int_Impl for Ptr<Templated_int_> {
    fn destructor(&self) {
        global_0.with(|rc| {
            *rc.borrow_mut() = {
                ((global_0.with(|rc| *rc.borrow()) as usize)
                    .wrapping_add((::std::mem::size_of::<i32>() as usize))) as i32
            }
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = global_0.with(|_| ());
    let _ = order_1.with(|_| ());
    let _ = order_count_2.with(|_| ());
}
