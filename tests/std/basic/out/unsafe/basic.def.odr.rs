extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copyCount_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HasConstexprCtor {
    pub m: i32,
}
impl HasConstexprCtor {
    pub unsafe fn reset() {
        (*(*&raw const copyCount_0).get()) = 0;
    }
    pub unsafe fn new(this: *mut HasConstexprCtor, mut v: i32) {
        (&raw mut (*this).m).write(v);
    }
    pub unsafe fn copy_from(this: *mut HasConstexprCtor, o: *const HasConstexprCtor) {
        (&raw mut (*this).m).write((*o).m);
        (*(*&raw const copyCount_0).get()).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const HasConstexprCtor) -> *mut HasConstexprCtor {
        self.m = (*_a0).m;
        return &mut (*(self as *mut HasConstexprCtor));
    }
}
impl Clone for HasConstexprCtor {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
            HasConstexprCtor::copy_from(&raw mut __v, self as *const HasConstexprCtor);
            __v
        }
    }
}
pub static mut ctorCount_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut copyCount_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut dtorCount_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut useCount_4: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct LifetimeTracked {
    pub uses: i32,
}
impl LifetimeTracked {
    pub unsafe fn reset() {
        (*(*&raw const ctorCount_1).get()) = {
            (*(*&raw const copyCount_2).get()) = {
                (*(*&raw const dtorCount_3).get()) = {
                    (*(*&raw const useCount_4).get()) = 0;
                    (*(*&raw const useCount_4).get())
                };
                (*(*&raw const dtorCount_3).get())
            };
            (*(*&raw const copyCount_2).get())
        };
    }
    pub unsafe fn new(this: *mut LifetimeTracked) {
        (&raw mut (*this).uses).write(0);
        (*(*&raw const ctorCount_1).get()).prefix_inc();
    }
    pub unsafe fn copy_from(this: *mut LifetimeTracked, _a0: *const LifetimeTracked) {
        (&raw mut (*this).uses).write(0);
        (*(*&raw const copyCount_2).get()).prefix_inc();
    }
    pub unsafe fn destructor(&mut self) {
        (*(*&raw const dtorCount_3).get()).prefix_inc();
    }
    pub unsafe fn use_(&self) {
        (*(&raw const self.uses).cast_mut()).prefix_inc();
        (*(*&raw const useCount_4).get()).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const LifetimeTracked) -> *mut LifetimeTracked {
        self.uses = (*_a0).uses;
        return &mut (*(self as *mut LifetimeTracked));
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
            LifetimeTracked::copy_from(&raw mut __v, self as *const LifetimeTracked);
            __v
        }
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
                LifetimeTracked::new(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn test_5() {
    (unsafe { HasConstexprCtor::reset() });
    let mut arr: [HasConstexprCtor; 2] = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = (&raw mut arr).cast::<HasConstexprCtor>();
        HasConstexprCtor::new(__d.offset(0), { 3 });
        HasConstexprCtor::new(__d.offset(1), { 4 });
    };
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return [
                {
                    let mut __v: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
                    HasConstexprCtor::new(&raw mut __v, { 3 });
                    __v
                },
                {
                    let mut __v: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
                    HasConstexprCtor::new(&raw mut __v, { 4 });
                    __v
                },
            ][(1) as usize]
                .m;
        }
    });
    assert!(((*(*&raw const copyCount_0).get()) == (0)));
    assert!(((unsafe { f.call() }) == (4)));
    assert!(((*(*&raw const copyCount_0).get()) == (0)));
}
pub unsafe fn test_6() {
    (unsafe { HasConstexprCtor::reset() });
    let mut k: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
    HasConstexprCtor::new(&raw mut k, { 5 });
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return {
                let mut __v: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
                HasConstexprCtor::new(&raw mut __v, { 5 });
                __v
            }
            .m;
        }
    });
    assert!(((*(*&raw const copyCount_0).get()) == (0)));
    assert!(((unsafe { f.call() }) == (5)));
    assert!(((*(*&raw const copyCount_0).get()) == (0)));
}
pub unsafe fn test_7() {
    (unsafe { HasConstexprCtor::reset() });
    let mut k1: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
    HasConstexprCtor::new(&raw mut k1, { 7 });
    let mut k2: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
    HasConstexprCtor::new(&raw mut k2, { 9 });
    let mut f: FnPtr<fn(bool) -> i32> = FnPtr::<fn(bool) -> i32>::new(|mut b: bool| -> i32 {
        unsafe {
            return (if b {
                {
                    let mut __v: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
                    HasConstexprCtor::new(&raw mut __v, { 7 });
                    __v
                }
            } else {
                {
                    let mut __v: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
                    HasConstexprCtor::new(&raw mut __v, { 9 });
                    __v
                }
            })
            .m;
        }
    });
    assert!(((*(*&raw const copyCount_0).get()) == (0)));
    assert!(((unsafe { f.call(true,) }) == (7)));
    assert!(((unsafe { f.call(false,) }) == (9)));
    assert!(((*(*&raw const copyCount_0).get()) == (0)));
}
pub static mut left_8: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn test_9() {
    (unsafe { HasConstexprCtor::reset() });
    let mut k: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
    HasConstexprCtor::new(&raw mut k, { 5 });
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return {
                (*(*&raw const left_8).get()).prefix_inc();
                {
                    let mut __v: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
                    HasConstexprCtor::new(&raw mut __v, { 5 });
                    __v
                }
            }
            .m;
        }
    });
    assert!(((*(*&raw const copyCount_0).get()) == (0)));
    assert!(((*(*&raw const left_8).get()) == (0)));
    assert!(((unsafe { f.call() }) == (5)));
    assert!(((*(*&raw const left_8).get()) == (1)));
    assert!(((*(*&raw const copyCount_0).get()) == (0)));
}
pub static mut derived_calls_10: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe trait p4_1_pure_virtual_without_definition_HasPureVirtual {
    unsafe fn f(&mut self);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_1_pure_virtual_without_definition_OverridesPureVirtual {}
impl p4_1_pure_virtual_without_definition_OverridesPureVirtual {
    pub unsafe fn new_1(this: *mut p4_1_pure_virtual_without_definition_OverridesPureVirtual) {}
    pub unsafe fn copy_from(
        this: *mut p4_1_pure_virtual_without_definition_OverridesPureVirtual,
        _a0: *const p4_1_pure_virtual_without_definition_OverridesPureVirtual,
    ) {
    }
}
pub unsafe fn test_11() {
    let mut d: p4_1_pure_virtual_without_definition_OverridesPureVirtual =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p4_1_pure_virtual_without_definition_OverridesPureVirtual::new_1(&raw mut d);
    let mut p: *mut dyn p4_1_pure_virtual_without_definition_HasPureVirtual =
        (&mut d as *mut p4_1_pure_virtual_without_definition_OverridesPureVirtual);
    (unsafe { (*p).f() });
    assert!(((*(*&raw const derived_calls_10).get()) == (1)));
}
pub static mut global_12: libcc2rs::LazyStorage<LifetimeTracked> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __static = LifetimeTracked::new(__p);
            libcc2rs::register_static_destructor(|| (*(*&raw const global_12).get()).destructor());
            __static
        }
    });
