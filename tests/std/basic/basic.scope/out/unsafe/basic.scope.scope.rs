extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut level_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(11) });
pub unsafe fn test_1() {
    let mut level: i32 = 22;
    assert!(((level) == (22)));
    {
        assert!(((*(*&raw const level_0).get()) == (11)));
        (*(*&raw const level_0).get()) = 33;
    }
    assert!(((level) == (22)));
    assert!(((*(*&raw const level_0).get()) == (33)));
}
pub static mut function_calls_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn test_3() {
    let mut measure: i32 = 5;
    {
        assert!(((unsafe { measure_4() }) == (7)));
        assert!(((*(*&raw const function_calls_2).get()) == (1)));
    }
    assert!(((measure) == (5)));
}
pub unsafe fn measure_4() -> i32 {
    (*(*&raw const function_calls_2).get()).prefix_inc();
    return 7;
}
pub type p2_11_unscoped_enumerator_in_both_scopes_Direction = u32;
pub const p2_11_unscoped_enumerator_in_both_scopes_Direction_north:
    p2_11_unscoped_enumerator_in_both_scopes_Direction = 4;
pub const p2_11_unscoped_enumerator_in_both_scopes_Direction_south:
    p2_11_unscoped_enumerator_in_both_scopes_Direction = 9;
pub type p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size = u32;
pub const p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small:
    p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size = 3;
pub const p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large:
    p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size = 8;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum {}
