extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
}
impl Inner {
    pub fn copy_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
    pub fn move_from(this: Ptr<Inner>, _a0: Ptr<Inner>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
    }
    pub fn new_1(this: Ptr<Inner>) {}
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(i32, new, value)]
#[byte_size(16)]
pub struct Explicit {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub inner: Inner,
    #[offset(8)]
    #[byte_size(8)]
    #[array_len(2)]
    pub arr: Box<[i32]>,
}
impl Explicit {
    pub fn new(this: Ptr<Explicit>, mut v: i32) {
        field!(this, v).write(v);
        {
            let __d = field_ptr!(this, inner);
            field!(__d, x).write((v * 10));
        };
        field!(this, arr).write(Box::new([v, (v + 1)]));
    }
    pub fn copy_from(this: Ptr<Explicit>, _a0: Ptr<Explicit>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
        Inner::copy_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0.upgrade().deref()).arr[(__i) as usize]
        })));
    }
    pub fn move_from(this: Ptr<Explicit>, _a0: Ptr<Explicit>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
        Inner::move_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0.upgrade().deref()).arr[(__i) as usize]
        })));
    }
}
impl Default for Explicit {
    fn default() -> Self {
        {
            Explicit {
                v: 0_i32,
                inner: <Inner>::default(),
                arr: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Implicit {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub inner: Inner,
    #[offset(8)]
    #[byte_size(8)]
    #[array_len(2)]
    pub arr: Box<[i32]>,
}
impl Implicit {
    pub fn new_1(this: Ptr<Implicit>) {
        Inner::new_1(field_ptr!(this, inner));
    }
    pub fn copy_from(this: Ptr<Implicit>, _a0: Ptr<Implicit>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
        Inner::copy_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0.upgrade().deref()).arr[(__i) as usize]
        })));
    }
    pub fn move_from(this: Ptr<Implicit>, _a0: Ptr<Implicit>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
        Inner::move_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0.upgrade().deref()).arr[(__i) as usize]
        })));
    }
}
impl Default for Implicit {
    fn default() -> Self {
        {
            Implicit {
                v: 0_i32,
                inner: <Inner>::default(),
                arr: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
#[derive(
    Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, MoveAssign, Default,
)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct DefaultCopyUserMove {
    #[offset(0)]
    pub v: i32,
}
impl DefaultCopyUserMove {
    pub fn new(this: Ptr<DefaultCopyUserMove>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn copy_from(this: Ptr<DefaultCopyUserMove>, _a0: Ptr<DefaultCopyUserMove>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<DefaultCopyUserMove>, o: Ptr<DefaultCopyUserMove>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(o, v).write(0);
    }
}
#[derive(
    Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, CopyAssign, MoveAssign, Default,
)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct UserCopyDefaultMove {
    #[offset(0)]
    pub v: i32,
}
impl UserCopyDefaultMove {
    pub fn new(this: Ptr<UserCopyDefaultMove>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn copy_from(this: Ptr<UserCopyDefaultMove>, o: Ptr<UserCopyDefaultMove>) {
        field!(this, v).write((o.with(|__s| __s.v) + 100));
    }
    pub fn move_from(this: Ptr<UserCopyDefaultMove>, _a0: Ptr<UserCopyDefaultMove>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        let __src: Value<UserCopyDefaultMove> =
            Rc::new(RefCell::new(UserCopyDefaultMove { v: self.v.clone() }));
        let __dst: Value<UserCopyDefaultMove> =
            Rc::new(RefCell::new(<UserCopyDefaultMove>::uninit()));
        UserCopyDefaultMove::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, value)]
#[byte_size(64)]
pub struct Buffer {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Vec<i32>,
    #[offset(24)]
    #[byte_size(24)]
    pub rows: Vec<Vec<i32>>,
    #[offset(48)]
    pub n: i32,
    #[offset(52)]
    #[byte_size(8)]
    #[array_len(2)]
    pub arr: Box<[i32]>,
}
impl Buffer {
    pub fn new(this: Ptr<Buffer>, mut n: i32) {
        field!(this, data).write(vec![n; (n as usize)]);
        field!(this, rows).write(Vec::new());
        field!(this, n).write(n);
        field!(this, arr).write(Box::new([n, (n + 1)]));
        {
            let a0_clone = (*this.upgrade().deref()).data.clone();
            field!(this, rows).with_mut(|__v: &mut Vec<Vec<i32>>| __v.push(a0_clone))
        };
    }
    pub fn move_from(this: Ptr<Buffer>, _a0: Ptr<Buffer>) {
        field!(this, data).write(field!(_a0, data).with_mut(std::mem::take));
        field!(this, rows).write(field!(_a0, rows).with_mut(std::mem::take));
        field!(this, n).write({ (*_a0.upgrade().deref()).n });
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0.upgrade().deref()).arr[(__i) as usize]
        })));
    }
}
impl Default for Buffer {
    fn default() -> Self {
        {
            Buffer {
                data: Default::default(),
                rows: Default::default(),
                n: 0_i32,
                arr: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(48)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Vec<i32>,
    #[offset(24)]
    pub n: i32,
    #[offset(28)]
    #[byte_size(8)]
    #[array_len(2)]
    pub arr: Box<[i32]>,
    #[offset(40)]
    #[byte_size(8)]
    pub p: Option<Value<i32>>,
}
impl Owner {
    pub fn move_from(this: Ptr<Owner>, _a0: Ptr<Owner>) {
        field!(this, data).write(field!(_a0, data).with_mut(std::mem::take));
        field!(this, n).write({ (*_a0.upgrade().deref()).n });
        field!(this, arr).write(Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0.upgrade().deref()).arr[(__i) as usize]
        })));
        field!(this, p).write(field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
    }
    pub fn new_1(this: Ptr<Owner>) {
        field!(this, data).write(Vec::new());
        field!(this, p).write(None);
    }
}
impl Default for Owner {
    fn default() -> Self {
        {
            Owner {
                data: Default::default(),
                n: 0_i32,
                arr: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
                p: None,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Copyable {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Vec<i32>,
}
impl Copyable {
    pub fn copy_from(this: Ptr<Copyable>, _a0: Ptr<Copyable>) {
        field!(this, data).write((*_a0.upgrade().deref()).data.clone());
    }
    pub fn move_from(this: Ptr<Copyable>, _a0: Ptr<Copyable>) {
        field!(this, data).write(field!(_a0, data).with_mut(std::mem::take));
    }
    pub fn new_1(this: Ptr<Copyable>) {
        field!(this, data).write(Vec::new());
    }
}
#[derive(
    Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, MoveAssign, Destructor, Default,
)]
#[cpp_from(i32, new, value)]
#[byte_size(32)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub inner: Inner,
    #[offset(4)]
    #[byte_size(16)]
    pub e: Explicit,
    #[offset(24)]
    #[byte_size(8)]
    pub p: Option<Value<i32>>,
}
impl Holder {
    pub fn new(this: Ptr<Holder>, mut v: i32) {
        {
            let __d = field_ptr!(this, inner);
            field!(__d, x).write(v);
        };
        Explicit::new(field_ptr!(this, e), { v });
        field!(this, p).write(None);
    }
    pub fn move_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        Inner::move_from(field_ptr!(this, inner), { field_ptr!(_a0, inner) });
        Explicit::move_from(field_ptr!(this, e), { field_ptr!(_a0, e) });
        field!(this, p).write(field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
    }
}
pub fn same_0(a: Ptr<Explicit>, b: Ptr<Explicit>) -> bool {
    return ((({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) })
        && ({ a.with(|__s| __s.inner.x) } == { b.with(|__s| __s.inner.x) }))
        && ({ (array_elem!(a, arr, 0).read()) } == { (array_elem!(b, arr, 0).read()) }))
        && ({ (array_elem!(a, arr, 1).read()) } == { (array_elem!(b, arr, 1).read()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Explicit> = Rc::new(RefCell::new(<Explicit>::uninit()));
    Explicit::new(a.as_pointer(), { 1 });
    let _dtor_a = std::mem::DropGuard::new(a.clone(), |__p| __p.as_pointer().destructor());
    let b: Value<Explicit> = Rc::new(RefCell::new(<Explicit>::uninit()));
    Explicit::copy_from(b.as_pointer(), { a.as_pointer() });
    let _dtor_b = std::mem::DropGuard::new(b.clone(), |__p| __p.as_pointer().destructor());
    let c: Value<Explicit> = Rc::new(RefCell::new(<Explicit>::uninit()));
    Explicit::copy_from(c.as_pointer(), { a.as_pointer() });
    let _dtor_c = std::mem::DropGuard::new(c.clone(), |__p| __p.as_pointer().destructor());
    let d: Value<Explicit> = Rc::new(RefCell::new(<Explicit>::uninit()));
    Explicit::move_from(d.as_pointer(), { a.as_pointer() });
    let _dtor_d = std::mem::DropGuard::new(d.clone(), |__p| __p.as_pointer().destructor());
    assert!(
        (({ same_0(b.as_pointer(), a.as_pointer(),) })
            && ({ same_0(c.as_pointer(), a.as_pointer(),) }))
            && ({ same_0(d.as_pointer(), a.as_pointer(),) })
    );
    let e: Value<Explicit> = Rc::new(RefCell::new(<Explicit>::uninit()));
    Explicit::new(e.as_pointer(), { 2 });
    let _dtor_e = std::mem::DropGuard::new(e.clone(), |__p| __p.as_pointer().destructor());
    let f: Value<Explicit> = Rc::new(RefCell::new(<Explicit>::uninit()));
    Explicit::new(f.as_pointer(), { 3 });
    let _dtor_f = std::mem::DropGuard::new(f.clone(), |__p| __p.as_pointer().destructor());
    (*e.borrow_mut()) = (*b.borrow()).clone();
    (*f.borrow_mut()) = (*c.borrow()).clone();
    assert!(
        ({ same_0(e.as_pointer(), b.as_pointer(),) })
            && ({ same_0(f.as_pointer(), c.as_pointer(),) })
    );
    let g: Value<Explicit> = Rc::new(RefCell::new(<Explicit>::uninit()));
    Explicit::new(g.as_pointer(), { 4 });
    let _dtor_g = std::mem::DropGuard::new(g.clone(), |__p| __p.as_pointer().destructor());
    (*g.borrow_mut()) = {
        (*e.borrow_mut()) = (*f.borrow()).clone();
        (*e.borrow()).clone()
    };
    assert!(
        ({ same_0(g.as_pointer(), f.as_pointer(),) })
            && ({ same_0(e.as_pointer(), f.as_pointer(),) })
    );
    let i: Value<Implicit> = Rc::new(RefCell::new(<Implicit>::uninit()));
    {
        let __d = i.as_pointer();
        field!(__d, v).write(5);
        {
            let __d = field_ptr!(__d, inner);
            field!(__d, x).write(50);
        };
        field!(__d, arr).write(Box::new([5, 6]));
    };
    let j: Value<Implicit> = Rc::new(RefCell::new(<Implicit>::uninit()));
    Implicit::copy_from(j.as_pointer(), { i.as_pointer() });
    let k: Value<Implicit> = Rc::new(RefCell::new(<Implicit>::uninit()));
    Implicit::move_from(k.as_pointer(), { i.as_pointer() });
    assert!(
        ({
            (({ (*j.borrow()).v } == 5) && ({ (*j.borrow()).inner.x } == 50))
                && ((array_elem!(j.as_pointer(), arr, 1).read()) == 6)
        })
    );
    assert!(({ ({ (*i.borrow()).v } == 5) && ({ (*k.borrow()).v } == 5) }));
    let l: Value<Implicit> = Rc::new(RefCell::new(<Implicit>::uninit()));
    {
        let __d = l.as_pointer();
        field!(__d, v).write(0);
        {
            let __d = field_ptr!(__d, inner);
            field!(__d, x).write(0);
        };
        field!(__d, arr).write(Box::new([0, 0]));
    };
    (*l.borrow_mut()) = (*j.borrow()).clone();
    assert!(
        ({
            (({ (*l.borrow()).v } == 5) && ({ (*l.borrow()).inner.x } == 50))
                && ((array_elem!(l.as_pointer(), arr, 0).read()) == 5)
        })
    );
    let mut vec_: Vec<Explicit> = Vec::new();
    {
        let a0_clone = (*b.borrow()).clone();
        vec_.push(a0_clone)
    };
    {
        let __a1 = Rc::try_unwrap({
            let __v: Value<Explicit> = Rc::new(RefCell::new(<Explicit>::uninit()));
            Explicit::new(__v.as_pointer(), { 9 });
            __v
        })
        .ok()
        .unwrap()
        .into_inner();
        vec_.push(__a1)
    };
    assert!((vec_[0_usize].v == 1) && (vec_[1_usize].v == 9));
    let m: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(<DefaultCopyUserMove>::uninit()));
    DefaultCopyUserMove::new(m.as_pointer(), { 7 });
    let m1: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(<DefaultCopyUserMove>::uninit()));
    DefaultCopyUserMove::copy_from(m1.as_pointer(), { m.as_pointer() });
    let m2: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(<DefaultCopyUserMove>::uninit()));
    DefaultCopyUserMove::move_from(m2.as_pointer(), { m.as_pointer() });
    assert!(
        ({
            (({ (*m1.borrow()).v } == 7) && ({ (*m2.borrow()).v } == 7))
                && ({ (*m.borrow()).v } == 0)
        })
    );
    let m3: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(<DefaultCopyUserMove>::uninit()));
    DefaultCopyUserMove::new(m3.as_pointer(), { 1 });
    let m4: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(<DefaultCopyUserMove>::uninit()));
    DefaultCopyUserMove::new(m4.as_pointer(), { 1 });
    (*m3.borrow_mut()) = (*m1.borrow());
    ({ DefaultCopyUserMoveImpl::move_assign(&m4.as_pointer(), m1.as_pointer()) });
    assert!(
        ({
            (({ (*m3.borrow()).v } == 7) && ({ (*m4.borrow()).v } == 7))
                && ({ (*m1.borrow()).v } == 0)
        })
    );
    let u: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(<UserCopyDefaultMove>::uninit()));
    UserCopyDefaultMove::new(u.as_pointer(), { 8 });
    let u1: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(<UserCopyDefaultMove>::uninit()));
    UserCopyDefaultMove::copy_from(u1.as_pointer(), { u.as_pointer() });
    let u2: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(<UserCopyDefaultMove>::uninit()));
    UserCopyDefaultMove::move_from(u2.as_pointer(), { u.as_pointer() });
    assert!(
        ({
            (({ (*u1.borrow()).v } == 108) && ({ (*u2.borrow()).v } == 8))
                && ({ (*u.borrow()).v } == 8)
        })
    );
    let u3: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(<UserCopyDefaultMove>::uninit()));
    UserCopyDefaultMove::new(u3.as_pointer(), { 1 });
    let u4: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(<UserCopyDefaultMove>::uninit()));
    UserCopyDefaultMove::new(u4.as_pointer(), { 1 });
    ({ UserCopyDefaultMoveImpl::copy_assign(&u3.as_pointer(), u2.as_pointer()) });
    ({ UserCopyDefaultMoveImpl::move_assign(&u4.as_pointer(), u2.as_pointer()) });
    assert!(({ ({ (*u3.borrow()).v } == 108) && ({ (*u4.borrow()).v } == 8) }));
    let p: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
    Buffer::new(p.as_pointer(), { 3 });
    let q: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
    Buffer::move_from(q.as_pointer(), { p.as_pointer() });
    assert!(
        ({
            ((({ (*q.borrow()).n } == 3) && ((*q.borrow()).data.len() == 3_usize))
                && ((vec_elem!(q.as_pointer(), data, 2_usize).read()) == 3))
                && ((*p.borrow()).data.is_empty())
        })
    );
    let r: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
    Buffer::new(r.as_pointer(), { 1 });
    ({ BufferImpl::move_assign(&r.as_pointer(), q.as_pointer()) });
    assert!(
        ({
            ((({ (*r.borrow()).n } == 3) && ((*r.borrow()).data.len() == 3_usize))
                && ((array_elem!(r.as_pointer(), arr, 1).read()) == 4))
                && ((*q.borrow()).data.is_empty())
        })
    );
    assert!(
        ({
            (((*r.borrow()).rows.len() == 1_usize)
                && ((*field_ref!(r.as_pointer(), rows)
                    .decay()
                    .offset(0_usize)
                    .upgrade()
                    .deref())
                .len()
                    == 3_usize))
                && ((*q.borrow()).rows.is_empty())
        })
    );
    let bufs: Value<Vec<Buffer>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = Rc::try_unwrap({
            let __v: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
            Buffer::move_from(__v.as_pointer(), { r.as_pointer() });
            __v
        })
        .ok()
        .unwrap()
        .into_inner();
        (*bufs.borrow_mut()).push(__a1)
    };
    {
        let mut __tmp_2 = {
            Rc::try_unwrap({
                let __v: Value<Buffer> = Rc::new(RefCell::new(<Buffer>::uninit()));
                Buffer::move_from(__v.as_pointer(), { bufs.decay().offset(0_usize) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner()
        };
        (*bufs.borrow_mut()).push(__tmp_2);
    };
    assert!(
        ({
            (({ (*bufs.borrow())[1_usize].n } == 3)
                && ((*bufs.borrow())[1_usize].data.len() == 3_usize))
                && ((*bufs.borrow())[0_usize].data.is_empty())
        })
    );
    let o1: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
    Owner::new_1(o1.as_pointer());
    {
        let __a1 = 5;
        (*o1.borrow_mut()).data.push(__a1)
    };
    (*o1.borrow_mut()).n = 5;
    array_elem!(o1.as_pointer(), arr, 0).write(5);
    array_elem!(o1.as_pointer(), arr, 1).write(6);
    {
        let _p: Ptr<_> = Ptr::alloc(7);
        field_ptr!(o1.as_pointer(), p).write(_p.to_owned_opt())
    };
    let o2: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
    Owner::move_from(o2.as_pointer(), { o1.as_pointer() });
    assert!(
        ({
            ((({ (*o2.borrow()).n } == 5) && ((*o2.borrow()).data.len() == 1_usize))
                && ((array_elem!(o2.as_pointer(), arr, 1).read()) == 6))
                && ((({ { (*o2.borrow()).p.clone() } }).as_pointer().read()) == 7)
        })
    );
    assert!(
        ({
            ((*o1.borrow()).data.is_empty())
                && ((({ { (*o1.borrow()).p.clone() } }).as_pointer()).is_null())
        })
    );
    let o3: Value<Owner> = Rc::new(RefCell::new(<Owner>::uninit()));
    Owner::new_1(o3.as_pointer());
    ({ OwnerImpl::move_assign(&o3.as_pointer(), o2.as_pointer()) });
    assert!(
        ({
            ((({ (*o3.borrow()).n } == 5)
                && ((vec_elem!(o3.as_pointer(), data, 0_usize).read()) == 5))
                && ((array_elem!(o3.as_pointer(), arr, 0).read()) == 5))
                && ((({ { (*o3.borrow()).p.clone() } }).as_pointer().read()) == 7)
        })
    );
    assert!(
        ({
            ((*o2.borrow()).data.is_empty())
                && ((({ { (*o2.borrow()).p.clone() } }).as_pointer()).is_null())
        })
    );
    let c1: Value<Copyable> = Rc::new(RefCell::new(<Copyable>::uninit()));
    {
        let __d = c1.as_pointer();
        field!(__d, data).write(Vec::from([1, 2]));
    };
    let c2: Value<Copyable> = Rc::new(RefCell::new(<Copyable>::uninit()));
    Copyable::copy_from(c2.as_pointer(), { c1.as_pointer() });
    let c3: Value<Copyable> = Rc::new(RefCell::new(<Copyable>::uninit()));
    Copyable::move_from(c3.as_pointer(), { c1.as_pointer() });
    assert!(({ ((*c2.borrow()).data.len() == 2_usize) && ((*c3.borrow()).data.len() == 2_usize) }));
    assert!(({ (*c1.borrow()).data.is_empty() }));
    let c4: Value<Copyable> = Rc::new(RefCell::new(<Copyable>::uninit()));
    Copyable::new_1(c4.as_pointer());
    ({ CopyableImpl::move_assign(&c4.as_pointer(), c3.as_pointer()) });
    assert!(
        ({
            ((vec_elem!(c4.as_pointer(), data, 1_usize).read()) == 2)
                && ((*c3.borrow()).data.is_empty())
        })
    );
    let h1: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    Holder::new(h1.as_pointer(), { 4 });
    let _dtor_h1 = std::mem::DropGuard::new(h1.clone(), |__p| __p.as_pointer().destructor());
    {
        let _p: Ptr<_> = Ptr::alloc(9);
        field_ptr!(h1.as_pointer(), p).write(_p.to_owned_opt())
    };
    let h2: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    Holder::move_from(h2.as_pointer(), { h1.as_pointer() });
    let _dtor_h2 = std::mem::DropGuard::new(h2.clone(), |__p| __p.as_pointer().destructor());
    assert!(
        ({
            ((({ (*h2.borrow()).inner.x } == 4) && ({ (*h2.borrow()).e.v } == 4))
                && ((({ { (*h2.borrow()).p.clone() } }).as_pointer().read()) == 9))
                && ((({ { (*h1.borrow()).p.clone() } }).as_pointer()).is_null())
        })
    );
    let h3: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    Holder::new(h3.as_pointer(), { 1 });
    let _dtor_h3 = std::mem::DropGuard::new(h3.clone(), |__p| __p.as_pointer().destructor());
    ({ HolderImpl::move_assign(&h3.as_pointer(), h2.as_pointer()) });
    assert!(
        ({
            ((({ (*h3.borrow()).inner.x } == 4)
                && ((array_elem!(field_ptr!(h3.as_pointer(), e), arr, 1).read()) == 5))
                && ((({ { (*h3.borrow()).p.clone() } }).as_pointer().read()) == 9))
                && ((({ { (*h2.borrow()).p.clone() } }).as_pointer()).is_null())
        })
    );
    return 0;
}
pub trait BufferImpl {
    fn move_assign(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn move_assign(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer> {
        field!((*self), data).with_mut(|__b0| {
            (*__b0) = field!(_a0, data).with_mut(std::mem::take);
        });
        field!((*self), rows).with_mut(|__b0| {
            (*__b0) = field!(_a0, rows).with_mut(std::mem::take);
        });
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        {
            let mut __tmp_0 = { (array_field_ptr!((*self), arr)).to_any() };
            __tmp_0.memcpy(&(array_field_ptr!(_a0, arr)).to_any(), 8_usize);
            (__tmp_0).clone()
        };
        return (*self).clone();
    }
}
pub trait CopyableImpl {
    fn move_assign(&self, _a0: Ptr<Copyable>) -> Ptr<Copyable>;
}
impl CopyableImpl for Ptr<Copyable> {
    fn move_assign(&self, _a0: Ptr<Copyable>) -> Ptr<Copyable> {
        field!((*self), data).with_mut(|__b0| {
            (*__b0) = field!(_a0, data).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub trait DefaultCopyUserMoveImpl {
    fn move_assign(&self, o: Ptr<DefaultCopyUserMove>) -> Ptr<DefaultCopyUserMove>;
}
impl DefaultCopyUserMoveImpl for Ptr<DefaultCopyUserMove> {
    fn move_assign(&self, o: Ptr<DefaultCopyUserMove>) -> Ptr<DefaultCopyUserMove> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0);
        return (*self).clone();
    }
}
pub trait ExplicitImpl {
    fn destructor(&self);
}
impl ExplicitImpl for Ptr<Explicit> {
    fn destructor(&self) {}
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
    fn destructor(&self);
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        field!((*self), inner).write({ { (*_a0.upgrade().deref()).inner } });
        field!((*self), e).write({ { (*_a0.upgrade().deref()).e.clone() } });
        field!((*self), p).with_mut(|__b0| {
            (*__b0) = field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take());
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, e).destructor();
    }
}
pub trait OwnerImpl {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner>;
}
impl OwnerImpl for Ptr<Owner> {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner> {
        field!((*self), data).with_mut(|__b0| {
            (*__b0) = field!(_a0, data).with_mut(std::mem::take);
        });
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        {
            let mut __tmp_1 = { (array_field_ptr!((*self), arr)).to_any() };
            __tmp_1.memcpy(&(array_field_ptr!(_a0, arr)).to_any(), 8_usize);
            (__tmp_1).clone()
        };
        field!((*self), p).with_mut(|__b0| {
            (*__b0) = field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take());
        });
        return (*self).clone();
    }
}
pub trait UserCopyDefaultMoveImpl {
    fn copy_assign(&self, o: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove>;
    fn move_assign(&self, _a0: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove>;
}
impl UserCopyDefaultMoveImpl for Ptr<UserCopyDefaultMove> {
    fn copy_assign(&self, o: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove> {
        field!((*self), v).write({ (o.with(|__s| __s.v) + 100) });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove> {
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
