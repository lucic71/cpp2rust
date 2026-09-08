extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Inner {
    pub x: Value<i32>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let __this: Value<Inner> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Inner> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive()]
pub struct Explicit {
    pub v: Value<i32>,
    pub inner: Value<Inner>,
    pub arr: Value<Box<[i32]>>,
}
impl Explicit {
    pub fn Explicit(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Explicit> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
            inner: Rc::new(RefCell::new(Inner {
                x: Rc::new(RefCell::new(((*v.borrow()) * 10))),
            })),
            arr: Rc::new(RefCell::new(Box::new([(*v.borrow()), ((*v.borrow()) + 1)]))),
        }));
        let this: Ptr<Explicit> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Explicit {
    fn clone(&self) -> Self {
        let __this: Value<Explicit> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
            inner: Rc::new(RefCell::new((*self.inner.borrow()).clone())),
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
        }));
        let this: Ptr<Explicit> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Explicit {
    fn default() -> Self {
        Explicit {
            v: <Value<i32>>::default(),
            inner: <Value<Inner>>::default(),
            arr: Rc::new(RefCell::new(
                (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
        }
    }
}
impl ByteRepr for Explicit {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
        (*self.inner.borrow()).to_bytes(&mut buf[4..8]);
        (*self.arr.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            inner: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[4..8]))),
            arr: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive()]
pub struct Implicit {
    pub v: Value<i32>,
    pub inner: Value<Inner>,
    pub arr: Value<Box<[i32]>>,
}
impl Clone for Implicit {
    fn clone(&self) -> Self {
        let __this: Value<Implicit> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
            inner: Rc::new(RefCell::new((*self.inner.borrow()).clone())),
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
        }));
        let this: Ptr<Implicit> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Implicit {
    fn default() -> Self {
        Implicit {
            v: <Value<i32>>::default(),
            inner: <Value<Inner>>::default(),
            arr: Rc::new(RefCell::new(
                (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
        }
    }
}
impl ByteRepr for Implicit {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
        (*self.inner.borrow()).to_bytes(&mut buf[4..8]);
        (*self.arr.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            inner: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[4..8]))),
            arr: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct DefaultCopyUserMove {
    pub v: Value<i32>,
}
impl DefaultCopyUserMove {
    pub fn DefaultCopyUserMove(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<DefaultCopyUserMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn DefaultCopyUserMove_pmutDefaultCopyUserMove(o: Ptr<DefaultCopyUserMove>) -> Self {
        let __this: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
        }));
        let this: Ptr<DefaultCopyUserMove> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for DefaultCopyUserMove {
    fn clone(&self) -> Self {
        let __this: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<DefaultCopyUserMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for DefaultCopyUserMove {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct UserCopyDefaultMove {
    pub v: Value<i32>,
}
impl UserCopyDefaultMove {
    pub fn UserCopyDefaultMove(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<UserCopyDefaultMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn UserCopyDefaultMove_pconstUserCopyDefaultMove(o: Ptr<UserCopyDefaultMove>) -> Self {
        let __this: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(((*(*o.upgrade().deref()).v.borrow()) + 100))),
        }));
        let this: Ptr<UserCopyDefaultMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        let __this: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(((*(*o.upgrade().deref()).v.borrow()) + 100))),
        }));
        let this: Ptr<UserCopyDefaultMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for UserCopyDefaultMove {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn same_0(a: Ptr<Explicit>, b: Ptr<Explicit>) -> bool {
    return ((({
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs == (*(*b.upgrade().deref()).v.borrow())
    }) && ({
        let _lhs = (*(*(*a.upgrade().deref()).inner.borrow()).x.borrow());
        _lhs == (*(*(*b.upgrade().deref()).inner.borrow()).x.borrow())
    })) && ({
        let _lhs = (*(*a.upgrade().deref()).arr.borrow())[(0) as usize];
        _lhs == (*(*b.upgrade().deref()).arr.borrow())[(0) as usize]
    })) && ({
        let _lhs = (*(*a.upgrade().deref()).arr.borrow())[(1) as usize];
        _lhs == (*(*b.upgrade().deref()).arr.borrow())[(1) as usize]
    });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Explicit> = Rc::new(RefCell::new(Explicit::Explicit({ 1 })));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    let b: Value<Explicit> = Rc::new(RefCell::new((*a.borrow()).clone()));
    let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
    let c: Value<Explicit> = Rc::new(RefCell::new((*a.borrow()).clone()));
    let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
    let d: Value<Explicit> = Rc::new(RefCell::new((*a.borrow_mut())));
    let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
    assert!(
        (({ same_0(b.as_pointer(), a.as_pointer(),) })
            && ({ same_0(c.as_pointer(), a.as_pointer(),) }))
            && ({ same_0(d.as_pointer(), a.as_pointer(),) })
    );
    let e: Value<Explicit> = Rc::new(RefCell::new(Explicit::Explicit({ 2 })));
    let _dtor_e = ScopedDestructor::new(&e, |__p| __p.destructor());
    let f: Value<Explicit> = Rc::new(RefCell::new(Explicit::Explicit({ 3 })));
    let _dtor_f = ScopedDestructor::new(&f, |__p| __p.destructor());
    (*e.borrow_mut()) = (*b.borrow()).clone();
    (*f.borrow_mut()) = (*c.borrow());
    assert!(
        ({ same_0(e.as_pointer(), b.as_pointer(),) })
            && ({ same_0(f.as_pointer(), c.as_pointer(),) })
    );
    let g: Value<Explicit> = Rc::new(RefCell::new(Explicit::Explicit({ 4 })));
    let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
    (*g.borrow_mut()) = {
        (*e.borrow_mut()) = (*f.borrow()).clone();
        (*e.borrow()).clone()
    };
    assert!(
        ({ same_0(g.as_pointer(), f.as_pointer(),) })
            && ({ same_0(e.as_pointer(), f.as_pointer(),) })
    );
    let i: Value<Implicit> = Rc::new(RefCell::new(Implicit {
        v: Rc::new(RefCell::new(5)),
        inner: Rc::new(RefCell::new(Inner {
            x: Rc::new(RefCell::new(50)),
        })),
        arr: Rc::new(RefCell::new(Box::new([5, 6]))),
    }));
    let j: Value<Implicit> = Rc::new(RefCell::new((*i.borrow()).clone()));
    let k: Value<Implicit> = Rc::new(RefCell::new((*i.borrow_mut())));
    assert!(
        (((*(*j.borrow()).v.borrow()) == 5)
            && ((*(*(*j.borrow()).inner.borrow()).x.borrow()) == 50))
            && ((*(*j.borrow()).arr.borrow())[(1) as usize] == 6)
    );
    assert!(((*(*i.borrow()).v.borrow()) == 5) && ((*(*k.borrow()).v.borrow()) == 5));
    let l: Value<Implicit> = Rc::new(RefCell::new(Implicit {
        v: Rc::new(RefCell::new(0)),
        inner: Rc::new(RefCell::new(Inner {
            x: Rc::new(RefCell::new(0)),
        })),
        arr: Rc::new(RefCell::new(Box::new([0, 0]))),
    }));
    (*l.borrow_mut()) = (*j.borrow()).clone();
    assert!(
        (((*(*l.borrow()).v.borrow()) == 5)
            && ((*(*(*l.borrow()).inner.borrow()).x.borrow()) == 50))
            && ((*(*l.borrow()).arr.borrow())[(0) as usize] == 5)
    );
    let vec_: Value<Vec<Explicit>> = Rc::new(RefCell::new(Vec::new()));
    {
        let a0_clone = (*b.borrow()).clone();
        (*vec_.borrow_mut()).push(a0_clone)
    };
    (*vec_.borrow_mut()).push(Explicit::Explicit({ 9 }));
    assert!(
        ((*(*(vec_.as_pointer() as Ptr<Explicit>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .v
        .borrow())
            == 1)
            && ((*(*(vec_.as_pointer() as Ptr<Explicit>)
                .offset(1_usize)
                .upgrade()
                .deref())
            .v
            .borrow())
                == 9)
    );
    let m: Value<DefaultCopyUserMove> =
        Rc::new(RefCell::new(DefaultCopyUserMove::DefaultCopyUserMove({
            7
        })));
    let m1: Value<DefaultCopyUserMove> = Rc::new(RefCell::new((*m.borrow()).clone()));
    let m2: Value<DefaultCopyUserMove> = Rc::new(RefCell::new((*m.borrow_mut())));
    assert!(
        (((*(*m1.borrow()).v.borrow()) == 7) && ((*(*m2.borrow()).v.borrow()) == 7))
            && ((*(*m.borrow()).v.borrow()) == 0)
    );
    let m3: Value<DefaultCopyUserMove> =
        Rc::new(RefCell::new(DefaultCopyUserMove::DefaultCopyUserMove({
            1
        })));
    let m4: Value<DefaultCopyUserMove> =
        Rc::new(RefCell::new(DefaultCopyUserMove::DefaultCopyUserMove({
            1
        })));
    (*m3.borrow_mut()) = (*m1.borrow()).clone();
    ({
        DefaultCopyUserMoveImpl::operator_assign_pmutDefaultCopyUserMove(
            &m4.as_pointer(),
            m1.as_pointer(),
        )
    });
    assert!(
        (((*(*m3.borrow()).v.borrow()) == 7) && ((*(*m4.borrow()).v.borrow()) == 7))
            && ((*(*m1.borrow()).v.borrow()) == 0)
    );
    let u: Value<UserCopyDefaultMove> =
        Rc::new(RefCell::new(UserCopyDefaultMove::UserCopyDefaultMove({
            8
        })));
    let u1: Value<UserCopyDefaultMove> = Rc::new(RefCell::new((*u.borrow()).clone()));
    let u2: Value<UserCopyDefaultMove> = Rc::new(RefCell::new((*u.borrow_mut())));
    assert!(
        (((*(*u1.borrow()).v.borrow()) == 108) && ((*(*u2.borrow()).v.borrow()) == 8))
            && ((*(*u.borrow()).v.borrow()) == 8)
    );
    let u3: Value<UserCopyDefaultMove> =
        Rc::new(RefCell::new(UserCopyDefaultMove::UserCopyDefaultMove({
            1
        })));
    let u4: Value<UserCopyDefaultMove> =
        Rc::new(RefCell::new(UserCopyDefaultMove::UserCopyDefaultMove({
            1
        })));
    ({
        UserCopyDefaultMoveImpl::operator_assign_pconstUserCopyDefaultMove(
            &u3.as_pointer(),
            u2.as_pointer(),
        )
    });
    (*u4.borrow_mut()) = (*u2.borrow());
    assert!(((*(*u3.borrow()).v.borrow()) == 108) && ((*(*u4.borrow()).v.borrow()) == 8));
    return 0;
}
pub trait DefaultCopyUserMoveImpl {
    fn operator_assign_pmutDefaultCopyUserMove(
        &self,
        o: Ptr<DefaultCopyUserMove>,
    ) -> Ptr<DefaultCopyUserMove>;
}
impl DefaultCopyUserMoveImpl for Ptr<DefaultCopyUserMove> {
    fn operator_assign_pmutDefaultCopyUserMove(
        &self,
        o: Ptr<DefaultCopyUserMove>,
    ) -> Ptr<DefaultCopyUserMove> {
        let __rhs = (*(*o.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = __rhs;
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        return (*self).clone();
    }
}
pub trait ExplicitImpl {
    fn destructor(&self);
}
impl ExplicitImpl for Ptr<Explicit> {
    fn destructor(&self) {}
}
pub trait UserCopyDefaultMoveImpl {
    fn operator_assign_pconstUserCopyDefaultMove(
        &self,
        o: Ptr<UserCopyDefaultMove>,
    ) -> Ptr<UserCopyDefaultMove>;
}
impl UserCopyDefaultMoveImpl for Ptr<UserCopyDefaultMove> {
    fn operator_assign_pconstUserCopyDefaultMove(
        &self,
        o: Ptr<UserCopyDefaultMove>,
    ) -> Ptr<UserCopyDefaultMove> {
        let __rhs = ((*(*o.upgrade().deref()).v.borrow()) + 100);
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
