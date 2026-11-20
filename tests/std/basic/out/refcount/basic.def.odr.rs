extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copyCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct HasConstexprCtor {
    #[offset(0)]
    pub m: i32,
}
impl HasConstexprCtor {
    pub fn reset() {
        copyCount_0.with(|rc| *rc.borrow_mut() = 0);
    }
    pub fn new(this: Ptr<HasConstexprCtor>, mut v: i32) {
        field!(this, m).write(v);
    }
    pub fn copy_from(this: Ptr<HasConstexprCtor>, o: Ptr<HasConstexprCtor>) {
        field!(this, m).write(o.with(|__s| __s.m));
        (*copyCount_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for HasConstexprCtor {
    fn clone(&self) -> Self {
        let __src: Value<HasConstexprCtor> =
            Rc::new(RefCell::new(HasConstexprCtor { m: self.m.clone() }));
        let __dst: Value<HasConstexprCtor> = Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
        HasConstexprCtor::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
thread_local!(
    pub static ctorCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static copyCount_2: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dtorCount_3: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static useCount_4: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(4)]
pub struct LifetimeTracked {
    #[offset(0)]
    pub uses: i32,
}
impl LifetimeTracked {
    pub fn reset() {
        ctorCount_1.with(|rc| {
            *rc.borrow_mut() = {
                copyCount_2.with(|rc| {
                    *rc.borrow_mut() = {
                        dtorCount_3.with(|rc| {
                            *rc.borrow_mut() = {
                                useCount_4.with(|rc| *rc.borrow_mut() = 0);
                                useCount_4.with(|rc| *rc.borrow())
                            }
                        });
                        dtorCount_3.with(|rc| *rc.borrow())
                    }
                });
                copyCount_2.with(|rc| *rc.borrow())
            }
        });
    }
    pub fn new(this: Ptr<LifetimeTracked>) {
        field!(this, uses).write(0);
        (*ctorCount_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn copy_from(this: Ptr<LifetimeTracked>, _a0: Ptr<LifetimeTracked>) {
        field!(this, uses).write(0);
        (*copyCount_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        let __src: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked {
            uses: self.uses.clone(),
        }));
        let __dst: Value<LifetimeTracked> = Rc::new(RefCell::new(<LifetimeTracked>::uninit()));
        LifetimeTracked::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<LifetimeTracked> = Rc::new(RefCell::new(<LifetimeTracked>::uninit()));
            LifetimeTracked::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_5() {
    ({ HasConstexprCtor::reset() });
    let arr: Value<Box<[HasConstexprCtor]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <HasConstexprCtor>::uninit())
            .collect::<Box<[_]>>(),
    ));
    {
        let __d = arr.decay();
        HasConstexprCtor::new(__d.offset(0), { 3 });
        HasConstexprCtor::new(__d.offset(1), { 4 });
    };
    let f: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return {
                    [
                        Rc::try_unwrap({
                            let __v: Value<HasConstexprCtor> =
                                Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
                            HasConstexprCtor::new(__v.as_pointer(), { 3 });
                            __v
                        })
                        .ok()
                        .unwrap()
                        .into_inner(),
                        Rc::try_unwrap({
                            let __v: Value<HasConstexprCtor> =
                                Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
                            HasConstexprCtor::new(__v.as_pointer(), { 4 });
                            __v
                        })
                        .ok()
                        .unwrap()
                        .into_inner(),
                    ][(1) as usize]
                        .m
                };
            }
        })));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 4) }));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
}
pub fn test_6() {
    ({ HasConstexprCtor::reset() });
    let k: Value<HasConstexprCtor> = Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
    HasConstexprCtor::new(k.as_pointer(), { 5 });
    let f: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return {
                    Rc::try_unwrap({
                        let __v: Value<HasConstexprCtor> =
                            Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
                        HasConstexprCtor::new(__v.as_pointer(), { 5 });
                        __v
                    })
                    .ok()
                    .unwrap()
                    .into_inner()
                    .m
                };
            }
        })));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
}
pub fn test_7() {
    ({ HasConstexprCtor::reset() });
    let k1: Value<HasConstexprCtor> = Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
    HasConstexprCtor::new(k1.as_pointer(), { 7 });
    let k2: Value<HasConstexprCtor> = Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
    HasConstexprCtor::new(k2.as_pointer(), { 9 });
    let f: Value<FnPtr<fn(bool) -> i32>> = Rc::new(RefCell::new(FnPtr::<fn(bool) -> i32>::new(
        |mut b: bool| -> i32 {
            {
                return {
                    (if b {
                        Rc::try_unwrap({
                            let __v: Value<HasConstexprCtor> =
                                Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
                            HasConstexprCtor::new(__v.as_pointer(), { 7 });
                            __v
                        })
                        .ok()
                        .unwrap()
                        .into_inner()
                    } else {
                        Rc::try_unwrap({
                            let __v: Value<HasConstexprCtor> =
                                Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
                            HasConstexprCtor::new(__v.as_pointer(), { 9 });
                            __v
                        })
                        .ok()
                        .unwrap()
                        .into_inner()
                    })
                    .m
                };
            }
        },
    )));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call(true,) }) == 7) }));
    assert!(({ (({ (*f.borrow()).call(false,) }) == 9) }));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static left_8: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn test_9() {
    ({ HasConstexprCtor::reset() });
    let k: Value<HasConstexprCtor> = Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
    HasConstexprCtor::new(k.as_pointer(), { 5 });
    let f: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return {
                    {
                        (*left_8.with(Value::clone).borrow_mut()).prefix_inc();
                        Rc::try_unwrap({
                            let __v: Value<HasConstexprCtor> =
                                Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
                            HasConstexprCtor::new(__v.as_pointer(), { 5 });
                            __v
                        })
                        .ok()
                        .unwrap()
                        .into_inner()
                    }
                    .m
                };
            }
        })));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
    assert!((left_8.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((left_8.with(|rc| *rc.borrow()) == 1));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static derived_calls_10: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait p4_1_pure_virtual_without_definition_HasPureVirtual {
    fn f(&mut self);
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p4_1_pure_virtual_without_definition_OverridesPureVirtual {}
impl p4_1_pure_virtual_without_definition_OverridesPureVirtual {
    pub fn new_1(this: Ptr<p4_1_pure_virtual_without_definition_OverridesPureVirtual>) {}
    pub fn copy_from(
        this: Ptr<p4_1_pure_virtual_without_definition_OverridesPureVirtual>,
        _a0: Ptr<p4_1_pure_virtual_without_definition_OverridesPureVirtual>,
    ) {
    }
}
pub fn test_11() {
    let d: Value<p4_1_pure_virtual_without_definition_OverridesPureVirtual> = Rc::new(
        RefCell::new(<p4_1_pure_virtual_without_definition_OverridesPureVirtual>::uninit()),
    );
    p4_1_pure_virtual_without_definition_OverridesPureVirtual::new_1(d.as_pointer());
    let mut p: PtrDyn<dyn p4_1_pure_virtual_without_definition_HasPureVirtual> =
        (d.as_pointer()).to_dyn::<dyn p4_1_pure_virtual_without_definition_HasPureVirtual>(|w| w);
    ({ (*p.upgrade().deref_mut()).f() });
    assert!((derived_calls_10.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static global_12: Value<LifetimeTracked> = {
        let __static = {
            let __v: Value<LifetimeTracked> = Rc::new(RefCell::new(<LifetimeTracked>::uninit()));
            LifetimeTracked::new(__v.as_pointer());
            __v
        };
        libcc2rs::register_static_destructor(|| {
            global_12.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn test_13() {
    ({ LifetimeTracked::reset() });
    let r: Ptr<LifetimeTracked> = global_12.with(|v| v.as_pointer());
    let f: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        ({ LifetimeTrackedImpl::use_(&global_12.with(|v| v.as_pointer())) });
    })));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    ({ (*f.borrow()).call() });
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!(({ ({ (*global_12.with(Value::clone).borrow()).uses } == 1) }));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
}
pub fn test_14() {
    ({ LifetimeTracked::reset() });
    let t: Value<LifetimeTracked> = Rc::new(RefCell::new(<LifetimeTracked>::uninit()));
    LifetimeTracked::new(t.as_pointer());
    let _dtor_t = std::mem::DropGuard::new(t.clone(), |__p| __p.as_pointer().destructor());
    let r: Ptr<LifetimeTracked> = t.as_pointer();
    let f: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let r: Value<LifetimeTracked> = {
                let __v: Value<LifetimeTracked> =
                    Rc::new(RefCell::new(<LifetimeTracked>::uninit()));
                LifetimeTracked::copy_from(__v.as_pointer(), { (r).clone() });
                __v
            };
        },
        || {
            ({ LifetimeTrackedImpl::use_(&r.as_pointer()) });
        }
    )));
    let _dtor_f =
        std::mem::DropGuard::new(f.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 1));
    ({ (*f.borrow()).call() });
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 1));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ (*t.borrow()).uses } == 0) }));
}
pub fn test_15() {
    let n: Value<i32> = Rc::new(RefCell::new(5));
    let f: Value<FnPtr<fn() -> Ptr<i32>>> = Rc::new(RefCell::new(lambda!(
        {
            let n: Value<i32> = Rc::new(RefCell::new((*n.borrow())));
        },
        || -> Ptr<i32> {
            return (n.as_pointer());
        }
    )));
    assert!(({ (({ (*f.borrow()).call() }) != (n.as_pointer())) }));
    assert!(({ ((({ (*f.borrow()).call() }).read()) == 5) }));
}
pub fn test_16() {
    ({ HasConstexprCtor::reset() });
    let k: Value<HasConstexprCtor> = Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
    HasConstexprCtor::new(k.as_pointer(), { 5 });
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<HasConstexprCtor> = {
                let __v: Value<HasConstexprCtor> =
                    Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
                HasConstexprCtor::copy_from(__v.as_pointer(), { k.as_pointer() });
                __v
            };
        },
        || -> i32 {
            let c: Value<HasConstexprCtor> = Rc::new(RefCell::new(<HasConstexprCtor>::uninit()));
            HasConstexprCtor::copy_from(c.as_pointer(), { k.as_pointer() });
            return { (*c.borrow()).m };
        }
    )));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 2));
}
pub fn test_17() {
    let mut n: i32 = 5;
    let f: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return 5;
            }
        })));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
}
thread_local!(
    pub static copies_18: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub scratch: i32,
}
impl p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    pub fn new(this: Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>, mut v: i32) {
        field!(this, value).write(v);
        field!(this, scratch).write(0);
    }
    pub fn copy_from(
        this: Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>,
        o: Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>,
    ) {
        field!(this, value).write(o.with(|__s| __s.value));
        field!(this, scratch).write(o.with(|__s| __s.scratch));
        (*copies_18.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Clone for p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    fn clone(&self) -> Self {
        let __src: Value<p5_2_mutable_subobject_is_odr_used_HasMutableMember> = Rc::new(
            RefCell::new(p5_2_mutable_subobject_is_odr_used_HasMutableMember {
                value: self.value.clone(),
                scratch: self.scratch.clone(),
            }),
        );
        let __dst: Value<p5_2_mutable_subobject_is_odr_used_HasMutableMember> = Rc::new(
            RefCell::new(<p5_2_mutable_subobject_is_odr_used_HasMutableMember>::uninit()),
        );
        p5_2_mutable_subobject_is_odr_used_HasMutableMember::copy_from(
            __dst.as_pointer(),
            __src.as_pointer(),
        );
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
pub fn test_19() {
    let k: Value<p5_2_mutable_subobject_is_odr_used_HasMutableMember> = Rc::new(RefCell::new(
        <p5_2_mutable_subobject_is_odr_used_HasMutableMember>::uninit(),
    ));
    p5_2_mutable_subobject_is_odr_used_HasMutableMember::new(k.as_pointer(), { 5 });
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<p5_2_mutable_subobject_is_odr_used_HasMutableMember> = {
                let __v: Value<p5_2_mutable_subobject_is_odr_used_HasMutableMember> = Rc::new(
                    RefCell::new(<p5_2_mutable_subobject_is_odr_used_HasMutableMember>::uninit()),
                );
                p5_2_mutable_subobject_is_odr_used_HasMutableMember::copy_from(__v.as_pointer(), {
                    k.as_pointer()
                });
                __v
            };
        },
        || -> i32 {
            return { (*k.borrow()).value };
        }
    )));
    assert!((copies_18.with(|rc| *rc.borrow()) == 1));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((copies_18.with(|rc| *rc.borrow()) == 1));
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(8)]
pub struct p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
    #[offset(0)]
    #[byte_size(4)]
    pub a: LifetimeTracked,
    #[offset(4)]
    #[byte_size(4)]
    pub b: LifetimeTracked,
}
impl p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
    pub fn copy_from(
        this: Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>,
        _a0: Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>,
    ) {
        LifetimeTracked::copy_from(field_ptr!(this, a), { field_ptr!(_a0, a) });
        LifetimeTracked::copy_from(field_ptr!(this, b), { field_ptr!(_a0, b) });
    }
}
pub fn test_20() {
    ({ LifetimeTracked::reset() });
    let p: Value<p6_unevaluated_binding_needs_no_capture_HasTwoMembers> = Rc::new(RefCell::new(
        <p6_unevaluated_binding_needs_no_capture_HasTwoMembers>::uninit(),
    ));
    {
        let __d = p.as_pointer();
        LifetimeTracked::new(field_ptr!(__d, a));
        LifetimeTracked::new(field_ptr!(__d, b));
    };
    let _dtor_p = std::mem::DropGuard::new(p.clone(), |__p| __p.as_pointer().destructor());
    let __decomp_21: Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers> = p.as_pointer();
    let f: Value<FnPtr<fn() -> usize>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> usize>::new(|| -> usize {
            {
                return 4usize;
            }
        })));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 4usize) }));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct p7_member_access_captures_this_HasMemberUsedInLambda {
    #[offset(0)]
    #[byte_size(4)]
    pub x: LifetimeTracked,
    #[offset(4)]
    pub n: i32,
}
impl p7_member_access_captures_this_HasMemberUsedInLambda {
    pub fn copy_from(
        this: Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>,
        _a0: Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>,
    ) {
        LifetimeTracked::copy_from(field_ptr!(this, x), { field_ptr!(_a0, x) });
        field!(this, n).write(_a0.with(|__s| __s.n));
    }
    pub fn new_1(this: Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>) {
        LifetimeTracked::new(field_ptr!(this, x));
        field!(this, n).write(1);
    }
}
impl Default for p7_member_access_captures_this_HasMemberUsedInLambda {
    fn default() -> Self {
        {
            p7_member_access_captures_this_HasMemberUsedInLambda {
                x: <LifetimeTracked>::default(),
                n: 1,
            }
        }
    }
}
pub fn test_22() {
    ({ LifetimeTracked::reset() });
    let c: Value<p7_member_access_captures_this_HasMemberUsedInLambda> = Rc::new(RefCell::new(
        <p7_member_access_captures_this_HasMemberUsedInLambda>::uninit(),
    ));
    p7_member_access_captures_this_HasMemberUsedInLambda::new_1(c.as_pointer());
    let _dtor_c = std::mem::DropGuard::new(c.clone(), |__p| __p.as_pointer().destructor());
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(
        ({ p7_member_access_captures_this_HasMemberUsedInLambdaImpl::makeLambda(&c.as_pointer()) }),
    ));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 1) }));
    assert!(({ ({ (*c.borrow()).x.uses } == 1) }));
    (*c.borrow_mut()).n = 10;
    assert!(({ (({ (*f.borrow()).call() }) == 10) }));
    assert!(({ ({ (*c.borrow()).x.uses } == 2) }));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(4)]