pub unsafe fn test_13() {
    (unsafe { LifetimeTracked::reset() });
    let r: *mut LifetimeTracked = &mut (*(*&raw const global_12).get());
    let mut f: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        (unsafe { LifetimeTracked::use_(&(*(*&raw const global_12).get())) });
    });
    assert!(((*(*&raw const copyCount_2).get()) == (0)));
    (unsafe { f.call() });
    assert!(((*(*&raw const copyCount_2).get()) == (0)));
    assert!((((*(*&raw const global_12).get()).uses) == (1)));
    assert!(((*(*&raw const ctorCount_1).get()) == (0)));
}
pub unsafe fn test_14() {
    (unsafe { LifetimeTracked::reset() });
    let mut t: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
    LifetimeTracked::new(&raw mut t);
    let _dtor_t = std::mem::DropGuard::new(&raw mut t, |__p| unsafe { (*__p).destructor() });
    let r: *mut LifetimeTracked = &mut t;
    let mut f: FnPtr<fn()> = lambda_unsafe!(
        {
            let r: LifetimeTracked = {
                let mut __v: LifetimeTracked = ::std::mem::MaybeUninit::uninit().assume_init();
                LifetimeTracked::copy_from(&raw mut __v, { r });
                __v
            };
        },
        || {
            (unsafe { LifetimeTracked::use_(&r) });
        }
    );
    let _dtor_f = std::mem::DropGuard::new(&raw mut f, |__p| unsafe { (*__p).destroy() });
    assert!(((*(*&raw const copyCount_2).get()) == (1)));
    (unsafe { f.call() });
    assert!(((*(*&raw const copyCount_2).get()) == (1)));
    assert!(((*(*&raw const useCount_4).get()) == (1)));
    assert!(((t.uses) == (0)));
}
pub unsafe fn test_15() {
    let n: i32 = 5;
    let mut f: FnPtr<fn() -> *const i32> = lambda_unsafe!(
        {
            let n: i32 = n;
        },
        || -> *const i32 {
            return (&n as *const i32);
        }
    );
    assert!(((unsafe { f.call() }) != (&n as *const i32)));
    assert!(((*(unsafe { f.call() })) == (5)));
}
pub unsafe fn test_16() {
    (unsafe { HasConstexprCtor::reset() });
    let mut k: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
    HasConstexprCtor::new(&raw mut k, { 5 });
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let k: HasConstexprCtor = {
                let mut __v: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
                HasConstexprCtor::copy_from(&raw mut __v, { &k });
                __v
            };
        },
        || -> i32 {
            let mut c: HasConstexprCtor = ::std::mem::MaybeUninit::uninit().assume_init();
            HasConstexprCtor::copy_from(&raw mut c, { &k });
            return c.m;
        }
    );
    assert!(((*(*&raw const copyCount_0).get()) == (1)));
    assert!(((unsafe { f.call() }) == (5)));
    assert!(((*(*&raw const copyCount_0).get()) == (2)));
}
pub unsafe fn test_17() {
    let n: i32 = 5;
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return 5;
        }
    });
    assert!(((unsafe { f.call() }) == (5)));
}
pub static mut copies_18: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    pub value: i32,
    pub scratch: i32,
}
impl p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    pub unsafe fn new(this: *mut p5_2_mutable_subobject_is_odr_used_HasMutableMember, mut v: i32) {
        (&raw mut (*this).value).write(v);
        (&raw mut (*this).scratch).write(0);
    }
    pub unsafe fn copy_from(
        this: *mut p5_2_mutable_subobject_is_odr_used_HasMutableMember,
        o: *const p5_2_mutable_subobject_is_odr_used_HasMutableMember,
    ) {
        (&raw mut (*this).value).write((*o).value);
        (&raw mut (*this).scratch).write((*o).scratch);
        (*(*&raw const copies_18).get()).prefix_inc();
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const p5_2_mutable_subobject_is_odr_used_HasMutableMember,
    ) -> *mut p5_2_mutable_subobject_is_odr_used_HasMutableMember {
        self.value = (*_a0).value;
        self.scratch = (*_a0).scratch;
        return &mut (*(self as *mut p5_2_mutable_subobject_is_odr_used_HasMutableMember));
    }
}
impl Clone for p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: p5_2_mutable_subobject_is_odr_used_HasMutableMember =
                ::std::mem::MaybeUninit::uninit().assume_init();
            p5_2_mutable_subobject_is_odr_used_HasMutableMember::copy_from(
                &raw mut __v,
                self as *const p5_2_mutable_subobject_is_odr_used_HasMutableMember,
            );
            __v
        }
    }
}
pub unsafe fn test_19() {
    let mut k: p5_2_mutable_subobject_is_odr_used_HasMutableMember =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p5_2_mutable_subobject_is_odr_used_HasMutableMember::new(&raw mut k, { 5 });
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let k: p5_2_mutable_subobject_is_odr_used_HasMutableMember = {
                let mut __v: p5_2_mutable_subobject_is_odr_used_HasMutableMember =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p5_2_mutable_subobject_is_odr_used_HasMutableMember::copy_from(&raw mut __v, {
                    &k
                });
                __v
            };
        },
        || -> i32 {
            return k.value;
        }
    );
    assert!(((*(*&raw const copies_18).get()) == (1)));
    assert!(((unsafe { f.call() }) == (5)));
    assert!(((*(*&raw const copies_18).get()) == (1)));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
    pub a: LifetimeTracked,
    pub b: LifetimeTracked,
}
impl p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
    pub unsafe fn copy_from(
        this: *mut p6_unevaluated_binding_needs_no_capture_HasTwoMembers,
        _a0: *const p6_unevaluated_binding_needs_no_capture_HasTwoMembers,
    ) {
        LifetimeTracked::copy_from((&raw mut (*this).a), { &(*_a0).a });
        LifetimeTracked::copy_from((&raw mut (*this).b), { &(*_a0).b });
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const p6_unevaluated_binding_needs_no_capture_HasTwoMembers,
    ) -> *mut p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
        (unsafe {
            let _arg0: *const LifetimeTracked = &(*_a0).a;
            LifetimeTracked::copy_assign(&mut self.a, _arg0)
        });
        (unsafe {
            let _arg0: *const LifetimeTracked = &(*_a0).b;
            LifetimeTracked::copy_assign(&mut self.b, _arg0)
        });
        return &mut (*(self as *mut p6_unevaluated_binding_needs_no_capture_HasTwoMembers));
    }
}
impl p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
    pub unsafe fn destructor(&mut self) {
        LifetimeTracked::destructor(&mut self.b);
        LifetimeTracked::destructor(&mut self.a);
    }
}
pub unsafe fn test_20() {
    (unsafe { LifetimeTracked::reset() });
    let mut p: p6_unevaluated_binding_needs_no_capture_HasTwoMembers =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut p;
        LifetimeTracked::new((&raw mut (*__d).a));
        LifetimeTracked::new((&raw mut (*__d).b));
    };
    let _dtor_p = std::mem::DropGuard::new(&raw mut p, |__p| unsafe { (*__p).destructor() });
    let __decomp_21: *mut p6_unevaluated_binding_needs_no_capture_HasTwoMembers = &mut p;
    let mut f: FnPtr<fn() -> usize> = FnPtr::<fn() -> usize>::new(|| -> usize {
        unsafe {
            return ::std::mem::size_of::<LifetimeTracked>();
        }
    });
    assert!(((*(*&raw const copyCount_2).get()) == (0)));
    assert!(((unsafe { f.call() }) == (::std::mem::size_of::<LifetimeTracked>())));
    assert!(((*(*&raw const copyCount_2).get()) == (0)));
    assert!(((*(*&raw const useCount_4).get()) == (0)));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct p7_member_access_captures_this_HasMemberUsedInLambda {
    pub x: LifetimeTracked,
    pub n: i32,
}
impl p7_member_access_captures_this_HasMemberUsedInLambda {
    pub unsafe fn makeLambda(&mut self) -> FnPtr<fn() -> i32> {
        return lambda_unsafe!(
            {
                let this_: *mut p7_member_access_captures_this_HasMemberUsedInLambda =
                    (self as *mut p7_member_access_captures_this_HasMemberUsedInLambda);
            },
            || -> i32 {
                (unsafe { LifetimeTracked::use_(&(*this_).x) });
                return (*this_).n;
            }
        );
    }
    pub unsafe fn copy_from(
        this: *mut p7_member_access_captures_this_HasMemberUsedInLambda,
        _a0: *const p7_member_access_captures_this_HasMemberUsedInLambda,
    ) {
        LifetimeTracked::copy_from((&raw mut (*this).x), { &(*_a0).x });
        (&raw mut (*this).n).write((*_a0).n);
    }
    pub unsafe fn new_1(this: *mut p7_member_access_captures_this_HasMemberUsedInLambda) {
        LifetimeTracked::new((&raw mut (*this).x));
        (&raw mut (*this).n).write(1);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const p7_member_access_captures_this_HasMemberUsedInLambda,
    ) -> *mut p7_member_access_captures_this_HasMemberUsedInLambda {
        (unsafe {
            let _arg0: *const LifetimeTracked = &(*_a0).x;
            LifetimeTracked::copy_assign(&mut self.x, _arg0)
        });
        self.n = (*_a0).n;
        return &mut (*(self as *mut p7_member_access_captures_this_HasMemberUsedInLambda));
    }
}
impl p7_member_access_captures_this_HasMemberUsedInLambda {
    pub unsafe fn destructor(&mut self) {
        LifetimeTracked::destructor(&mut self.x);
    }
}
impl Default for p7_member_access_captures_this_HasMemberUsedInLambda {
    fn default() -> Self {
        unsafe {
            p7_member_access_captures_this_HasMemberUsedInLambda {
                x: <LifetimeTracked>::default(),
                n: 1,
            }
        }
    }
}
pub unsafe fn test_22() {
    (unsafe { LifetimeTracked::reset() });
    let mut c: p7_member_access_captures_this_HasMemberUsedInLambda =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p7_member_access_captures_this_HasMemberUsedInLambda::new_1(&raw mut c);
    let _dtor_c = std::mem::DropGuard::new(&raw mut c, |__p| unsafe { (*__p).destructor() });
    let mut f: FnPtr<fn() -> i32> =
        (unsafe { p7_member_access_captures_this_HasMemberUsedInLambda::makeLambda(&mut c) });
    assert!(((*(*&raw const copyCount_2).get()) == (0)));
    assert!(((unsafe { f.call() }) == (1)));
    assert!(((c.x.uses) == (1)));
    c.n = 10;
    assert!(((unsafe { f.call() }) == (10)));
    assert!(((c.x.uses) == (2)));
    assert!(((*(*&raw const copyCount_2).get()) == (0)));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
    pub x: LifetimeTracked,
}
impl p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
    pub unsafe fn sizer(&mut self) -> FnPtr<fn() -> usize> {
        return FnPtr::<fn() -> usize>::new(|| -> usize {
            unsafe {
                return ::std::mem::size_of::<LifetimeTracked>();
            }
        });
    }
    pub unsafe fn copy_from(
        this: *mut p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda,
        _a0: *const p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda,
    ) {
        LifetimeTracked::copy_from((&raw mut (*this).x), { &(*_a0).x });
    }
    pub unsafe fn new_1(this: *mut p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda) {
        LifetimeTracked::new((&raw mut (*this).x));
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda,
    ) -> *mut p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
        (unsafe {
            let _arg0: *const LifetimeTracked = &(*_a0).x;
            LifetimeTracked::copy_assign(&mut self.x, _arg0)
        });
        return &mut (*(self as *mut p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda));
    }
}
impl p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
    pub unsafe fn destructor(&mut self) {
        LifetimeTracked::destructor(&mut self.x);
    }
}
pub unsafe fn test_23() {
    (unsafe { LifetimeTracked::reset() });
    let mut c: p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda::new_1(&raw mut c);
    let _dtor_c = std::mem::DropGuard::new(&raw mut c, |__p| unsafe { (*__p).destructor() });
    let mut f: FnPtr<fn() -> usize> =
        (unsafe { p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda::sizer(&mut c) });
    assert!(((unsafe { f.call() }) == (::std::mem::size_of::<LifetimeTracked>())));
    assert!(((*(*&raw const copyCount_2).get()) == (0)));
    assert!(((*(*&raw const useCount_4).get()) == (0)));
}
pub static mut noisy_calls_24: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn noisy_25() -> i32 {
    (*(*&raw const noisy_calls_24).get()).prefix_inc();
    return 1;
}
pub unsafe fn test_26() {
    (unsafe { LifetimeTracked::reset() });
    assert!(
        ((::std::mem::size_of::<LifetimeTracked>()) == (::std::mem::size_of::<LifetimeTracked>()))
    );
    assert!(((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<i32>())));
    assert!(!(false));
    assert!(!(false));
    let mut n: i32 = 3;
    assert!(((n) == (3)));
    assert!(((*(*&raw const noisy_calls_24).get()) == (0)));
    assert!(((*(*&raw const ctorCount_1).get()) == (0)));
    assert!(((unsafe { noisy_25() }) == (1)));
    assert!(((*(*&raw const noisy_calls_24).get()) == (1)));
}
pub static mut t_ctors_27: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_destructor_not_potentially_invoked_HasUndefinedDestructor {}
impl p9_destructor_not_potentially_invoked_HasUndefinedDestructor {
    pub unsafe fn new(this: *mut p9_destructor_not_potentially_invoked_HasUndefinedDestructor) {
        (*(*&raw const t_ctors_27).get()).prefix_inc();
    }
    pub unsafe fn copy_from(
        this: *mut p9_destructor_not_potentially_invoked_HasUndefinedDestructor,
        _a0: *const p9_destructor_not_potentially_invoked_HasUndefinedDestructor,
    ) {
    }
}
impl Default for p9_destructor_not_potentially_invoked_HasUndefinedDestructor {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p9_destructor_not_potentially_invoked_HasUndefinedDestructor =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p9_destructor_not_potentially_invoked_HasUndefinedDestructor::new(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn test_28() {
    let mut p: *mut p9_destructor_not_potentially_invoked_HasUndefinedDestructor = {
        let __p = Box::into_raw(Box::<
            p9_destructor_not_potentially_invoked_HasUndefinedDestructor,
        >::new_uninit())
            as *mut p9_destructor_not_potentially_invoked_HasUndefinedDestructor;
        p9_destructor_not_potentially_invoked_HasUndefinedDestructor::new(__p);
        __p
    };
    assert!(!((p).is_null()));
    assert!(((*(*&raw const t_ctors_27).get()) == (1)));
}
pub static mut ctor_calls_29: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct p9_unused_copy_constructor_HasUndefinedCopyCtor {
    pub value: i32,
}
impl p9_unused_copy_constructor_HasUndefinedCopyCtor {
    pub unsafe fn new(this: *mut p9_unused_copy_constructor_HasUndefinedCopyCtor) {
        (&raw mut (*this).value).write(3);
        (*(*&raw const ctor_calls_29).get()).prefix_inc();
    }
}
impl Default for p9_unused_copy_constructor_HasUndefinedCopyCtor {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p9_unused_copy_constructor_HasUndefinedCopyCtor =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p9_unused_copy_constructor_HasUndefinedCopyCtor::new(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn read_30(object: *const p9_unused_copy_constructor_HasUndefinedCopyCtor) -> i32 {
    return (*object).value;
}
pub unsafe fn test_31() {
    let mut a: p9_unused_copy_constructor_HasUndefinedCopyCtor =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_unused_copy_constructor_HasUndefinedCopyCtor::new(&raw mut a);
    assert!(((unsafe { read_30(&a,) }) == (3)));
    assert!(((*(*&raw const ctor_calls_29).get()) == (1)));
}
pub static mut m_ctors_32: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_unused_implicit_assignment_HasUndefinedCopyAssignment {}
impl p9_unused_implicit_assignment_HasUndefinedCopyAssignment {
    pub unsafe fn new(this: *mut p9_unused_implicit_assignment_HasUndefinedCopyAssignment) {
        (*(*&raw const m_ctors_32).get()).prefix_inc();
    }
    pub unsafe fn copy_from(
        this: *mut p9_unused_implicit_assignment_HasUndefinedCopyAssignment,
        _a0: *const p9_unused_implicit_assignment_HasUndefinedCopyAssignment,
    ) {
    }
}
impl Default for p9_unused_implicit_assignment_HasUndefinedCopyAssignment {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: p9_unused_implicit_assignment_HasUndefinedCopyAssignment =
                    ::std::mem::MaybeUninit::uninit().assume_init();
                p9_unused_implicit_assignment_HasUndefinedCopyAssignment::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment {
    pub m: p9_unused_implicit_assignment_HasUndefinedCopyAssignment,
}
impl p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment {
    pub unsafe fn new_1(
        this: *mut p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment,
    ) {
        p9_unused_implicit_assignment_HasUndefinedCopyAssignment::new((&raw mut (*this).m));
    }
    pub unsafe fn copy_from(
        this: *mut p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment,
        _a0: *const p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment,
    ) {
        p9_unused_implicit_assignment_HasUndefinedCopyAssignment::copy_from(
            (&raw mut (*this).m),
            { &(*_a0).m },
        );
    }
}
pub unsafe fn test_33() {
    let mut a: p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment::new_1(&raw mut a);
    let mut b: p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment::new_1(&raw mut b);
    assert!(((*(*&raw const m_ctors_32).get()) == (2)));
}
pub static mut else_runs_34: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn g_35() {
    {
        (*(*&raw const else_runs_34).get()).prefix_inc();
    }
}
pub unsafe fn test_36() {
    (unsafe { LifetimeTracked::reset() });
    (unsafe { g_35() });
    assert!(((*(*&raw const else_runs_34).get()) == (1)));
    assert!(((*(*&raw const ctorCount_1).get()) == (0)));
    assert!(((*(*&raw const useCount_4).get()) == (0)));
}
pub static mut else_runs_37: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn g_38(mut _a0: i32) {
    {
        (*(*&raw const else_runs_37).get()).prefix_inc();
    }
}
pub unsafe fn g_39(mut _a0: libc::c_char) {
    {
        (*(*&raw const else_runs_37).get()).prefix_inc();
    }
}
pub unsafe fn test_40() {
    (unsafe { LifetimeTracked::reset() });
    (unsafe { g_38(1) });
    (unsafe { g_39(('c' as libc::c_char)) });
    assert!(((*(*&raw const else_runs_37).get()) == (2)));
    assert!(((*(*&raw const ctorCount_1).get()) == (0)));
    assert!(((*(*&raw const useCount_4).get()) == (0)));
}
pub static mut else_runs_41: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn g_43() {
    {
        (*(*&raw const else_runs_41).get()).prefix_inc();
    }
}
pub unsafe fn test_44() {
    (unsafe { LifetimeTracked::reset() });
    (unsafe { g_43() });
    assert!(((*(*&raw const else_runs_41).get()) == (1)));
    assert!(((*(*&raw const ctorCount_1).get()) == (0)));
    assert!(((*(*&raw const useCount_4).get()) == (0)));
}
pub static mut g_calls_45: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn g_46() {
    {}
    (*(*&raw const g_calls_45).get()).prefix_inc();
}
pub unsafe fn test_47() {
    (unsafe { g_46() });
    assert!(((*(*&raw const g_calls_45).get()) == (1)));
}
pub static mut x1_48: libcc2rs::LazyStorage<*mut p13_pointer_to_incomplete_class_X> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub static mut x2_49: libcc2rs::LazyStorage<*mut p13_pointer_to_incomplete_class_X> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub unsafe fn identity_50(
    mut p: *mut p13_pointer_to_incomplete_class_X,
) -> *mut p13_pointer_to_incomplete_class_X {
    return p;
}
pub unsafe fn test_51() {
    assert!((*(*&raw const x1_48).get()).is_null());
    assert!(
        ((unsafe { identity_50((*(*&raw const x2_49).get()),) }) == (*(*&raw const x1_48).get()))
    );
    let mut pp: *mut *mut p13_pointer_to_incomplete_class_X =
        (&raw mut (*(*&raw const x1_48).get()) as *mut *mut p13_pointer_to_incomplete_class_X);
    assert!((*pp).is_null());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_5() });
    (unsafe { test_6() });
    (unsafe { test_7() });
    (unsafe { test_9() });
    (unsafe { test_11() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_17() });
    (unsafe { test_19() });
    (unsafe { test_20() });
    (unsafe { test_22() });
    (unsafe { test_23() });
    (unsafe { test_26() });
    (unsafe { test_28() });
    (unsafe { test_31() });
    (unsafe { test_33() });
    (unsafe { test_36() });
    (unsafe { test_40() });
    (unsafe { test_44() });
    (unsafe { test_47() });
    (unsafe { test_51() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct p13_pointer_to_incomplete_class_X;
unsafe impl p4_1_pure_virtual_without_definition_HasPureVirtual
    for p4_1_pure_virtual_without_definition_OverridesPureVirtual
{
    unsafe fn f(&mut self) {
        (*(*&raw const derived_calls_10).get()).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const copyCount_0).get();
    (*&raw const ctorCount_1).get();
    (*&raw const copyCount_2).get();
    (*&raw const dtorCount_3).get();
    (*&raw const useCount_4).get();
    (*&raw const left_8).get();
    (*&raw const derived_calls_10).get();
    (*&raw const global_12).get();
    (*&raw const copies_18).get();
    (*&raw const noisy_calls_24).get();
    (*&raw const t_ctors_27).get();
    (*&raw const ctor_calls_29).get();
    (*&raw const m_ctors_32).get();
    (*&raw const else_runs_34).get();
    (*&raw const else_runs_37).get();
    (*&raw const else_runs_41).get();
    (*&raw const g_calls_45).get();
    (*&raw const x1_48).get();
    (*&raw const x2_49).get();
}
