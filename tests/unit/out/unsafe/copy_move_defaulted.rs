extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Inner {
    pub x: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct Explicit {
    pub v: i32,
    pub inner: Inner,
    pub arr: [i32; 2],
}
impl Explicit {
    pub unsafe fn Explicit(mut v: i32) -> Self {
        let mut __this = Self {
            v: v,
            inner: Inner { x: ((v) * (10)) },
            arr: [v, ((v) + (1))],
        };
        let this = &raw mut __this;
        __this
    }
    pub unsafe fn destructor(&mut self) {
        let this = self as *mut Explicit;
    }
}
impl Default for Explicit {
    fn default() -> Self {
        Explicit {
            v: 0_i32,
            inner: <Inner>::default(),
            arr: [0_i32; 2],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Implicit {
    pub v: i32,
    pub inner: Inner,
    pub arr: [i32; 2],
}
impl Default for Implicit {
    fn default() -> Self {
        Implicit {
            v: 0_i32,
            inner: <Inner>::default(),
            arr: [0_i32; 2],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DefaultCopyUserMove {
    pub v: i32,
}
impl DefaultCopyUserMove {
    pub unsafe fn DefaultCopyUserMove(mut v: i32) -> Self {
        let mut __this = Self { v: v };
        let this = &raw mut __this;
        __this
    }
    pub unsafe fn DefaultCopyUserMove_pmutDefaultCopyUserMove(o: *mut DefaultCopyUserMove) -> Self {
        let mut __this = Self { v: (*o).v };
        let this = &raw mut __this;
        (*o).v = 0;
        __this
    }
    pub unsafe fn operator_assign_pmutDefaultCopyUserMove(
        &mut self,
        o: *mut DefaultCopyUserMove,
    ) -> *mut DefaultCopyUserMove {
        let this = self as *mut DefaultCopyUserMove;
        (*this).v = (*o).v;
        (*o).v = 0;
        return &mut (*this) as *mut DefaultCopyUserMove;
    }
}
#[repr(C)]
#[derive(Default)]
pub struct UserCopyDefaultMove {
    pub v: i32,
}
impl UserCopyDefaultMove {
    pub unsafe fn UserCopyDefaultMove(mut v: i32) -> Self {
        let mut __this = Self { v: v };
        let this = &raw mut __this;
        __this
    }
    pub unsafe fn UserCopyDefaultMove_pconstUserCopyDefaultMove(
        o: *const UserCopyDefaultMove,
    ) -> Self {
        let mut __this = Self {
            v: (((*o).v) + (100)),
        };
        let this = &raw mut __this;
        __this
    }
    pub unsafe fn operator_assign_pconstUserCopyDefaultMove(
        &mut self,
        o: *const UserCopyDefaultMove,
    ) -> *mut UserCopyDefaultMove {
        let this = self as *mut UserCopyDefaultMove;
        (*this).v = (((*o).v) + (100));
        return &mut (*this) as *mut UserCopyDefaultMove;
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        unsafe {
            UserCopyDefaultMove::UserCopyDefaultMove_pconstUserCopyDefaultMove(
                self as *const UserCopyDefaultMove,
            )
        }
    }
}
pub unsafe fn same_0(a: *const Explicit, b: *const Explicit) -> bool {
    return (((((*a).v) == ((*b).v)) && (((*a).inner.x) == ((*b).inner.x)))
        && (((*a).arr[(0) as usize]) == ((*b).arr[(0) as usize])))
        && (((*a).arr[(1) as usize]) == ((*b).arr[(1) as usize]));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Explicit = Explicit::Explicit({ 1 });
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Explicit::destructor);
    let mut b: Explicit = a.clone();
    let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Explicit::destructor);
    let mut c: Explicit = a.clone();
    let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Explicit::destructor);
    let mut d: Explicit = a.clone();
    let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Explicit::destructor);
    assert!(
        ((unsafe { same_0(&b as *const Explicit, &a as *const Explicit,) })
            && (unsafe { same_0(&c as *const Explicit, &a as *const Explicit,) }))
            && (unsafe { same_0(&d as *const Explicit, &a as *const Explicit,) })
    );
    let mut e: Explicit = Explicit::Explicit({ 2 });
    let _dtor_e = ScopedDestructorUnsafe::new(&raw mut e, Explicit::destructor);
    let mut f: Explicit = Explicit::Explicit({ 3 });
    let _dtor_f = ScopedDestructorUnsafe::new(&raw mut f, Explicit::destructor);
    e = b;
    f = c;
    assert!(
        (unsafe { same_0(&e as *const Explicit, &b as *const Explicit,) })
            && (unsafe { same_0(&f as *const Explicit, &c as *const Explicit,) })
    );
    let mut g: Explicit = Explicit::Explicit({ 4 });
    let _dtor_g = ScopedDestructorUnsafe::new(&raw mut g, Explicit::destructor);
    g = {
        e = f;
        e
    };
    assert!(
        (unsafe { same_0(&g as *const Explicit, &f as *const Explicit,) })
            && (unsafe { same_0(&e as *const Explicit, &f as *const Explicit,) })
    );
    let mut i: Implicit = Implicit {
        v: 5,
        inner: Inner { x: 50 },
        arr: [5, 6],
    };
    let mut j: Implicit = i;
    let mut k: Implicit = i;
    assert!((((j.v) == (5)) && ((j.inner.x) == (50))) && ((j.arr[(1) as usize]) == (6)));
    assert!(((i.v) == (5)) && ((k.v) == (5)));
    let mut l: Implicit = Implicit {
        v: 0,
        inner: Inner { x: 0 },
        arr: [0, 0],
    };
    l = j;
    assert!((((l.v) == (5)) && ((l.inner.x) == (50))) && ((l.arr[(0) as usize]) == (5)));
    let mut vec_: Vec<Explicit> = Vec::new();
    {
        let a0_clone = b.clone();
        vec_.push(a0_clone)
    };
    vec_.push(Explicit::Explicit({ 9 }));
    assert!(((vec_[(0_usize)].v) == (1)) && ((vec_[(1_usize)].v) == (9)));
    let mut m: DefaultCopyUserMove = DefaultCopyUserMove::DefaultCopyUserMove({ 7 });
    let mut m1: DefaultCopyUserMove = m;
    let mut m2: DefaultCopyUserMove =
        DefaultCopyUserMove::DefaultCopyUserMove_pmutDefaultCopyUserMove({ &mut m });
    assert!((((m1.v) == (7)) && ((m2.v) == (7))) && ((m.v) == (0)));
    let mut m3: DefaultCopyUserMove = DefaultCopyUserMove::DefaultCopyUserMove({ 1 });
    let mut m4: DefaultCopyUserMove = DefaultCopyUserMove::DefaultCopyUserMove({ 1 });
    m3 = m1;
    (unsafe { DefaultCopyUserMove::operator_assign_pmutDefaultCopyUserMove(&mut m4, &mut m1) });
    assert!((((m3.v) == (7)) && ((m4.v) == (7))) && ((m1.v) == (0)));
    let mut u: UserCopyDefaultMove = UserCopyDefaultMove::UserCopyDefaultMove({ 8 });
    let mut u1: UserCopyDefaultMove =
        UserCopyDefaultMove::UserCopyDefaultMove_pconstUserCopyDefaultMove({
            &u as *const UserCopyDefaultMove
        });
    let mut u2: UserCopyDefaultMove = u.clone();
    assert!((((u1.v) == (108)) && ((u2.v) == (8))) && ((u.v) == (8)));
    let mut u3: UserCopyDefaultMove = UserCopyDefaultMove::UserCopyDefaultMove({ 1 });
    let mut u4: UserCopyDefaultMove = UserCopyDefaultMove::UserCopyDefaultMove({ 1 });
    (unsafe {
        UserCopyDefaultMove::operator_assign_pconstUserCopyDefaultMove(
            &mut u3,
            &u2 as *const UserCopyDefaultMove,
        )
    });
    u4 = u2;
    assert!(((u3.v) == (108)) && ((u4.v) == (8)));
    return 0;
}