pub struct p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
    #[offset(0)]
    #[byte_size(4)]
    pub x: LifetimeTracked,
}
impl p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
    pub fn copy_from(
        this: Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>,
        _a0: Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>,
    ) {
        LifetimeTracked::copy_from(field_ptr!(this, x), { field_ptr!(_a0, x) });
    }
    pub fn new_1(this: Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>) {
        LifetimeTracked::new(field_ptr!(this, x));
    }
}
pub fn test_23() {
    ({ LifetimeTracked::reset() });
    let c: Value<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda> = Rc::new(
        RefCell::new(<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>::uninit()),
    );
    p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda::new_1(c.as_pointer());
    let _dtor_c = std::mem::DropGuard::new(c.clone(), |__p| __p.as_pointer().destructor());
    let f: Value<FnPtr<fn() -> usize>> = Rc::new(RefCell::new(
        ({
            p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambdaImpl::sizer(&c.as_pointer())
        }),
    ));
    assert!(({ (({ (*f.borrow()).call() }) == 4usize) }));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static noisy_calls_24: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn noisy_25() -> i32 {
    (*noisy_calls_24.with(Value::clone).borrow_mut()).prefix_inc();
    return 1;
}
pub fn test_26() {
    ({ LifetimeTracked::reset() });
    assert!((4usize == 4usize));
    assert!((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i32>()));
    assert!(!(false));
    assert!(!(false));
    let mut n: i32 = 3;
    assert!((n == 3));
    assert!((noisy_calls_24.with(|rc| *rc.borrow()) == 0));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
    assert!((({ noisy_25() }) == 1));
    assert!((noisy_calls_24.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static t_ctors_27: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct p9_destructor_not_potentially_invoked_HasUndefinedDestructor {}
impl p9_destructor_not_potentially_invoked_HasUndefinedDestructor {
    pub fn new(this: Ptr<p9_destructor_not_potentially_invoked_HasUndefinedDestructor>) {
        (*t_ctors_27.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn copy_from(
        this: Ptr<p9_destructor_not_potentially_invoked_HasUndefinedDestructor>,
        _a0: Ptr<p9_destructor_not_potentially_invoked_HasUndefinedDestructor>,
    ) {
    }
}
impl Default for p9_destructor_not_potentially_invoked_HasUndefinedDestructor {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p9_destructor_not_potentially_invoked_HasUndefinedDestructor> =
                Rc::new(RefCell::new(
                    <p9_destructor_not_potentially_invoked_HasUndefinedDestructor>::uninit(),
                ));
            p9_destructor_not_potentially_invoked_HasUndefinedDestructor::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn test_28() {
    let mut p: Ptr<p9_destructor_not_potentially_invoked_HasUndefinedDestructor> = {
        let __p =
            Ptr::alloc(<p9_destructor_not_potentially_invoked_HasUndefinedDestructor>::uninit());
        p9_destructor_not_potentially_invoked_HasUndefinedDestructor::new(__p.clone());
        __p
    };
    assert!(!((p).is_null()));
    assert!((t_ctors_27.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static ctor_calls_29: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Record, ByteRepr, Uninit, FnPtrArg)]
#[byte_size(4)]
pub struct p9_unused_copy_constructor_HasUndefinedCopyCtor {
    #[offset(0)]
    pub value: i32,
}
impl p9_unused_copy_constructor_HasUndefinedCopyCtor {
    pub fn new(this: Ptr<p9_unused_copy_constructor_HasUndefinedCopyCtor>) {
        field!(this, value).write(3);
        (*ctor_calls_29.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Default for p9_unused_copy_constructor_HasUndefinedCopyCtor {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p9_unused_copy_constructor_HasUndefinedCopyCtor> = Rc::new(
                RefCell::new(<p9_unused_copy_constructor_HasUndefinedCopyCtor>::uninit()),
            );
            p9_unused_copy_constructor_HasUndefinedCopyCtor::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn read_30(object: Ptr<p9_unused_copy_constructor_HasUndefinedCopyCtor>) -> i32 {
    return object.with(|__s| __s.value);
}
pub fn test_31() {
    let a: Value<p9_unused_copy_constructor_HasUndefinedCopyCtor> = Rc::new(RefCell::new(
        <p9_unused_copy_constructor_HasUndefinedCopyCtor>::uninit(),
    ));
    p9_unused_copy_constructor_HasUndefinedCopyCtor::new(a.as_pointer());
    assert!((({ read_30(a.as_pointer(),) }) == 3));
    assert!((ctor_calls_29.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static m_ctors_32: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct p9_unused_implicit_assignment_HasUndefinedCopyAssignment {}
impl p9_unused_implicit_assignment_HasUndefinedCopyAssignment {
    pub fn new(this: Ptr<p9_unused_implicit_assignment_HasUndefinedCopyAssignment>) {
        (*m_ctors_32.with(Value::clone).borrow_mut()).prefix_inc();
    }
    pub fn copy_from(
        this: Ptr<p9_unused_implicit_assignment_HasUndefinedCopyAssignment>,
        _a0: Ptr<p9_unused_implicit_assignment_HasUndefinedCopyAssignment>,
    ) {
    }
}
impl Default for p9_unused_implicit_assignment_HasUndefinedCopyAssignment {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<p9_unused_implicit_assignment_HasUndefinedCopyAssignment> = Rc::new(
                RefCell::new(<p9_unused_implicit_assignment_HasUndefinedCopyAssignment>::uninit()),
            );
            p9_unused_implicit_assignment_HasUndefinedCopyAssignment::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment {
    #[offset(0)]
    #[byte_size(1)]
    pub m: p9_unused_implicit_assignment_HasUndefinedCopyAssignment,
}
impl p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment {
    pub fn new_1(this: Ptr<p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment>) {
        p9_unused_implicit_assignment_HasUndefinedCopyAssignment::new(field_ptr!(this, m));
    }
    pub fn copy_from(
        this: Ptr<p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment>,
        _a0: Ptr<p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment>,
    ) {
        p9_unused_implicit_assignment_HasUndefinedCopyAssignment::copy_from(field_ptr!(this, m), {
            field_ptr!(_a0, m)
        });
    }
}
pub fn test_33() {
    let a: Value<p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment> =
        Rc::new(RefCell::new(
            <p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment>::uninit(),
        ));
    p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment::new_1(a.as_pointer());
    let b: Value<p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment> =
        Rc::new(RefCell::new(
            <p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment>::uninit(),
        ));
    p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment::new_1(b.as_pointer());
    assert!((m_ctors_32.with(|rc| *rc.borrow()) == 2));
}
thread_local!(
    pub static else_runs_34: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn g_35() {
    {
        (*else_runs_34.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn test_36() {
    ({ LifetimeTracked::reset() });
    ({ g_35() });
    assert!((else_runs_34.with(|rc| *rc.borrow()) == 1));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static else_runs_37: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn g_38(mut _a0: i32) {
    {
        (*else_runs_37.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn g_39(mut _a0: i8) {
    {
        (*else_runs_37.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn test_40() {
    ({ LifetimeTracked::reset() });
    ({ g_38(1) });
    ({ g_39(('c' as i8)) });
    assert!((else_runs_37.with(|rc| *rc.borrow()) == 2));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static else_runs_41: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn g_43() {
    {
        (*else_runs_41.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn test_44() {
    ({ LifetimeTracked::reset() });
    ({ g_43() });
    assert!((else_runs_41.with(|rc| *rc.borrow()) == 1));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static g_calls_45: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn g_46() {
    {}
    (*g_calls_45.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn test_47() {
    ({ g_46() });
    assert!((g_calls_45.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static x1_48: Value<Ptr<p13_pointer_to_incomplete_class_X>> = Rc::new(RefCell::new(Ptr::<
        p13_pointer_to_incomplete_class_X,
    >::null(
    )));
);
thread_local!(
    pub static x2_49: Value<Ptr<p13_pointer_to_incomplete_class_X>> = Rc::new(RefCell::new(Ptr::<
        p13_pointer_to_incomplete_class_X,
    >::null(
    )));
);
pub fn identity_50(
    mut p: Ptr<p13_pointer_to_incomplete_class_X>,
) -> Ptr<p13_pointer_to_incomplete_class_X> {
    return (p).clone();
}
pub fn test_51() {
    assert!(({ (*x1_48.with(Value::clone).borrow()).is_null() }));
    assert!(
        ({
            ({ ({ identity_50((*x2_49.with(Value::clone).borrow()).clone()) }) } == {
                (*x1_48.with(Value::clone).borrow()).clone()
            })
        })
    );
    let mut pp: Ptr<Ptr<p13_pointer_to_incomplete_class_X>> = (x1_48.with(|v| v.as_pointer()));
    assert!((pp.read()).is_null());
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_9() });
    ({ test_11() });
    ({ test_13() });
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    ({ test_17() });
    ({ test_19() });
    ({ test_20() });
    ({ test_22() });
    ({ test_23() });
    ({ test_26() });
    ({ test_28() });
    ({ test_31() });
    ({ test_33() });
    ({ test_36() });
    ({ test_40() });
    ({ test_44() });
    ({ test_47() });
    ({ test_51() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct p13_pointer_to_incomplete_class_X;
impl p4_1_pure_virtual_without_definition_HasPureVirtual
    for p4_1_pure_virtual_without_definition_OverridesPureVirtual
{
    fn f(&mut self) {
        (*derived_calls_10.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub trait HasConstexprCtorImpl {
    fn copy_assign(&self, _a0: Ptr<HasConstexprCtor>) -> Ptr<HasConstexprCtor>;
}
impl HasConstexprCtorImpl for Ptr<HasConstexprCtor> {
    fn copy_assign(&self, _a0: Ptr<HasConstexprCtor>) -> Ptr<HasConstexprCtor> {
        field!((*self), m).write({ _a0.with(|__s| __s.m) });
        return (*self).clone();
    }
}
pub trait LifetimeTrackedImpl {
    fn destructor(&self);
    fn use_(&self);
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked>;
}
impl LifetimeTrackedImpl for Ptr<LifetimeTracked> {
    fn destructor(&self) {
        (*dtorCount_3.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn use_(&self) {
        field!((*self), uses).with_mut(|__v| {
            __v.prefix_inc();
        });
        (*useCount_4.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked> {
        field!((*self), uses).write({ _a0.with(|__s| __s.uses) });
        return (*self).clone();
    }
}
pub trait p5_2_mutable_subobject_is_odr_used_HasMutableMemberImpl {
    fn copy_assign(
        &self,
        _a0: Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>,
    ) -> Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>;
}
impl p5_2_mutable_subobject_is_odr_used_HasMutableMemberImpl
    for Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>
{
    fn copy_assign(
        &self,
        _a0: Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>,
    ) -> Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        field!((*self), scratch).write({ _a0.with(|__s| __s.scratch) });
        return (*self).clone();
    }
}
pub trait p6_unevaluated_binding_needs_no_capture_HasTwoMembersImpl {
    fn copy_assign(
        &self,
        _a0: Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>,
    ) -> Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>;
    fn destructor(&self);
}
impl p6_unevaluated_binding_needs_no_capture_HasTwoMembersImpl
    for Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>
{
    fn copy_assign(
        &self,
        _a0: Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>,
    ) -> Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers> {
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, a);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), a), _arg0)
        });
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, b);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), b), _arg0)
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, b).destructor();
        field_ptr!(self, a).destructor();
    }
}
pub trait p7_member_access_captures_this_HasMemberUsedInLambdaImpl {
    fn makeLambda(&self) -> FnPtr<fn() -> i32>;
    fn copy_assign(
        &self,
        _a0: Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>,
    ) -> Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>;
    fn destructor(&self);
}
impl p7_member_access_captures_this_HasMemberUsedInLambdaImpl
    for Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>
{
    fn makeLambda(&self) -> FnPtr<fn() -> i32> {
        return lambda!(
            {
                let this_: Value<Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>> =
                    Rc::new(RefCell::new((*self).clone()));
            },
            || -> i32 {
                ({ LifetimeTrackedImpl::use_(&field_ptr!((*this_.borrow()), x)) });
                return (*this_.borrow()).with(|__s| __s.n);
            }
        );
    }
    fn copy_assign(
        &self,
        _a0: Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>,
    ) -> Ptr<p7_member_access_captures_this_HasMemberUsedInLambda> {
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, x);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), x), _arg0)
        });
        field!((*self), n).write({ _a0.with(|__s| __s.n) });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, x).destructor();
    }
}
pub trait p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambdaImpl {
    fn sizer(&self) -> FnPtr<fn() -> usize>;
    fn copy_assign(
        &self,
        _a0: Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>,
    ) -> Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>;
    fn destructor(&self);
}
impl p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambdaImpl
    for Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>
{
    fn sizer(&self) -> FnPtr<fn() -> usize> {
        return FnPtr::<fn() -> usize>::new(|| -> usize {
            {
                return 4usize;
            }
        });
    }
    fn copy_assign(
        &self,
        _a0: Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>,
    ) -> Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda> {
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, x);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), x), _arg0)
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, x).destructor();
    }
}
pub trait p9_destructor_not_potentially_invoked_HasUndefinedDestructorImpl {
    fn destructor(&self) {
        unimplemented!()
    }
}
impl p9_destructor_not_potentially_invoked_HasUndefinedDestructorImpl
    for Ptr<p9_destructor_not_potentially_invoked_HasUndefinedDestructor>
{
}
pub trait p9_unused_implicit_assignment_HasUndefinedCopyAssignmentImpl {
    fn copy_assign(
        &self,
        _a0: Ptr<p9_unused_implicit_assignment_HasUndefinedCopyAssignment>,
    ) -> Ptr<p9_unused_implicit_assignment_HasUndefinedCopyAssignment> {
        unimplemented!()
    }
}
impl p9_unused_implicit_assignment_HasUndefinedCopyAssignmentImpl
    for Ptr<p9_unused_implicit_assignment_HasUndefinedCopyAssignment>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = copyCount_0.with(|_| ());
    let _ = ctorCount_1.with(|_| ());
    let _ = copyCount_2.with(|_| ());
    let _ = dtorCount_3.with(|_| ());
    let _ = useCount_4.with(|_| ());
    let _ = left_8.with(|_| ());
    let _ = derived_calls_10.with(|_| ());
    let _ = global_12.with(|_| ());
    let _ = copies_18.with(|_| ());
    let _ = noisy_calls_24.with(|_| ());
    let _ = t_ctors_27.with(|_| ());
    let _ = ctor_calls_29.with(|_| ());
    let _ = m_ctors_32.with(|_| ());
    let _ = else_runs_34.with(|_| ());
    let _ = else_runs_37.with(|_| ());
    let _ = else_runs_41.with(|_| ());
    let _ = g_calls_45.with(|_| ());
    let _ = x1_48.with(|_| ());
    let _ = x2_49.with(|_| ());
}
