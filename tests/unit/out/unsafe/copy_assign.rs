extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut assigns_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Partial {
    pub v: i32,
    pub keep: i32,
}
impl Partial {
    pub unsafe fn new(this: *mut Partial, mut v: i32, mut keep: i32) {
        (&raw mut (*this).v).write(v);
        (&raw mut (*this).keep).write(keep);
    }
    pub unsafe fn copy_from(this: *mut Partial, o: *const Partial) {
        (&raw mut (*this).v).write((*o).v);
        (&raw mut (*this).keep).write((*o).keep);
    }
    pub unsafe fn copy_assign(&mut self, o: *const Partial) -> *mut Partial {
        if (((self as *mut Partial).cast_const()) == (o)) {
            return &mut (*(self as *mut Partial));
        }
        self.v = (*o).v;
        (*(*&raw const assigns_0).get()).prefix_inc();
        return &mut (*(self as *mut Partial));
    }
}
impl Clone for Partial {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Partial = ::std::mem::MaybeUninit::uninit().assume_init();
            Partial::copy_from(&raw mut __v, self as *const Partial);
            __v
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NonConstAssign {
    pub mark: i32,
}
impl NonConstAssign {
    pub unsafe fn new(this: *mut NonConstAssign) {
        (&raw mut (*this).mark).write(0);
    }
    pub unsafe fn operator_assign_2(&mut self, o: *mut NonConstAssign) -> *mut NonConstAssign {
        self.mark = (((*o).mark) + (1));
        return &mut (*(self as *mut NonConstAssign));
    }
    pub unsafe fn operator_assign_3(&mut self, o: *const NonConstAssign) -> *mut NonConstAssign {
        self.mark = (((*o).mark) + (10));
        return &mut (*(self as *mut NonConstAssign));
    }
    pub unsafe fn copy_from(this: *mut NonConstAssign, _a0: *const NonConstAssign) {
        (&raw mut (*this).mark).write((*_a0).mark);
    }
}
impl Default for NonConstAssign {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: NonConstAssign = ::std::mem::MaybeUninit::uninit().assume_init();
                NonConstAssign::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct RefQualified {
    pub mark: i32,
}
impl RefQualified {
    pub unsafe fn new(this: *mut RefQualified) {
        (&raw mut (*this).mark).write(0);
    }
    pub unsafe fn copy_assign(&mut self, o: *const RefQualified) -> *mut RefQualified {
        self.mark = (((*o).mark) + (1));
        return &mut (*(self as *mut RefQualified));
    }
    pub unsafe fn copy_from(this: *mut RefQualified, _a0: *const RefQualified) {
        (&raw mut (*this).mark).write((*_a0).mark);
    }
}
impl Default for RefQualified {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: RefQualified = ::std::mem::MaybeUninit::uninit().assume_init();
                RefQualified::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Holder {
    pub p: Partial,
    pub arr: [Partial; 2],
}
impl Holder {
    pub unsafe fn copy_from(this: *mut Holder, _a0: *const Holder) {
        Partial::copy_from((&raw mut (*this).p), { &(*_a0).p });
        {
            let __d = (&raw mut (*this).arr).cast::<Partial>();
            for __i in 0..2_usize {
                Partial::copy_from(__d.offset(__i as isize), { &(*_a0).arr[(__i)] });
            }
        };
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Holder) -> *mut Holder {
        (unsafe {
            let _o: *const Partial = &(*_a0).p;
            Partial::copy_assign(&mut self.p, _o)
        });
        {
            let mut __i0: usize = 0_usize;
            'loop_: while ((__i0) != (2_usize)) {
                (unsafe {
                    let _o: *const Partial = &(*_a0).arr[(__i0)];
                    Partial::copy_assign(&mut self.arr[(__i0)], _o)
                });
                __i0.prefix_inc();
            }
        }
        return &mut (*(self as *mut Holder));
    }
}
pub static mut copies_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct CopyCounted {
    pub v: i32,
}
impl CopyCounted {
    pub unsafe fn new(this: *mut CopyCounted, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut CopyCounted, o: *const CopyCounted) {
        (&raw mut (*this).v).write((*o).v);
        (*(*&raw const copies_1).get()).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const CopyCounted) -> *mut CopyCounted {
        self.v = (*_a0).v;
        return &mut (*(self as *mut CopyCounted));
    }
}
impl Clone for CopyCounted {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: CopyCounted = ::std::mem::MaybeUninit::uninit().assume_init();
            CopyCounted::copy_from(&raw mut __v, self as *const CopyCounted);
            __v
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct HasCopyCounted {
    pub c: CopyCounted,
    pub arr: [CopyCounted; 2],
}
impl HasCopyCounted {
    pub unsafe fn copy_from(this: *mut HasCopyCounted, _a0: *const HasCopyCounted) {
        CopyCounted::copy_from((&raw mut (*this).c), { &(*_a0).c });
        {
            let __d = (&raw mut (*this).arr).cast::<CopyCounted>();
            for __i in 0..2_usize {
                CopyCounted::copy_from(__d.offset(__i as isize), { &(*_a0).arr[(__i)] });
            }
        };
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const HasCopyCounted) -> *mut HasCopyCounted {
        (unsafe {
            let _arg0: *const CopyCounted = &(*_a0).c;
            CopyCounted::copy_assign(&mut self.c, _arg0)
        });
        {
            let mut __tmp_0 = { ((&mut self.arr as *mut [CopyCounted; 2]) as *mut ::libc::c_void) };
            let mut __tmp_1 =
                { ((&(*_a0).arr as *const [CopyCounted; 2]) as *const ::libc::c_void) };
            if 8_usize != 0 {
                ::std::ptr::copy_nonoverlapping(__tmp_1, __tmp_0, 8_usize)
            }
            __tmp_0
        };
        return &mut (*(self as *mut HasCopyCounted));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Partial = ::std::mem::MaybeUninit::uninit().assume_init();
    Partial::new(&raw mut a, { 1 }, { 100 });
    let mut b: Partial = ::std::mem::MaybeUninit::uninit().assume_init();
    Partial::new(&raw mut b, { 2 }, { 200 });
    let mut c: Partial = ::std::mem::MaybeUninit::uninit().assume_init();
    Partial::new(&raw mut c, { 3 }, { 300 });
    (unsafe { Partial::copy_assign(&mut a, &b) });
    assert!(((a.v) == (2)) && ((a.keep) == (100)));
    assert!(((*(*&raw const assigns_0).get()) == (1)));
    (unsafe { Partial::copy_assign(&mut c, &(*(unsafe { Partial::copy_assign(&mut a, &b) }))) });
    assert!(((c.v) == (2)) && ((c.keep) == (300)));
    assert!(((*(*&raw const assigns_0).get()) == (3)));
    (unsafe {
        let _o: *const Partial = &a;
        Partial::copy_assign(&mut a, _o)
    });
    assert!(((*(*&raw const assigns_0).get()) == (3)));
    (unsafe {
        let mut _o: Partial = ::std::mem::MaybeUninit::uninit().assume_init();
        Partial::new(&raw mut _o, { 9 }, { 900 });
        Partial::copy_assign(&mut a, &mut _o)
    });
    assert!(((a.v) == (9)) && ((a.keep) == (100)));
    assert!(((*(*&raw const assigns_0).get()) == (4)));
    let ra: *mut Partial = &mut a;
    (unsafe {
        let _o: *const Partial = &c;
        Partial::copy_assign(&mut (*ra), _o)
    });
    assert!(((a.v) == (2)));
    let mut pa: *mut Partial = (&mut a as *mut Partial);
    (unsafe {
        let _o: *const Partial = &b;
        Partial::copy_assign(&mut (*pa), _o)
    });
    assert!(((a.v) == (2)));
    assert!(((*(*&raw const assigns_0).get()) == (6)));
    let mut h: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut h;
        Partial::new((&raw mut (*__d).p), { 4 }, { 40 });
        {
            let __d = (&raw mut (*__d).arr).cast::<Partial>();
            Partial::new(__d.offset(0), { 5 }, { 50 });
            Partial::new(__d.offset(1), { 6 }, { 60 });
        };
    };
    (unsafe { Partial::copy_assign(&mut h.p, &b) });
    (unsafe { Partial::copy_assign(&mut h.arr[(1) as usize], &c) });
    assert!(((h.p.v) == (2)) && ((h.p.keep) == (40)));
    assert!(((h.arr[(1) as usize].v) == (2)) && ((h.arr[(1) as usize].keep) == (60)));
    assert!(((*(*&raw const assigns_0).get()) == (8)));
    let mut n: NonConstAssign = ::std::mem::MaybeUninit::uninit().assume_init();
    NonConstAssign::new(&raw mut n);
    let mut n1: NonConstAssign = ::std::mem::MaybeUninit::uninit().assume_init();
    NonConstAssign::new(&raw mut n1);
    let mut n2: NonConstAssign = ::std::mem::MaybeUninit::uninit().assume_init();
    NonConstAssign::new(&raw mut n2);
    let mut cn: NonConstAssign = ::std::mem::MaybeUninit::uninit().assume_init();
    NonConstAssign::new(&raw mut cn);
    (unsafe { NonConstAssign::operator_assign_2(&mut n1, &mut n) });
    (unsafe { NonConstAssign::operator_assign_3(&mut n2, &cn) });
    assert!(((n1.mark) == (1)));
    assert!(((n2.mark) == (10)));
    let mut r: RefQualified = ::std::mem::MaybeUninit::uninit().assume_init();
    RefQualified::new(&raw mut r);
    let mut r1: RefQualified = ::std::mem::MaybeUninit::uninit().assume_init();
    RefQualified::new(&raw mut r1);
    (unsafe { RefQualified::copy_assign(&mut r1, &r) });
    assert!(((r1.mark) == (1)));
    let mut h2: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut h2;
        Partial::new((&raw mut (*__d).p), { 7 }, { 70 });
        {
            let __d = (&raw mut (*__d).arr).cast::<Partial>();
            Partial::new(__d.offset(0), { 8 }, { 80 });
            Partial::new(__d.offset(1), { 9 }, { 90 });
        };
    };
    (unsafe { Holder::copy_assign(&mut h2, &h) });
    assert!(((h2.p.v) == (2)) && ((h2.p.keep) == (70)));
    assert!(((h2.arr[(1) as usize].v) == (2)) && ((h2.arr[(1) as usize].keep) == (90)));
    assert!(((*(*&raw const assigns_0).get()) == (11)));
    let mut cc1: CopyCounted = ::std::mem::MaybeUninit::uninit().assume_init();
    CopyCounted::new(&raw mut cc1, { 1 });
    let mut cc2: CopyCounted = ::std::mem::MaybeUninit::uninit().assume_init();
    CopyCounted::new(&raw mut cc2, { 2 });
    (unsafe { CopyCounted::copy_assign(&mut cc1, &cc2) });
    assert!(((cc1.v) == (2)));
    let mut hc1: HasCopyCounted = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut hc1;
        CopyCounted::new((&raw mut (*__d).c), { 1 });
        {
            let __d = (&raw mut (*__d).arr).cast::<CopyCounted>();
            CopyCounted::new(__d.offset(0), { 2 });
            CopyCounted::new(__d.offset(1), { 3 });
        };
    };
    let mut hc2: HasCopyCounted = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut hc2;
        CopyCounted::new((&raw mut (*__d).c), { 4 });
        {
            let __d = (&raw mut (*__d).arr).cast::<CopyCounted>();
            CopyCounted::new(__d.offset(0), { 5 });
            CopyCounted::new(__d.offset(1), { 6 });
        };
    };
    (unsafe { HasCopyCounted::copy_assign(&mut hc1, &hc2) });
    assert!(
        (((hc1.c.v) == (4)) && ((hc1.arr[(0) as usize].v) == (5)))
            && ((hc1.arr[(1) as usize].v) == (6))
    );
    assert!(((*(*&raw const copies_1).get()) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const assigns_0).get();
    (*&raw const copies_1).get();
}