impl p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum {
    pub unsafe fn scale(&self, mut count: i32) -> i32 {
        return ((count)
            * (p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large as i32));
    }
    pub unsafe fn new_1(this: *mut p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum) {}
    pub unsafe fn copy_from(
        this: *mut p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum,
        _a0: *const p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum,
    ) {
    }
    pub unsafe fn move_from(
        this: *mut p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum,
        _a0: *mut p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum,
    ) {
    }
}
pub unsafe fn test_5() {
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_north as i32) == (4)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_north as i32) == (4)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_south as i32) == (9)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_south as i32) == (9)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small as i32) == (3)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small as i32) == (3)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large as i32) == (8)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large as i32) == (8)));
    let mut object: p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum::new_1(&raw mut object);
    assert!(
        ((unsafe { p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum::scale(&object, 2,) })
            == (16))
    );
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small as i32) == (3)));
}
pub static mut count_6: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_12_class_name_in_own_scope_HasStaticCounter {}
impl p2_12_class_name_in_own_scope_HasStaticCounter {
    pub unsafe fn bump() {
        (*(*&raw const count_6).get()).prefix_inc();
    }
    pub unsafe fn new_1(this: *mut p2_12_class_name_in_own_scope_HasStaticCounter) {}
    pub unsafe fn copy_from(
        this: *mut p2_12_class_name_in_own_scope_HasStaticCounter,
        _a0: *const p2_12_class_name_in_own_scope_HasStaticCounter,
    ) {
    }
    pub unsafe fn move_from(
        this: *mut p2_12_class_name_in_own_scope_HasStaticCounter,
        _a0: *mut p2_12_class_name_in_own_scope_HasStaticCounter,
    ) {
    }
}
pub unsafe fn test_7() {
    (unsafe { p2_12_class_name_in_own_scope_HasStaticCounter::bump() });
    assert!(((*(*&raw const count_6).get()) == (1)));
    (unsafe { p2_12_class_name_in_own_scope_HasStaticCounter::bump() });
    assert!(((*(*&raw const count_6).get()) == (2)));
    (*(*&raw const count_6).get()) = 6;
    assert!(((*(*&raw const count_6).get()) == (6)));
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_8 {
    pub namespace_count: i32,
    pub namespace_ratio: f64,
}
impl Default for anon_8 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub static mut anon_8: libcc2rs::LazyStorage<anon_8> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(<anon_8>::default()) });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_9 {
    pub member_count: i32,
    pub member_ratio: f64,
}
impl Default for anon_9 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion {
    pub tag: i32,
    pub anon_9: anon_9,
}
impl p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion {
    pub unsafe fn read_count(&self) -> i32 {
        return self.anon_9.member_count;
    }
    pub unsafe fn new_1(
        this: *mut p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion,
    ) {
        (&raw mut (*this).anon_9).write(<anon_9>::default());
    }
    pub unsafe fn copy_from(
        this: *mut p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion,
        _a0: *const p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion,
    ) {
        (&raw mut (*this).tag).write((*_a0).tag);
        (&raw mut (*this).anon_9).write((*_a0).anon_9);
    }
    pub unsafe fn move_from(
        this: *mut p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion,
        _a0: *mut p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion,
    ) {
        (&raw mut (*this).tag).write((*_a0).tag);
        (&raw mut (*this).anon_9).write((*_a0).anon_9);
    }
}
pub unsafe fn test_10() {
    (*(*&raw const anon_8).get()).namespace_count = 4;
    assert!((((*(*&raw const anon_8).get()).namespace_count) == (4)));
    (*(*&raw const anon_8).get()).namespace_ratio = 0.25_f64;
    assert!((((*(*&raw const anon_8).get()).namespace_ratio) == (0.25_f64)));
    let mut object: p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion::new_1(&raw mut object);
    object.tag = 1;
    object.anon_9.member_count = 6;
    assert!(((object.anon_9.member_count) == (6)));
    assert!(
        ((unsafe {
            p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion::read_count(&object)
        }) == (6))
    );
    object.anon_9.member_ratio = 0.5_f64;
    assert!(((object.anon_9.member_ratio) == (0.5_f64)));
    assert!(((object.tag) == (1)));
    let mut anon_11: anon_11 = <anon_11>::default();
    anon_11.block_count = 9;
    assert!(((anon_11.block_count) == (9)));
    anon_11.block_ratio = 0.75_f64;
    assert!(((anon_11.block_ratio) == (0.75_f64)));
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_11 {
    pub block_count: i32,
    pub block_ratio: f64,
}
impl Default for anon_11 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p3_2_const_qualified_member_is_distinct_HasConstOverload {
    pub plain_calls: i32,
    pub const_calls: i32,
}
impl p3_2_const_qualified_member_is_distinct_HasConstOverload {
    pub unsafe fn get_1(&mut self) -> i32 {
        self.plain_calls.prefix_inc();
        return 1;
    }
    pub unsafe fn get_2(&self) -> i32 {
        (*(&raw const self.const_calls).cast_mut()).prefix_inc();
        return 2;
    }
    pub unsafe fn new_1(this: *mut p3_2_const_qualified_member_is_distinct_HasConstOverload) {
        (&raw mut (*this).plain_calls).write(0);
        (&raw mut (*this).const_calls).write(0);
    }
    pub unsafe fn copy_from(
        this: *mut p3_2_const_qualified_member_is_distinct_HasConstOverload,
        _a0: *const p3_2_const_qualified_member_is_distinct_HasConstOverload,
    ) {
        (&raw mut (*this).plain_calls).write((*_a0).plain_calls);
        (&raw mut (*this).const_calls).write((*_a0).const_calls);
    }
    pub unsafe fn move_from(
        this: *mut p3_2_const_qualified_member_is_distinct_HasConstOverload,
        _a0: *mut p3_2_const_qualified_member_is_distinct_HasConstOverload,
    ) {
        (&raw mut (*this).plain_calls).write((*_a0).plain_calls);
        (&raw mut (*this).const_calls).write((*_a0).const_calls);
    }
}
impl Default for p3_2_const_qualified_member_is_distinct_HasConstOverload {
    fn default() -> Self {
        unsafe {
            p3_2_const_qualified_member_is_distinct_HasConstOverload {
                plain_calls: 0,
                const_calls: 0,
            }
        }
    }
}
pub unsafe fn test_12() {
    let mut object: p3_2_const_qualified_member_is_distinct_HasConstOverload =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p3_2_const_qualified_member_is_distinct_HasConstOverload::new_1(&raw mut object);
    let view: *const p3_2_const_qualified_member_is_distinct_HasConstOverload = &object;
    assert!(
        ((unsafe {
            p3_2_const_qualified_member_is_distinct_HasConstOverload::get_1(&mut object)
        }) == (1))
    );
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (0)));
    assert!(
        ((unsafe { p3_2_const_qualified_member_is_distinct_HasConstOverload::get_2(&(*view),) })
            == (2))
    );
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (1)));
}
pub static mut explicit_calls_13: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut rvalue_calls_14: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
    pub base: i32,
}
impl p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
    pub unsafe fn pick_1(
        self_: *mut p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload,
        mut amount: i32,
    ) -> i32 {
        (*(*&raw const explicit_calls_13).get()).prefix_inc();
        return (((*self_).base) + (amount));
    }
    pub unsafe fn pick_2(&mut self, mut amount: i32) -> i32 {
        (*(*&raw const rvalue_calls_14).get()).prefix_inc();
        return (((self.base) + (amount)) + (100));
    }
    pub unsafe fn new_1(
        this: *mut p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload,
    ) {
        (&raw mut (*this).base).write(10);
    }
    pub unsafe fn copy_from(
        this: *mut p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload,
        _a0: *const p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload,
    ) {
        (&raw mut (*this).base).write((*_a0).base);
    }
    pub unsafe fn move_from(
        this: *mut p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload,
        _a0: *mut p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload,
    ) {
        (&raw mut (*this).base).write((*_a0).base);
    }
}
impl Default for p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
    fn default() -> Self {
        unsafe {
            p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
                base: 10,
            }
        }
    }
}
pub unsafe fn test_15() {
    let mut object: p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload::new_1(
        &raw mut object,
    );
    assert!(
        ((unsafe {
            p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload::pick_1(
                &mut object,
                1,
            )
        }) == (11))
    );
    assert!(((*(*&raw const explicit_calls_13).get()) == (1)));
    assert!(((*(*&raw const rvalue_calls_14).get()) == (0)));
    assert!(
        ((unsafe {
            p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload :: pick_2 ( &mut *(|__p: *mut p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload | { { let __d = __p ;
 __d.write_bytes(0, 1) ;
 p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload :: new_1 ( __d , )  } ;
 __p } )((&mut ::std::mem::MaybeUninit::<p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload >::uninit()).as_mut_ptr())  , 2 , )
        }) == (112))
    );
    assert!(((*(*&raw const explicit_calls_13).get()) == (1)));
    assert!(((*(*&raw const rvalue_calls_14).get()) == (1)));
}
pub static mut lvalue_calls_16: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut rvalue_calls_17: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads {}
impl p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads {
    pub unsafe fn get_1(&mut self) -> i32 {
        (*(*&raw const lvalue_calls_16).get()).prefix_inc();
        return 3;
    }
    pub unsafe fn get_2(&mut self) -> i32 {
        (*(*&raw const rvalue_calls_17).get()).prefix_inc();
        return 4;
    }
    pub unsafe fn new_1(
        this: *mut p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads,
    ) {
    }
    pub unsafe fn copy_from(
        this: *mut p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads,
        _a0: *const p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads,
    ) {
    }
    pub unsafe fn move_from(
        this: *mut p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads,
        _a0: *mut p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads,
    ) {
    }
}
pub unsafe fn test_18() {
    let mut object: p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads::new_1(&raw mut object);
    assert!(
        ((unsafe {
            p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads::get_1(&mut object)
        }) == (3))
    );
    assert!(((*(*&raw const lvalue_calls_16).get()) == (1)));
    assert!(((*(*&raw const rvalue_calls_17).get()) == (0)));
    assert!(
        ((unsafe {
            p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads :: get_2 ( &mut *(|__p: *mut p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads | { { let __d = __p ;
 __d.write_bytes(0, 1) ;
 p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads :: new_1 ( __d , )  } ;
 __p } )((&mut ::std::mem::MaybeUninit::<p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads >::uninit()).as_mut_ptr())  , )
        }) == (4))
    );
    assert!(((*(*&raw const lvalue_calls_16).get()) == (1)));
    assert!(((*(*&raw const rvalue_calls_17).get()) == (1)));
    assert!(
        ((unsafe {
            p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads::get_2(&mut object)
        }) == (4))
    );
    assert!(((*(*&raw const lvalue_calls_16).get()) == (1)));
    assert!(((*(*&raw const rvalue_calls_17).get()) == (2)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
    pub plain_calls: i32,
    pub const_calls: i32,
}
impl p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
    pub unsafe fn new_1(
        this: *mut p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload,
    ) {
        (&raw mut (*this).plain_calls).write(0);
        (&raw mut (*this).const_calls).write(0);
    }
    pub unsafe fn copy_from(
        this: *mut p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload,
        _a0: *const p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload,
    ) {
        (&raw mut (*this).plain_calls).write((*_a0).plain_calls);
        (&raw mut (*this).const_calls).write((*_a0).const_calls);
    }
    pub unsafe fn move_from(
        this: *mut p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload,
        _a0: *mut p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload,
    ) {
        (&raw mut (*this).plain_calls).write((*_a0).plain_calls);
        (&raw mut (*this).const_calls).write((*_a0).const_calls);
    }
    pub unsafe fn get_1_int(&mut self, mut value: i32) -> i32 {
        self.plain_calls.prefix_inc();
        return ((value) + (1));
    }
    pub unsafe fn get_2_int(&self, mut value: i32) -> i32 {
        (*(&raw const self.const_calls).cast_mut()).prefix_inc();
        return ((value) + (2));
    }
}
impl Default for p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
    fn default() -> Self {
        unsafe {
            p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
                plain_calls: 0,
                const_calls: 0,
            }
        }
    }
}
pub unsafe fn test_19() {
    let mut object: p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload::new_1(
        &raw mut object,
    );
    let view: *const p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload =
        &object;
    assert!(
        ((unsafe {
            p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload::get_1_int(
                &mut object,
                10,
            )
        }) == (11))
    );
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (0)));
    assert!(
        ((unsafe {
            p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload::get_2_int(
                &(*view),
                10,
            )
        }) == (12))
    );
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (1)));
}
pub static mut lvalue_calls_20: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut rvalue_calls_21: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads {}
impl p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads {
    pub unsafe fn new_1(
        this: *mut p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads,
    ) {
    }
    pub unsafe fn copy_from(
        this: *mut p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads,
        _a0 : *const p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads,
    ) {
    }
    pub unsafe fn move_from(
        this: *mut p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads,
        _a0: *mut p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads,
    ) {
    }
    pub unsafe fn get_1_int(&mut self, mut value: i32) -> i32 {
        (*(*&raw const lvalue_calls_20).get()).prefix_inc();
        return ((value) + (3));
    }
    pub unsafe fn get_2_int(&mut self, mut value: i32) -> i32 {
        (*(*&raw const rvalue_calls_21).get()).prefix_inc();
        return ((value) + (4));
    }
}
pub unsafe fn test_22() {
    let mut object : p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads = ::std::mem::MaybeUninit::uninit().assume_init() ;
    p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads::new_1(
        &raw mut object,
    );
    assert!(
        ((unsafe {
            p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads :: get_1_int ( &mut object  , 10 , )
        }) == (13))
    );
    assert!(((*(*&raw const lvalue_calls_20).get()) == (1)));
    assert!(((*(*&raw const rvalue_calls_21).get()) == (0)));
    assert!(
        ((unsafe {
            p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads :: get_2_int ( &mut *(|__p: *mut p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads | { { let __d = __p ;
 __d.write_bytes(0, 1) ;
 p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads :: new_1 ( __d , )  } ;
 __p } )((&mut ::std::mem::MaybeUninit::<p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads >::uninit()).as_mut_ptr())  , 10 , )
        }) == (14))
    );
    assert!(((*(*&raw const lvalue_calls_20).get()) == (1)));
    assert!(((*(*&raw const rvalue_calls_21).get()) == (1)));
    assert!(
        ((unsafe {
            p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads :: get_2_int ( &mut object  , 10 , )
        }) == (14))
    );
    assert!(((*(*&raw const lvalue_calls_20).get()) == (1)));
    assert!(((*(*&raw const rvalue_calls_21).get()) == (2)));
}
pub static mut friend_calls_23: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_friend_definition_defines_earlier_friend_declaration_DeclaresFriend {}
impl p4_friend_definition_defines_earlier_friend_declaration_DeclaresFriend {
    pub unsafe fn new_1(
        this: *mut p4_friend_definition_defines_earlier_friend_declaration_DeclaresFriend,
    ) {
    }
    pub unsafe fn copy_from(
        this: *mut p4_friend_definition_defines_earlier_friend_declaration_DeclaresFriend,
        _a0: *const p4_friend_definition_defines_earlier_friend_declaration_DeclaresFriend,
    ) {
    }
    pub unsafe fn move_from(
        this: *mut p4_friend_definition_defines_earlier_friend_declaration_DeclaresFriend,
        _a0: *mut p4_friend_definition_defines_earlier_friend_declaration_DeclaresFriend,
    ) {
    }
}
pub unsafe fn shared_friend_24() -> i32 {
    (*(*&raw const friend_calls_23).get()).prefix_inc();
    return 7;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_friend_definition_defines_earlier_friend_declaration_DefinesFriend {}
impl p4_friend_definition_defines_earlier_friend_declaration_DefinesFriend {
    pub unsafe fn new_1(
        this: *mut p4_friend_definition_defines_earlier_friend_declaration_DefinesFriend,
    ) {
    }
    pub unsafe fn copy_from(
        this: *mut p4_friend_definition_defines_earlier_friend_declaration_DefinesFriend,
        _a0: *const p4_friend_definition_defines_earlier_friend_declaration_DefinesFriend,
    ) {
    }
    pub unsafe fn move_from(
        this: *mut p4_friend_definition_defines_earlier_friend_declaration_DefinesFriend,
        _a0: *mut p4_friend_definition_defines_earlier_friend_declaration_DefinesFriend,
    ) {
    }
}
pub unsafe fn test_25() {
    assert!(((unsafe { shared_friend_24() }) == (7)));
    assert!(((*(*&raw const friend_calls_23).get()) == (1)));
}
pub static mut library_calls_26: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut local_calls_27: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn describe_28(mut amount: i32) -> i32 {
    (*(*&raw const library_calls_26).get()).prefix_inc();
    return ((amount) + (10));
}
pub unsafe fn describe_29(mut amount: f64) -> i32 {
    (*(*&raw const local_calls_27).get()).prefix_inc();
    return ((amount as i32) + (20));
}
pub unsafe fn test_30() {
    assert!(((unsafe { describe_28(1,) }) == (11)));
    assert!(((*(*&raw const library_calls_26).get()) == (1)));
    assert!(((*(*&raw const local_calls_27).get()) == (0)));
    assert!(((unsafe { describe_29(2_f64,) }) == (22)));
    assert!(((*(*&raw const library_calls_26).get()) == (1)));
    assert!(((*(*&raw const local_calls_27).get()) == (1)));
}
pub static mut function_calls_31: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_2_class_and_function_share_name_Reading {
    pub value: i32,
}
impl p4_2_class_and_function_share_name_Reading {
    pub unsafe fn new_1(this: *mut p4_2_class_and_function_share_name_Reading) {}
    pub unsafe fn copy_from(
        this: *mut p4_2_class_and_function_share_name_Reading,
        _a0: *const p4_2_class_and_function_share_name_Reading,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(
        this: *mut p4_2_class_and_function_share_name_Reading,
        _a0: *mut p4_2_class_and_function_share_name_Reading,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub unsafe fn Reading_32(mut raw_: i32) -> i32 {
    (*(*&raw const function_calls_31).get()).prefix_inc();
    return ((raw_) + (1));
}
pub unsafe fn test_33() {
    let mut reading: p4_2_class_and_function_share_name_Reading =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut reading;
        (&raw mut (*__d).value).write(4);
    };
    assert!(((reading.value) == (4)));
    assert!(((*(*&raw const function_calls_31).get()) == (0)));
    assert!(((unsafe { Reading_32(reading.value,) }) == (5)));
    assert!(((*(*&raw const function_calls_31).get()) == (1)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_2_class_and_variable_share_name_Limit {
    pub value: i32,
}
impl p4_2_class_and_variable_share_name_Limit {
    pub unsafe fn new_1(this: *mut p4_2_class_and_variable_share_name_Limit) {}
    pub unsafe fn copy_from(
        this: *mut p4_2_class_and_variable_share_name_Limit,
        _a0: *const p4_2_class_and_variable_share_name_Limit,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(
        this: *mut p4_2_class_and_variable_share_name_Limit,
        _a0: *mut p4_2_class_and_variable_share_name_Limit,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub static mut Limit_34: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(12) });
pub unsafe fn test_35() {
    let mut limit: p4_2_class_and_variable_share_name_Limit =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut limit;
        (&raw mut (*__d).value).write(3);
    };
    assert!(((limit.value) == (3)));
    assert!(((*(*&raw const Limit_34).get()) == (12)));
    (*(*&raw const Limit_34).get()) += limit.value;
    assert!(((*(*&raw const Limit_34).get()) == (15)));
    assert!(((limit.value) == (3)));
}
pub static mut int_calls_36: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut unit_calls_37: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub type p4_3_1_typedef_parameter_redeclares_function_Unit = i32;
pub const p4_3_1_typedef_parameter_redeclares_function_Unit_single:
    p4_3_1_typedef_parameter_redeclares_function_Unit = 1;
pub unsafe fn describe_38(mut unit: p4_3_1_typedef_parameter_redeclares_function_Unit) -> i32 {
    (*(*&raw const unit_calls_37).get()).prefix_inc();
    return ((unit as i32) + (20));
}
pub unsafe fn test_39() {
    assert!(((unsafe { describe_40(3,) }) == (13)));
    assert!(((*(*&raw const int_calls_36).get()) == (1)));
    assert!(((*(*&raw const unit_calls_37).get()) == (0)));
    assert!(
        ((unsafe { describe_38(p4_3_1_typedef_parameter_redeclares_function_Unit_single,) })
            == (21))
    );
    assert!(((*(*&raw const int_calls_36).get()) == (1)));
    assert!(((*(*&raw const unit_calls_37).get()) == (1)));
}
pub unsafe fn describe_40(mut amount: i32) -> i32 {
    (*(*&raw const int_calls_36).get()).prefix_inc();
    return ((amount) + (10));
}
pub static mut value_calls_41: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut pointer_calls_42: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn describe_43(mut renamed: i32) -> i32 {
    (*(*&raw const value_calls_41).get()).prefix_inc();
    return ((renamed) + (10));
}
pub unsafe fn describe_44(mut pointer: *mut i32) -> i32 {
    (*(*&raw const pointer_calls_42).get()).prefix_inc();
    return ((*pointer) + (20));
}
pub unsafe fn test_45() {
    let mut amount: i32 = 3;
    assert!(((unsafe { describe_43(amount,) }) == (13)));
    assert!(((*(*&raw const value_calls_41).get()) == (1)));
    assert!(((*(*&raw const pointer_calls_42).get()) == (0)));
    assert!(((unsafe { describe_44((&mut amount as *mut i32),) }) == (23)));
    assert!(((*(*&raw const value_calls_41).get()) == (1)));
    assert!(((*(*&raw const pointer_calls_42).get()) == (1)));
}
pub static mut value_calls_46: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut pointer_calls_47: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn describe_48(mut value: i32) -> i32 {
    (*(*&raw const value_calls_46).get()).prefix_inc();
    return ((value) + (10));
}
pub unsafe fn describe_49(mut pointer: *mut i32) -> i32 {
    (*(*&raw const pointer_calls_47).get()).prefix_inc();
    return ((*pointer) + (20));
}
pub unsafe fn test_50() {
    let mut amount: i32 = 3;
    assert!(((unsafe { describe_48(amount,) }) == (13)));
    assert!(((*(*&raw const value_calls_46).get()) == (1)));
    assert!(((*(*&raw const pointer_calls_47).get()) == (0)));
    assert!(((unsafe { describe_49((&mut amount as *mut i32),) }) == (23)));
    assert!(((*(*&raw const value_calls_46).get()) == (1)));
    assert!(((*(*&raw const pointer_calls_47).get()) == (1)));
}
pub static mut type_parameter_calls_51: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut value_parameter_calls_52: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn describe_53(mut value: i32) -> i32 {
    (*(*&raw const type_parameter_calls_51).get()).prefix_inc();
    return ((value) + (10));
}
pub unsafe fn describe_54() -> i32 {
    (*(*&raw const value_parameter_calls_52).get()).prefix_inc();
    return ((3) + (20));
}
pub unsafe fn test_55() {
    assert!(((unsafe { describe_53(3,) }) == (13)));
    assert!(((*(*&raw const type_parameter_calls_51).get()) == (1)));
    assert!(((*(*&raw const value_parameter_calls_52).get()) == (0)));
    assert!(((unsafe { describe_54() }) == (23)));
    assert!(((*(*&raw const type_parameter_calls_51).get()) == (1)));
    assert!(((*(*&raw const value_parameter_calls_52).get()) == (1)));
}
pub static mut version_calls_56: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn version_58() -> i32 {
    (*(*&raw const version_calls_56).get()).prefix_inc();
    return 2;
}
pub static mut revision_57: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(40) });
pub unsafe fn test_59() {
    assert!(((unsafe { version_58() }) == (2)));
    assert!(((*(*&raw const version_calls_56).get()) == (1)));
    assert!(((unsafe { version_58() }) == (2)));
    assert!(((*(*&raw const version_calls_56).get()) == (2)));
    assert!(
        ((&raw mut (*(*&raw const revision_57).get()) as *mut i32)
            == (&raw mut (*(*&raw const revision_57).get()) as *mut i32))
    );
    (*(*&raw const revision_57).get()) = 41;
    assert!(((*(*&raw const revision_57).get()) == (41)));
}
pub unsafe fn next_id_60() -> i32 {
    return (*(*(*&raw const last_61).get()).prefix_inc());
}
pub unsafe fn next_id_62() -> i32 {
    return (*(*(*&raw const last_63).get()).prefix_inc());
}
pub unsafe fn test_64() {
    assert!(((unsafe { next_id_60() }) == (1)));
    assert!(((unsafe { next_id_60() }) == (2)));
    assert!(((unsafe { next_id_62() }) == (1)));
    assert!(((unsafe { next_id_60() }) == (3)));
    assert!(((unsafe { next_id_62() }) == (2)));
}
static mut last_61: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
static mut last_63: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_3() });
    (unsafe { test_5() });
    (unsafe { test_7() });
    (unsafe { test_10() });
    (unsafe { test_12() });
    (unsafe { test_15() });
    (unsafe { test_18() });
    (unsafe { test_19() });
    (unsafe { test_22() });
    (unsafe { test_25() });
    (unsafe { test_30() });
    (unsafe { test_33() });
    (unsafe { test_35() });
    (unsafe { test_39() });
    (unsafe { test_45() });
    (unsafe { test_50() });
    (unsafe { test_55() });
    (unsafe { test_59() });
    (unsafe { test_64() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const level_0).get();
    (*&raw const function_calls_2).get();
    (*&raw const count_6).get();
    (*&raw const anon_8).get();
    (*&raw const explicit_calls_13).get();
    (*&raw const rvalue_calls_14).get();
    (*&raw const lvalue_calls_16).get();
    (*&raw const rvalue_calls_17).get();
    (*&raw const lvalue_calls_20).get();
    (*&raw const rvalue_calls_21).get();
    (*&raw const friend_calls_23).get();
    (*&raw const library_calls_26).get();
    (*&raw const local_calls_27).get();
    (*&raw const function_calls_31).get();
    (*&raw const Limit_34).get();
    (*&raw const int_calls_36).get();
    (*&raw const unit_calls_37).get();
    (*&raw const value_calls_41).get();
    (*&raw const pointer_calls_42).get();
    (*&raw const value_calls_46).get();
    (*&raw const pointer_calls_47).get();
    (*&raw const type_parameter_calls_51).get();
    (*&raw const value_parameter_calls_52).get();
    (*&raw const version_calls_56).get();
    (*&raw const revision_57).get();
}
