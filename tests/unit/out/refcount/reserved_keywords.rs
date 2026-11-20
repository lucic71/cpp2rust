extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(152)]
pub struct S {
    #[offset(0)]
    pub as_: i32,
    #[offset(4)]
    pub async_: i32,
    #[offset(8)]
    pub await_: i32,
    #[offset(12)]
    pub crate_: i32,
    #[offset(16)]
    pub dyn_: i32,
    #[offset(20)]
    pub fn_: i32,
    #[offset(24)]
    pub impl_: i32,
    #[offset(28)]
    pub in_: i32,
    #[offset(32)]
    pub let_: i32,
    #[offset(36)]
    pub loop_: i32,
    #[offset(40)]
    pub match_: i32,
    #[offset(44)]
    pub mod_: i32,
    #[offset(48)]
    pub move_: i32,
    #[offset(52)]
    pub mut_: i32,
    #[offset(56)]
    pub pub_: i32,
    #[offset(60)]
    pub ref_: i32,
    #[offset(64)]
    pub self_: i32,
    #[offset(68)]
    pub Self_: i32,
    #[offset(72)]
    pub super_: i32,
    #[offset(76)]
    pub trait_: i32,
    #[offset(80)]
    pub type_: i32,
    #[offset(84)]
    pub unsafe_: i32,
    #[offset(88)]
    pub use_: i32,
    #[offset(92)]
    pub where_: i32,
    #[offset(96)]
    pub abstract_: i32,
    #[offset(100)]
    pub become_: i32,
    #[offset(104)]
    pub box_: i32,
    #[offset(108)]
    pub final_: i32,
    #[offset(112)]
    pub gen_: i32,
    #[offset(116)]
    pub macro_: i32,
    #[offset(120)]
    pub override_: i32,
    #[offset(124)]
    pub priv_: i32,
    #[offset(128)]
    pub unsized_: i32,
    #[offset(132)]
    pub yield_: i32,
    #[offset(136)]
    pub macro_rules_: i32,
    #[offset(140)]
    pub raw_: i32,
    #[offset(144)]
    pub safe_: i32,
    #[offset(148)]
    pub vec_: i32,
}
impl S {
    pub fn new_1(this: Ptr<S>) {}
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, as_).write(_a0.with(|__s| __s.as_));
        field!(this, async_).write(_a0.with(|__s| __s.async_));
        field!(this, await_).write(_a0.with(|__s| __s.await_));
        field!(this, crate_).write(_a0.with(|__s| __s.crate_));
        field!(this, dyn_).write(_a0.with(|__s| __s.dyn_));
        field!(this, fn_).write(_a0.with(|__s| __s.fn_));
        field!(this, impl_).write(_a0.with(|__s| __s.impl_));
        field!(this, in_).write(_a0.with(|__s| __s.in_));
        field!(this, let_).write(_a0.with(|__s| __s.let_));
        field!(this, loop_).write(_a0.with(|__s| __s.loop_));
        field!(this, match_).write(_a0.with(|__s| __s.match_));
        field!(this, mod_).write(_a0.with(|__s| __s.mod_));
        field!(this, move_).write(_a0.with(|__s| __s.move_));
        field!(this, mut_).write(_a0.with(|__s| __s.mut_));
        field!(this, pub_).write(_a0.with(|__s| __s.pub_));
        field!(this, ref_).write(_a0.with(|__s| __s.ref_));
        field!(this, self_).write(_a0.with(|__s| __s.self_));
        field!(this, Self_).write(_a0.with(|__s| __s.Self_));
        field!(this, super_).write(_a0.with(|__s| __s.super_));
        field!(this, trait_).write(_a0.with(|__s| __s.trait_));
        field!(this, type_).write(_a0.with(|__s| __s.type_));
        field!(this, unsafe_).write(_a0.with(|__s| __s.unsafe_));
        field!(this, use_).write(_a0.with(|__s| __s.use_));
        field!(this, where_).write(_a0.with(|__s| __s.where_));
        field!(this, abstract_).write(_a0.with(|__s| __s.abstract_));
        field!(this, become_).write(_a0.with(|__s| __s.become_));
        field!(this, box_).write(_a0.with(|__s| __s.box_));
        field!(this, final_).write(_a0.with(|__s| __s.final_));
        field!(this, gen_).write(_a0.with(|__s| __s.gen_));
        field!(this, macro_).write(_a0.with(|__s| __s.macro_));
        field!(this, override_).write(_a0.with(|__s| __s.override_));
        field!(this, priv_).write(_a0.with(|__s| __s.priv_));
        field!(this, unsized_).write(_a0.with(|__s| __s.unsized_));
        field!(this, yield_).write(_a0.with(|__s| __s.yield_));
        field!(this, macro_rules_).write(_a0.with(|__s| __s.macro_rules_));
        field!(this, raw_).write(_a0.with(|__s| __s.raw_));
        field!(this, safe_).write(_a0.with(|__s| __s.safe_));
        field!(this, vec_).write(_a0.with(|__s| __s.vec_));
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, as_).write({ (*_a0.upgrade().deref()).as_ });
        field!(this, async_).write({ (*_a0.upgrade().deref()).async_ });
        field!(this, await_).write({ (*_a0.upgrade().deref()).await_ });
        field!(this, crate_).write({ (*_a0.upgrade().deref()).crate_ });
        field!(this, dyn_).write({ (*_a0.upgrade().deref()).dyn_ });
        field!(this, fn_).write({ (*_a0.upgrade().deref()).fn_ });
        field!(this, impl_).write({ (*_a0.upgrade().deref()).impl_ });
        field!(this, in_).write({ (*_a0.upgrade().deref()).in_ });
        field!(this, let_).write({ (*_a0.upgrade().deref()).let_ });
        field!(this, loop_).write({ (*_a0.upgrade().deref()).loop_ });
        field!(this, match_).write({ (*_a0.upgrade().deref()).match_ });
        field!(this, mod_).write({ (*_a0.upgrade().deref()).mod_ });
        field!(this, move_).write({ (*_a0.upgrade().deref()).move_ });
        field!(this, mut_).write({ (*_a0.upgrade().deref()).mut_ });
        field!(this, pub_).write({ (*_a0.upgrade().deref()).pub_ });
        field!(this, ref_).write({ (*_a0.upgrade().deref()).ref_ });
        field!(this, self_).write({ (*_a0.upgrade().deref()).self_ });
        field!(this, Self_).write({ (*_a0.upgrade().deref()).Self_ });
        field!(this, super_).write({ (*_a0.upgrade().deref()).super_ });
        field!(this, trait_).write({ (*_a0.upgrade().deref()).trait_ });
        field!(this, type_).write({ (*_a0.upgrade().deref()).type_ });
        field!(this, unsafe_).write({ (*_a0.upgrade().deref()).unsafe_ });
        field!(this, use_).write({ (*_a0.upgrade().deref()).use_ });
        field!(this, where_).write({ (*_a0.upgrade().deref()).where_ });
        field!(this, abstract_).write({ (*_a0.upgrade().deref()).abstract_ });
        field!(this, become_).write({ (*_a0.upgrade().deref()).become_ });
        field!(this, box_).write({ (*_a0.upgrade().deref()).box_ });
        field!(this, final_).write({ (*_a0.upgrade().deref()).final_ });
        field!(this, gen_).write({ (*_a0.upgrade().deref()).gen_ });
        field!(this, macro_).write({ (*_a0.upgrade().deref()).macro_ });
        field!(this, override_).write({ (*_a0.upgrade().deref()).override_ });
        field!(this, priv_).write({ (*_a0.upgrade().deref()).priv_ });
        field!(this, unsized_).write({ (*_a0.upgrade().deref()).unsized_ });
        field!(this, yield_).write({ (*_a0.upgrade().deref()).yield_ });
        field!(this, macro_rules_).write({ (*_a0.upgrade().deref()).macro_rules_ });
        field!(this, raw_).write({ (*_a0.upgrade().deref()).raw_ });
        field!(this, safe_).write({ (*_a0.upgrade().deref()).safe_ });
        field!(this, vec_).write({ (*_a0.upgrade().deref()).vec_ });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct type_ {
    #[offset(0)]
    pub v: i32,
}
impl type_ {
    pub fn new_1(this: Ptr<type_>) {}
    pub fn copy_from(this: Ptr<type_>, _a0: Ptr<type_>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<type_>, _a0: Ptr<type_>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct loop_ {
    #[offset(0)]
    #[byte_size(4)]
    pub t: type_,
}
impl loop_ {
    pub fn new_1(this: Ptr<loop_>) {
        type_::new_1(field_ptr!(this, t));
    }
    pub fn copy_from(this: Ptr<loop_>, _a0: Ptr<loop_>) {
        type_::copy_from(field_ptr!(this, t), { field_ptr!(_a0, t) });
    }
    pub fn move_from(this: Ptr<loop_>, _a0: Ptr<loop_>) {
        type_::move_from(field_ptr!(this, t), { field_ptr!(_a0, t) });
    }
}
pub fn foo_0(
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
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, as_).write(0);
        field!(__d, async_).write(0);
        field!(__d, await_).write(0);
        field!(__d, crate_).write(0);
        field!(__d, dyn_).write(0);
        field!(__d, fn_).write(0);
        field!(__d, impl_).write(0);
        field!(__d, in_).write(0);
        field!(__d, let_).write(0);
        field!(__d, loop_).write(0);
        field!(__d, match_).write(0);
        field!(__d, mod_).write(0);
        field!(__d, move_).write(0);
        field!(__d, mut_).write(0);
        field!(__d, pub_).write(0);
        field!(__d, ref_).write(0);
        field!(__d, self_).write(0);
        field!(__d, Self_).write(0);
        field!(__d, super_).write(0);
        field!(__d, trait_).write(0);
        field!(__d, type_).write(0);
        field!(__d, unsafe_).write(0);
        field!(__d, use_).write(0);
        field!(__d, where_).write(0);
        field!(__d, abstract_).write(0);
        field!(__d, become_).write(0);
        field!(__d, box_).write(0);
        field!(__d, final_).write(0);
        field!(__d, gen_).write(0);
        field!(__d, macro_).write(0);
        field!(__d, override_).write(0);
        field!(__d, priv_).write(0);
        field!(__d, unsized_).write(0);
        field!(__d, yield_).write(0);
        field!(__d, macro_rules_).write(0);
        field!(__d, raw_).write(0);
        field!(__d, safe_).write(0);
        field!(__d, vec_).write(0);
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
    let l: Value<loop_> = Rc::new(RefCell::new(<loop_>::uninit()));
    {
        let __d = l.as_pointer();
        {
            let __d = field_ptr!(__d, t);
            field!(__d, v).write(3);
        };
    };
    let t: Value<type_> = Rc::new(RefCell::new(<type_>::uninit()));
    type_::copy_from(t.as_pointer(), { field_ptr!(l.as_pointer(), t) });
    return ({
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
            ({ (*t.borrow()).v } - 3),
        )
    });
}
pub fn __cpp2rust_init_globals() {}
