extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub x: i32,
}
impl Inner {
    pub unsafe fn copy_from(this: *mut Inner, _a0: *const Inner) {
        (&raw mut (*this).x).write((*_a0).x);
    }
    pub unsafe fn move_from(this: *mut Inner, _a0: *mut Inner) {
        (&raw mut (*this).x).write((*_a0).x);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Explicit {
    pub v: i32,
    pub inner: Inner,
    pub arr: [i32; 2],
}
impl Explicit {
    pub unsafe fn new(this: *mut Explicit, mut v: i32) {
        (&raw mut (*this).v).write(v);
        {
            let __d = (&raw mut (*this).inner);
            (&raw mut (*__d).x).write(((v) * (10)));
        };
        (&raw mut (*this).arr).write([v, ((v) + (1))]);
    }
    pub unsafe fn copy_from(this: *mut Explicit, _a0: *const Explicit) {
        (&raw mut (*this).v).write((*_a0).v);
        Inner::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
        (&raw mut (*this).arr).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).arr[(__i)]
        }));
    }
    pub unsafe fn move_from(this: *mut Explicit, _a0: *mut Explicit) {
        (&raw mut (*this).v).write((*_a0).v);
        Inner::move_from((&raw mut (*this).inner), { &mut (*_a0).inner });
        (&raw mut (*this).arr).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).arr[(__i)]
        }));
    }
    pub unsafe fn destructor(&mut self) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Implicit {
    pub v: i32,
    pub inner: Inner,
    pub arr: [i32; 2],
}
impl Implicit {
    pub unsafe fn copy_from(this: *mut Implicit, _a0: *const Implicit) {
        (&raw mut (*this).v).write((*_a0).v);
        Inner::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
        (&raw mut (*this).arr).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).arr[(__i)]
        }));
    }
    pub unsafe fn move_from(this: *mut Implicit, _a0: *mut Implicit) {
        (&raw mut (*this).v).write((*_a0).v);
        Inner::move_from((&raw mut (*this).inner), { &mut (*_a0).inner });
        (&raw mut (*this).arr).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).arr[(__i)]
        }));
    }
}
#[repr(C)]
#[derive(
    Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, Default,
)]
#[cpp_from(i32, new, value)]
pub struct DefaultCopyUserMove {
    pub v: i32,
}
impl DefaultCopyUserMove {
    pub unsafe fn new(this: *mut DefaultCopyUserMove, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut DefaultCopyUserMove, _a0: *const DefaultCopyUserMove) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut DefaultCopyUserMove, o: *mut DefaultCopyUserMove) {
        (&raw mut (*this).v).write((*o).v);
        (*o).v = 0;
    }
    pub unsafe fn move_assign(&mut self, o: *mut DefaultCopyUserMove) -> *mut DefaultCopyUserMove {
        self.v = (*o).v;
        (*o).v = 0;
        return &mut (*(self as *mut DefaultCopyUserMove));
    }
}
#[repr(C)]
#[derive(
    VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default,
)]
#[cpp_from(i32, new, value)]
pub struct UserCopyDefaultMove {
    pub v: i32,
}
impl UserCopyDefaultMove {
    pub unsafe fn new(this: *mut UserCopyDefaultMove, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut UserCopyDefaultMove, o: *const UserCopyDefaultMove) {
        (&raw mut (*this).v).write((((*o).v) + (100)));
    }
    pub unsafe fn move_from(this: *mut UserCopyDefaultMove, _a0: *mut UserCopyDefaultMove) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn copy_assign(
        &mut self,
        o: *const UserCopyDefaultMove,
    ) -> *mut UserCopyDefaultMove {
        self.v = (((*o).v) + (100));
        return &mut (*(self as *mut UserCopyDefaultMove));
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut UserCopyDefaultMove,
    ) -> *mut UserCopyDefaultMove {
        self.v = (*_a0).v;
        return &mut (*(self as *mut UserCopyDefaultMove));
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: UserCopyDefaultMove = ::std::mem::MaybeUninit::uninit().assume_init();
            UserCopyDefaultMove::copy_from(&raw mut __v, self as *const UserCopyDefaultMove);
            __v
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Buffer {
    pub data: Vec<i32>,
    pub rows: Vec<Vec<i32>>,
    pub n: i32,
    pub arr: [i32; 2],
}
impl Buffer {
    pub unsafe fn new(this: *mut Buffer, mut n: i32) {
        (&raw mut (*this).data).write(vec![n; (n as usize)]);
        (&raw mut (*this).rows).write(Vec::new());
        (&raw mut (*this).n).write(n);
        (&raw mut (*this).arr).write([n, ((n) + (1))]);
        {
            let a0_clone = (*this).data.clone();
            (*this).rows.push(a0_clone)
        };
    }
    pub unsafe fn move_from(this: *mut Buffer, _a0: *mut Buffer) {
        (&raw mut (*this).data).write(std::mem::take(&mut (*_a0).data));
        (&raw mut (*this).rows).write(std::mem::take(&mut (*_a0).rows));
        (&raw mut (*this).n).write((*_a0).n);
        (&raw mut (*this).arr).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).arr[(__i)]
        }));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Buffer) -> *mut Buffer {
        {
            self.data = std::mem::take(&mut (*_a0).data);
        };
        {
            self.rows = std::mem::take(&mut (*_a0).rows);
        };
        self.n = (*_a0).n;
        {
            let mut __tmp_0 = { ((&mut self.arr as *mut [i32; 2]) as *mut ::libc::c_void) };
            let mut __tmp_1 = { ((&mut (*_a0).arr as *mut [i32; 2]) as *const ::libc::c_void) };
            if 8_usize != 0 {
                ::std::ptr::copy_nonoverlapping(__tmp_1, __tmp_0, 8_usize)
            }
            __tmp_0
        };
        return &mut (*(self as *mut Buffer));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Owner {
    pub data: Vec<i32>,
    pub n: i32,
    pub arr: [i32; 2],
    pub p: Option<Box<i32>>,
}
impl Owner {
    pub unsafe fn move_from(this: *mut Owner, _a0: *mut Owner) {
        (&raw mut (*this).data).write(std::mem::take(&mut (*_a0).data));
        (&raw mut (*this).n).write((*_a0).n);
        (&raw mut (*this).arr).write(std::array::from_fn::<_, 2, _>(|__i: usize| {
            (*_a0).arr[(__i)]
        }));
        (&raw mut (*this).p).write((*_a0).p.take());
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Owner) -> *mut Owner {
        {
            self.data = std::mem::take(&mut (*_a0).data);
        };
        self.n = (*_a0).n;
        {
            let mut __tmp_2 = { ((&mut self.arr as *mut [i32; 2]) as *mut ::libc::c_void) };
            let mut __tmp_3 = { ((&mut (*_a0).arr as *mut [i32; 2]) as *const ::libc::c_void) };
            if 8_usize != 0 {
                ::std::ptr::copy_nonoverlapping(__tmp_3, __tmp_2, 8_usize)
            }
            __tmp_2
        };
        {
            self.p = (*_a0).p.take();
        };
        return &mut (*(self as *mut Owner));
    }
    pub unsafe fn new_1(this: *mut Owner) {
        (&raw mut (*this).data).write(Vec::new());
        (&raw mut (*this).p).write(None);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Copyable {
    pub data: Vec<i32>,
}
impl Copyable {
    pub unsafe fn copy_from(this: *mut Copyable, _a0: *const Copyable) {
        (&raw mut (*this).data).write((*_a0).data.clone());
    }
    pub unsafe fn move_from(this: *mut Copyable, _a0: *mut Copyable) {
        (&raw mut (*this).data).write(std::mem::take(&mut (*_a0).data));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Copyable) -> *mut Copyable {
        {
            self.data = std::mem::take(&mut (*_a0).data);
        };
        return &mut (*(self as *mut Copyable));
    }
    pub unsafe fn new_1(this: *mut Copyable) {
        (&raw mut (*this).data).write(Vec::new());
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Holder {
    pub inner: Inner,
    pub e: Explicit,
    pub p: Option<Box<i32>>,
}
impl Holder {
    pub unsafe fn new(this: *mut Holder, mut v: i32) {
        {
            let __d = (&raw mut (*this).inner);
            (&raw mut (*__d).x).write(v);
        };
        Explicit::new((&raw mut (*this).e), { v });
        (&raw mut (*this).p).write(None);
    }
    pub unsafe fn move_from(this: *mut Holder, _a0: *mut Holder) {
        Inner::move_from((&raw mut (*this).inner), { &mut (*_a0).inner });
        Explicit::move_from((&raw mut (*this).e), { &mut (*_a0).e });
        (&raw mut (*this).p).write((*_a0).p.take());
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.inner = (*_a0).inner;
        self.e = ((*_a0).e).clone();
        {
            self.p = (*_a0).p.take();
        };
        return &mut (*(self as *mut Holder));
    }
}
impl Holder {
    pub unsafe fn destructor(&mut self) {
        Explicit::destructor(&mut self.e);
    }
}
pub unsafe fn same_0(a: *const Explicit, b: *const Explicit) -> bool {
    return (((((*a).v) == ((*b).v)) && (((*a).inner.x) == ((*b).inner.x)))
        && (((*a).arr[(0) as usize]) == ((*b).arr[(0) as usize])))
        && (((*a).arr[(1) as usize]) == ((*b).arr[(1) as usize]));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Explicit = ::std::mem::MaybeUninit::uninit().assume_init();
    Explicit::new(&raw mut a, { 1 });
    let _dtor_a = std::mem::DropGuard::new(&raw mut a, |__p| unsafe { (*__p).destructor() });
    let mut b: Explicit = ::std::mem::MaybeUninit::uninit().assume_init();
    Explicit::copy_from(&raw mut b, { &a });
    let _dtor_b = std::mem::DropGuard::new(&raw mut b, |__p| unsafe { (*__p).destructor() });
    let mut c: Explicit = ::std::mem::MaybeUninit::uninit().assume_init();
    Explicit::copy_from(&raw mut c, { &a });
    let _dtor_c = std::mem::DropGuard::new(&raw mut c, |__p| unsafe { (*__p).destructor() });
    let mut d: Explicit = ::std::mem::MaybeUninit::uninit().assume_init();
    Explicit::move_from(&raw mut d, { &mut a });
    let _dtor_d = std::mem::DropGuard::new(&raw mut d, |__p| unsafe { (*__p).destructor() });
    assert!(
        ((unsafe { same_0(&b, &a,) }) && (unsafe { same_0(&c, &a,) }))
            && (unsafe { same_0(&d, &a,) })
    );
    let mut e: Explicit = ::std::mem::MaybeUninit::uninit().assume_init();
    Explicit::new(&raw mut e, { 2 });
    let _dtor_e = std::mem::DropGuard::new(&raw mut e, |__p| unsafe { (*__p).destructor() });
    let mut f: Explicit = ::std::mem::MaybeUninit::uninit().assume_init();
    Explicit::new(&raw mut f, { 3 });
    let _dtor_f = std::mem::DropGuard::new(&raw mut f, |__p| unsafe { (*__p).destructor() });
    e = (b).clone();
    f = (c).clone();
    assert!((unsafe { same_0(&e, &b,) }) && (unsafe { same_0(&f, &c,) }));
    let mut g: Explicit = ::std::mem::MaybeUninit::uninit().assume_init();
    Explicit::new(&raw mut g, { 4 });
    let _dtor_g = std::mem::DropGuard::new(&raw mut g, |__p| unsafe { (*__p).destructor() });
    g = {
        e = (f).clone();
        (e).clone()
    };
    assert!((unsafe { same_0(&g, &f,) }) && (unsafe { same_0(&e, &f,) }));
    let mut i: Implicit = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut i;
        (&raw mut (*__d).v).write(5);
        {
            let __d = (&raw mut (*__d).inner);
            (&raw mut (*__d).x).write(50);
        };
        (&raw mut (*__d).arr).write([5, 6]);
    };
    let mut j: Implicit = ::std::mem::MaybeUninit::uninit().assume_init();
    Implicit::copy_from(&raw mut j, { &i });
    let mut k: Implicit = ::std::mem::MaybeUninit::uninit().assume_init();
    Implicit::move_from(&raw mut k, { &mut i });
    assert!((((j.v) == (5)) && ((j.inner.x) == (50))) && ((j.arr[(1) as usize]) == (6)));
    assert!(((i.v) == (5)) && ((k.v) == (5)));
    let mut l: Implicit = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut l;
        (&raw mut (*__d).v).write(0);
        {
            let __d = (&raw mut (*__d).inner);
            (&raw mut (*__d).x).write(0);
        };
        (&raw mut (*__d).arr).write([0, 0]);
    };
    l = j;
    assert!((((l.v) == (5)) && ((l.inner.x) == (50))) && ((l.arr[(0) as usize]) == (5)));
    let mut vec_: Vec<Explicit> = Vec::new();
    {
        let a0_clone = b.clone();
        vec_.push(a0_clone)
    };
    {
        let __a1 = {
            let mut __v: Explicit = ::std::mem::MaybeUninit::uninit().assume_init();
            Explicit::new(&raw mut __v, { 9 });
            __v
        };
        vec_.push(__a1)
    };
    assert!(((vec_[(0_usize)].v) == (1)) && ((vec_[(1_usize)].v) == (9)));
    let mut m: DefaultCopyUserMove = ::std::mem::MaybeUninit::uninit().assume_init();
    DefaultCopyUserMove::new(&raw mut m, { 7 });
    let mut m1: DefaultCopyUserMove = ::std::mem::MaybeUninit::uninit().assume_init();
    DefaultCopyUserMove::copy_from(&raw mut m1, { &m });
    let mut m2: DefaultCopyUserMove = ::std::mem::MaybeUninit::uninit().assume_init();
    DefaultCopyUserMove::move_from(&raw mut m2, { &mut m });
    assert!((((m1.v) == (7)) && ((m2.v) == (7))) && ((m.v) == (0)));
    let mut m3: DefaultCopyUserMove = ::std::mem::MaybeUninit::uninit().assume_init();
    DefaultCopyUserMove::new(&raw mut m3, { 1 });
    let mut m4: DefaultCopyUserMove = ::std::mem::MaybeUninit::uninit().assume_init();
    DefaultCopyUserMove::new(&raw mut m4, { 1 });
    m3 = m1;
    (unsafe { DefaultCopyUserMove::move_assign(&mut m4, &mut m1) });
    assert!((((m3.v) == (7)) && ((m4.v) == (7))) && ((m1.v) == (0)));
    let mut u: UserCopyDefaultMove = ::std::mem::MaybeUninit::uninit().assume_init();
    UserCopyDefaultMove::new(&raw mut u, { 8 });
    let mut u1: UserCopyDefaultMove = ::std::mem::MaybeUninit::uninit().assume_init();
    UserCopyDefaultMove::copy_from(&raw mut u1, { &u });
    let mut u2: UserCopyDefaultMove = ::std::mem::MaybeUninit::uninit().assume_init();
    UserCopyDefaultMove::move_from(&raw mut u2, { &mut u });
    assert!((((u1.v) == (108)) && ((u2.v) == (8))) && ((u.v) == (8)));
    let mut u3: UserCopyDefaultMove = ::std::mem::MaybeUninit::uninit().assume_init();
    UserCopyDefaultMove::new(&raw mut u3, { 1 });
    let mut u4: UserCopyDefaultMove = ::std::mem::MaybeUninit::uninit().assume_init();
    UserCopyDefaultMove::new(&raw mut u4, { 1 });
    (unsafe { UserCopyDefaultMove::copy_assign(&mut u3, &u2) });
    (unsafe { UserCopyDefaultMove::move_assign(&mut u4, &mut u2) });
    assert!(((u3.v) == (108)) && ((u4.v) == (8)));
    let mut p: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
    Buffer::new(&raw mut p, { 3 });
    let mut q: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
    Buffer::move_from(&raw mut q, { &mut p });
    assert!(
        ((((q.n) == (3)) && ((q.data.len()) == (3_usize))) && ((q.data[(2_usize)]) == (3)))
            && (p.data.is_empty())
    );
    let mut r: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
    Buffer::new(&raw mut r, { 1 });
    (unsafe { Buffer::move_assign(&mut r, &mut q) });
    assert!(
        ((((r.n) == (3)) && ((r.data.len()) == (3_usize))) && ((r.arr[(1) as usize]) == (4)))
            && (q.data.is_empty())
    );
    assert!(
        (((r.rows.len()) == (1_usize)) && ((r.rows[(0_usize)].len()) == (3_usize)))
            && (q.rows.is_empty())
    );
    let mut bufs: Vec<Buffer> = Vec::new();
    {
        let __a1 = {
            let mut __v: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
            Buffer::move_from(&raw mut __v, { &mut r });
            __v
        };
        bufs.push(__a1)
    };
    {
        let mut __tmp_4 = {
            {
                let mut __v: Buffer = ::std::mem::MaybeUninit::uninit().assume_init();
                Buffer::move_from(&raw mut __v, { &mut bufs[(0_usize)] });
                __v
            }
        };
        bufs.push_mut(__tmp_4)
    };
    assert!(
        (((bufs[(1_usize)].n) == (3)) && ((bufs[(1_usize)].data.len()) == (3_usize)))
            && (bufs[(0_usize)].data.is_empty())
    );
    let mut o1: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
    Owner::new_1(&raw mut o1);
    {
        let __a1 = 5;
        o1.data.push(__a1)
    };
    o1.n = 5;
    o1.arr[(0) as usize] = 5;
    o1.arr[(1) as usize] = 6;
    {
        let _a0: *mut i32 = (Box::leak(Box::new(7)) as *mut i32);
        o1.p = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let mut o2: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
    Owner::move_from(&raw mut o2, { &mut o1 });
    assert!(
        ((((o2.n) == (5)) && ((o2.data.len()) == (1_usize))) && ((o2.arr[(1) as usize]) == (6)))
            && ((*o2
                .p
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (7))
    );
    assert!(
        (o1.data.is_empty())
            && ((o1
                .p
                .as_deref_mut()
                .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
            .is_null())
    );
    let mut o3: Owner = ::std::mem::MaybeUninit::uninit().assume_init();
    Owner::new_1(&raw mut o3);
    (unsafe { Owner::move_assign(&mut o3, &mut o2) });
    assert!(
        ((((o3.n) == (5)) && ((o3.data[(0_usize)]) == (5))) && ((o3.arr[(0) as usize]) == (5)))
            && ((*o3
                .p
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (7))
    );
    assert!(
        (o2.data.is_empty())
            && ((o2
                .p
                .as_deref_mut()
                .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
            .is_null())
    );
    let mut c1: Copyable = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut c1;
        (&raw mut (*__d).data).write(Vec::from([1, 2]));
    };
    let mut c2: Copyable = ::std::mem::MaybeUninit::uninit().assume_init();
    Copyable::copy_from(&raw mut c2, { &c1 });
    let mut c3: Copyable = ::std::mem::MaybeUninit::uninit().assume_init();
    Copyable::move_from(&raw mut c3, { &mut c1 });
    assert!(((c2.data.len()) == (2_usize)) && ((c3.data.len()) == (2_usize)));
    assert!(c1.data.is_empty());
    let mut c4: Copyable = ::std::mem::MaybeUninit::uninit().assume_init();
    Copyable::new_1(&raw mut c4);
    (unsafe { Copyable::move_assign(&mut c4, &mut c3) });
    assert!(((c4.data[(1_usize)]) == (2)) && (c3.data.is_empty()));
    let mut h1: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    Holder::new(&raw mut h1, { 4 });
    let _dtor_h1 = std::mem::DropGuard::new(&raw mut h1, |__p| unsafe { (*__p).destructor() });
    {
        let _a0: *mut i32 = (Box::leak(Box::new(9)) as *mut i32);
        h1.p = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let mut h2: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    Holder::move_from(&raw mut h2, { &mut h1 });
    let _dtor_h2 = std::mem::DropGuard::new(&raw mut h2, |__p| unsafe { (*__p).destructor() });
    assert!(
        ((((h2.inner.x) == (4)) && ((h2.e.v) == (4)))
            && ((*h2
                .p
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (9)))
            && ((h1
                .p
                .as_deref_mut()
                .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
            .is_null())
    );
    let mut h3: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    Holder::new(&raw mut h3, { 1 });
    let _dtor_h3 = std::mem::DropGuard::new(&raw mut h3, |__p| unsafe { (*__p).destructor() });
    (unsafe { Holder::move_assign(&mut h3, &mut h2) });
    assert!(
        ((((h3.inner.x) == (4)) && ((h3.e.arr[(1) as usize]) == (5)))
            && ((*h3
                .p
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (9)))
            && ((h2
                .p
                .as_deref_mut()
                .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
            .is_null())
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
