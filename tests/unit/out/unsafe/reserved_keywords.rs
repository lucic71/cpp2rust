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
pub struct S {
    pub as_: i32,
    pub async_: i32,
    pub await_: i32,
    pub crate_: i32,
    pub dyn_: i32,
    pub fn_: i32,
    pub impl_: i32,
    pub in_: i32,
    pub let_: i32,
    pub loop_: i32,
    pub match_: i32,
    pub mod_: i32,
    pub move_: i32,
    pub mut_: i32,
    pub pub_: i32,
    pub ref_: i32,
    pub self_: i32,
    pub Self_: i32,
    pub super_: i32,
    pub trait_: i32,
    pub type_: i32,
    pub unsafe_: i32,
    pub use_: i32,
    pub where_: i32,
    pub abstract_: i32,
    pub become_: i32,
    pub box_: i32,
    pub final_: i32,
    pub gen_: i32,
    pub macro_: i32,
    pub override_: i32,
    pub priv_: i32,
    pub unsized_: i32,
    pub yield_: i32,
    pub macro_rules_: i32,
    pub raw_: i32,
    pub safe_: i32,
    pub vec_: i32,
}
impl S {
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).as_).write((*_a0).as_);
        (&raw mut (*this).async_).write((*_a0).async_);
        (&raw mut (*this).await_).write((*_a0).await_);
        (&raw mut (*this).crate_).write((*_a0).crate_);
        (&raw mut (*this).dyn_).write((*_a0).dyn_);
        (&raw mut (*this).fn_).write((*_a0).fn_);
        (&raw mut (*this).impl_).write((*_a0).impl_);
        (&raw mut (*this).in_).write((*_a0).in_);
        (&raw mut (*this).let_).write((*_a0).let_);
        (&raw mut (*this).loop_).write((*_a0).loop_);
        (&raw mut (*this).match_).write((*_a0).match_);
        (&raw mut (*this).mod_).write((*_a0).mod_);
        (&raw mut (*this).move_).write((*_a0).move_);
        (&raw mut (*this).mut_).write((*_a0).mut_);
        (&raw mut (*this).pub_).write((*_a0).pub_);
        (&raw mut (*this).ref_).write((*_a0).ref_);
        (&raw mut (*this).self_).write((*_a0).self_);
        (&raw mut (*this).Self_).write((*_a0).Self_);
        (&raw mut (*this).super_).write((*_a0).super_);
        (&raw mut (*this).trait_).write((*_a0).trait_);
        (&raw mut (*this).type_).write((*_a0).type_);
        (&raw mut (*this).unsafe_).write((*_a0).unsafe_);
        (&raw mut (*this).use_).write((*_a0).use_);
        (&raw mut (*this).where_).write((*_a0).where_);
        (&raw mut (*this).abstract_).write((*_a0).abstract_);
        (&raw mut (*this).become_).write((*_a0).become_);
        (&raw mut (*this).box_).write((*_a0).box_);
        (&raw mut (*this).final_).write((*_a0).final_);
        (&raw mut (*this).gen_).write((*_a0).gen_);
        (&raw mut (*this).macro_).write((*_a0).macro_);
        (&raw mut (*this).override_).write((*_a0).override_);
        (&raw mut (*this).priv_).write((*_a0).priv_);
        (&raw mut (*this).unsized_).write((*_a0).unsized_);
        (&raw mut (*this).yield_).write((*_a0).yield_);
        (&raw mut (*this).macro_rules_).write((*_a0).macro_rules_);
        (&raw mut (*this).raw_).write((*_a0).raw_);
        (&raw mut (*this).safe_).write((*_a0).safe_);
        (&raw mut (*this).vec_).write((*_a0).vec_);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).as_).write((*_a0).as_);
        (&raw mut (*this).async_).write((*_a0).async_);
        (&raw mut (*this).await_).write((*_a0).await_);
        (&raw mut (*this).crate_).write((*_a0).crate_);
        (&raw mut (*this).dyn_).write((*_a0).dyn_);
        (&raw mut (*this).fn_).write((*_a0).fn_);
        (&raw mut (*this).impl_).write((*_a0).impl_);
        (&raw mut (*this).in_).write((*_a0).in_);
        (&raw mut (*this).let_).write((*_a0).let_);
        (&raw mut (*this).loop_).write((*_a0).loop_);
        (&raw mut (*this).match_).write((*_a0).match_);
        (&raw mut (*this).mod_).write((*_a0).mod_);
        (&raw mut (*this).move_).write((*_a0).move_);
        (&raw mut (*this).mut_).write((*_a0).mut_);
        (&raw mut (*this).pub_).write((*_a0).pub_);
        (&raw mut (*this).ref_).write((*_a0).ref_);
        (&raw mut (*this).self_).write((*_a0).self_);
        (&raw mut (*this).Self_).write((*_a0).Self_);
        (&raw mut (*this).super_).write((*_a0).super_);
        (&raw mut (*this).trait_).write((*_a0).trait_);
        (&raw mut (*this).type_).write((*_a0).type_);
        (&raw mut (*this).unsafe_).write((*_a0).unsafe_);
        (&raw mut (*this).use_).write((*_a0).use_);
        (&raw mut (*this).where_).write((*_a0).where_);
        (&raw mut (*this).abstract_).write((*_a0).abstract_);
        (&raw mut (*this).become_).write((*_a0).become_);
        (&raw mut (*this).box_).write((*_a0).box_);
        (&raw mut (*this).final_).write((*_a0).final_);
        (&raw mut (*this).gen_).write((*_a0).gen_);
        (&raw mut (*this).macro_).write((*_a0).macro_);
        (&raw mut (*this).override_).write((*_a0).override_);
        (&raw mut (*this).priv_).write((*_a0).priv_);
        (&raw mut (*this).unsized_).write((*_a0).unsized_);
        (&raw mut (*this).yield_).write((*_a0).yield_);
        (&raw mut (*this).macro_rules_).write((*_a0).macro_rules_);
        (&raw mut (*this).raw_).write((*_a0).raw_);
        (&raw mut (*this).safe_).write((*_a0).safe_);
        (&raw mut (*this).vec_).write((*_a0).vec_);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct type_ {
    pub v: i32,
}
impl type_ {
    pub unsafe fn new_1(this: *mut type_) {}
    pub unsafe fn copy_from(this: *mut type_, _a0: *const type_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut type_, _a0: *mut type_) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct loop_ {
    pub t: type_,
}
impl loop_ {
    pub unsafe fn new_1(this: *mut loop_) {
        type_::new_1((&raw mut (*this).t));
    }
    pub unsafe fn copy_from(this: *mut loop_, _a0: *const loop_) {
        type_::copy_from((&raw mut (*this).t), { &(*_a0).t });
    }
    pub unsafe fn move_from(this: *mut loop_, _a0: *mut loop_) {
        type_::move_from((&raw mut (*this).t), { &mut (*_a0).t });
    }
}
pub unsafe fn foo_0(
    mut as_: i32,
    mut async_: i32,
    mut await_: i32,
    mut crate_: i32,
    mut dyn_: i32,
    mut fn_: i32,
    mut impl_: i32,
    mut in_: i32,
    mut let_: i32,
    mut loop_: i32,
    mut match_: i32,
    mut mod_: i32,
    mut move_: i32,
    mut mut_: i32,
    mut pub_: i32,
    mut ref_: i32,
    mut self_: i32,
    mut Self_: i32,
    mut super_: i32,
    mut trait_: i32,
    mut type_: i32,
    mut unsafe_: i32,
    mut use_: i32,
    mut where_: i32,
    mut abstract_: i32,
    mut become_: i32,
    mut box_: i32,
    mut final_: i32,
    mut gen_: i32,
    mut macro_: i32,
    mut override_: i32,
    mut priv_: i32,
    mut unsized_: i32,
    mut yield_: i32,
    mut macro_rules_: i32,
    mut raw_: i32,
    mut safe_: i32,
    mut vec_: i32,
    mut dummy: i32,
) -> i32 {
    return 0;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).as_).write(0);
        (&raw mut (*__d).async_).write(0);
        (&raw mut (*__d).await_).write(0);
        (&raw mut (*__d).crate_).write(0);
        (&raw mut (*__d).dyn_).write(0);
        (&raw mut (*__d).fn_).write(0);
        (&raw mut (*__d).impl_).write(0);
        (&raw mut (*__d).in_).write(0);
        (&raw mut (*__d).let_).write(0);
        (&raw mut (*__d).loop_).write(0);
        (&raw mut (*__d).match_).write(0);
        (&raw mut (*__d).mod_).write(0);
        (&raw mut (*__d).move_).write(0);
        (&raw mut (*__d).mut_).write(0);
        (&raw mut (*__d).pub_).write(0);
        (&raw mut (*__d).ref_).write(0);
        (&raw mut (*__d).self_).write(0);
        (&raw mut (*__d).Self_).write(0);
        (&raw mut (*__d).super_).write(0);
        (&raw mut (*__d).trait_).write(0);
        (&raw mut (*__d).type_).write(0);
        (&raw mut (*__d).unsafe_).write(0);
        (&raw mut (*__d).use_).write(0);
        (&raw mut (*__d).where_).write(0);
        (&raw mut (*__d).abstract_).write(0);
        (&raw mut (*__d).become_).write(0);
        (&raw mut (*__d).box_).write(0);
        (&raw mut (*__d).final_).write(0);
        (&raw mut (*__d).gen_).write(0);
        (&raw mut (*__d).macro_).write(0);
        (&raw mut (*__d).override_).write(0);
        (&raw mut (*__d).priv_).write(0);
        (&raw mut (*__d).unsized_).write(0);
        (&raw mut (*__d).yield_).write(0);
        (&raw mut (*__d).macro_rules_).write(0);
        (&raw mut (*__d).raw_).write(0);
        (&raw mut (*__d).safe_).write(0);
        (&raw mut (*__d).vec_).write(0);
    };
    let mut as_: i32 = 0;
    let mut async_: i32 = 0;
    let mut await_: i32 = 0;
    let mut crate_: i32 = 0;
    let mut dyn_: i32 = 0;
    let mut fn_: i32 = 0;
    let mut impl_: i32 = 0;
    let mut in_: i32 = 0;
    let mut let_: i32 = 0;
    let mut loop_: i32 = 0;
    let mut match_: i32 = 0;
    let mut mod_: i32 = 0;
    let mut move_: i32 = 0;
    let mut mut_: i32 = 0;
    let mut pub_: i32 = 0;
    let mut ref_: i32 = 0;
    let mut self_: i32 = 0;
    let mut Self_: i32 = 0;
    let mut super_: i32 = 0;
    let mut trait_: i32 = 0;
    let mut type_: i32 = 0;
    let mut unsafe_: i32 = 0;
    let mut use_: i32 = 0;
    let mut where_: i32 = 0;
    let mut abstract_: i32 = 0;
    let mut become_: i32 = 0;
    let mut box_: i32 = 0;
    let mut final_: i32 = 0;
    let mut gen_: i32 = 0;
    let mut macro_: i32 = 0;
    let mut override_: i32 = 0;
    let mut priv_: i32 = 0;
    let mut unsized_: i32 = 0;
    let mut yield_: i32 = 0;
    let mut macro_rules_: i32 = 0;
    let mut raw_: i32 = 0;
    let mut safe_: i32 = 0;
    let mut vec_: i32 = 0;
    let mut l: loop_ = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut l;
        {
            let __d = (&raw mut (*__d).t);
            (&raw mut (*__d).v).write(3);
        };
    };
    let mut t: type_ = ::std::mem::MaybeUninit::uninit().assume_init();
    type_::copy_from(&raw mut t, { &l.t });
    return (unsafe {
        foo_0(
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            ((t.v) - (3)),
        )
    });
}
pub unsafe fn __cpp2rust_init_globals() {}
