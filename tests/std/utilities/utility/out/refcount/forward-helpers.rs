extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let value: Value<i32> = Rc::new(RefCell::new(1));
    let mut forwarded: i32 = (*value.borrow());
    assert!((forwarded == 1));
    let lvalue: Ptr<i32> = value.as_pointer();
    lvalue.write(2);
    assert!(({ ((*value.borrow()) == 2) }));
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p6_forward_like_Movable {
    #[offset(0)]
    pub value: i32,
}
impl p6_forward_like_Movable {
    pub fn new(this: Ptr<p6_forward_like_Movable>, mut value: i32) {
        field!(this, value).write(value);
    }
    pub fn copy_from(this: Ptr<p6_forward_like_Movable>, other: Ptr<p6_forward_like_Movable>) {
        field!(this, value).write(other.with(|__s| __s.value));
    }
    pub fn move_from(this: Ptr<p6_forward_like_Movable>, other: Ptr<p6_forward_like_Movable>) {
        field!(this, value).write(other.with(|__s| __s.value));
        field!(other, value).write(0);
    }
}
impl Clone for p6_forward_like_Movable {
    fn clone(&self) -> Self {
        let __src: Value<p6_forward_like_Movable> =
            Rc::new(RefCell::new(p6_forward_like_Movable {
                value: self.value.clone(),
            }));
        let __dst: Value<p6_forward_like_Movable> =
            Rc::new(RefCell::new(<p6_forward_like_Movable>::uninit()));
        p6_forward_like_Movable::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(4)]
pub struct p6_forward_like_Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub member: p6_forward_like_Movable,
}
impl p6_forward_like_Holder {
    pub fn copy_from(this: Ptr<p6_forward_like_Holder>, _a0: Ptr<p6_forward_like_Holder>) {
        p6_forward_like_Movable::copy_from(field_ptr!(this, member), { field_ptr!(_a0, member) });
    }
    pub fn move_from(this: Ptr<p6_forward_like_Holder>, _a0: Ptr<p6_forward_like_Holder>) {
        p6_forward_like_Movable::move_from(field_ptr!(this, member), { field_ptr!(_a0, member) });
    }
    pub fn new_1(this: Ptr<p6_forward_like_Holder>) {
        p6_forward_like_Movable::new(field_ptr!(this, member), { 1 });
    }
}
impl Default for p6_forward_like_Holder {
    fn default() -> Self {
        {
            p6_forward_like_Holder {
                member: Rc::try_unwrap({
                    let __v: Value<p6_forward_like_Movable> =
                        Rc::new(RefCell::new(<p6_forward_like_Movable>::uninit()));
                    p6_forward_like_Movable::new(__v.as_pointer(), { 1 });
                    __v
                })
                .ok()
                .unwrap()
                .into_inner(),
            }
        }
    }
}
pub fn take_1(__ret: Ptr<p6_forward_like_Movable>, self_: Ptr<p6_forward_like_Holder>) {
    p6_forward_like_Movable::copy_from(__ret.clone(), { field_ptr!(self_, member) });
    return;
}
pub fn take_2(__ret: Ptr<p6_forward_like_Movable>, self_: Ptr<p6_forward_like_Holder>) {
    p6_forward_like_Movable::move_from(__ret.clone(), { field_ptr!(self_, member) });
    return;
}
pub fn test_3() {
    let holder: Value<p6_forward_like_Holder> =
        Rc::new(RefCell::new(<p6_forward_like_Holder>::uninit()));
    p6_forward_like_Holder::new_1(holder.as_pointer());
    let copied: Value<p6_forward_like_Movable> =
        Rc::new(RefCell::new(<p6_forward_like_Movable>::uninit()));
    ({ take_1(copied.as_pointer(), holder.as_pointer()) });
    assert!(
        ({ ({ (*copied.borrow()).value } == 1) && ({ (*holder.borrow()).member.value } == 1) })
    );
    let moved: Value<p6_forward_like_Movable> =
        Rc::new(RefCell::new(<p6_forward_like_Movable>::uninit()));
    ({ take_2(moved.as_pointer(), holder.as_pointer()) });
    assert!(({ ({ (*moved.borrow()).value } == 1) && ({ (*holder.borrow()).member.value } == 0) }));
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p9_move_Movable {
    #[offset(0)]
    pub value: i32,
}
impl p9_move_Movable {
    pub fn new(this: Ptr<p9_move_Movable>, mut value: i32) {
        field!(this, value).write(value);
    }
    pub fn move_from(this: Ptr<p9_move_Movable>, other: Ptr<p9_move_Movable>) {
        field!(this, value).write(other.with(|__s| __s.value));
        field!(other, value).write(0);
    }
}
pub fn test_4() {
    let mut number: i32 = 1;
    let mut moved: i32 = number;
    assert!((moved == 1) && (number == 1));
    let target: Value<i32> = Rc::new(RefCell::new(2));
    let mut pointer: Ptr<i32> = (target.as_pointer());
    let mut moved_pointer: Ptr<i32> = (pointer).clone();
    assert!(
        ({ (moved_pointer).clone() } == { (target.as_pointer()) })
            && ({ (pointer).clone() } == { (target.as_pointer()) })
    );
    let source: Value<p9_move_Movable> = Rc::new(RefCell::new(<p9_move_Movable>::uninit()));
    p9_move_Movable::new(source.as_pointer(), { 3 });
    let destination: Value<p9_move_Movable> = Rc::new(RefCell::new(<p9_move_Movable>::uninit()));
    p9_move_Movable::move_from(destination.as_pointer(), { source.as_pointer() });
    assert!(({ ({ (*destination.borrow()).value } == 3) && ({ (*source.borrow()).value } == 0) }));
    let strings: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new(Vec::new()));
    let mut text: Vec<i8> = {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"text").to_c_bytes();
        __bytes.push(0);
        __bytes
    };
    {
        let __a1 = std::mem::replace(&mut text, vec![0]);
        (*strings.borrow_mut()).push(__a1)
    };
    assert!(
        ({
            match (strings.decay().offset(0_usize),) {
                (__tmp_0,) => Ptr::<i8>::from_string_literal(b"text").with_c_str(|__s| {
                    (*__tmp_0.upgrade().deref())
                        [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }
        })
    );
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom)]
#[cpp_from(i32, new, value)]
#[byte_size(12)]
pub struct p10_forward_in_template_A {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl p10_forward_in_template_A {
    pub fn new(this: Ptr<p10_forward_in_template_A>, mut value: i32) {
        field!(this, value).write(value);
        field!(this, copies).write(0);
        field!(this, moves).write(0);
    }
    pub fn copy_from(this: Ptr<p10_forward_in_template_A>, other: Ptr<p10_forward_in_template_A>) {
        field!(this, value).write(other.with(|__s| __s.value));
        field!(this, copies).write((other.with(|__s| __s.copies) + 1));
        field!(this, moves).write(0);
    }
    pub fn move_from(this: Ptr<p10_forward_in_template_A>, other: Ptr<p10_forward_in_template_A>) {
        field!(this, value).write(other.with(|__s| __s.value));
        field!(this, copies).write(0);
        field!(this, moves).write((other.with(|__s| __s.moves) + 1));
        field!(other, value).write(0);
    }
}
impl Clone for p10_forward_in_template_A {
    fn clone(&self) -> Self {
        let __src: Value<p10_forward_in_template_A> =
            Rc::new(RefCell::new(p10_forward_in_template_A {
                value: self.value.clone(),
                copies: self.copies.clone(),
                moves: self.moves.clone(),
            }));
        let __dst: Value<p10_forward_in_template_A> =
            Rc::new(RefCell::new(<p10_forward_in_template_A>::uninit()));
        p10_forward_in_template_A::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl Default for p10_forward_in_template_A {
    fn default() -> Self {
        {
            p10_forward_in_template_A {
                value: 0_i32,
                copies: 0,
                moves: 0,
            }
        }
    }
}
pub fn make_5(__ret: Ptr<p10_forward_in_template_A>, source: Ptr<p10_forward_in_template_A>) {
    p10_forward_in_template_A::copy_from(__ret.clone(), { (source).clone() });
    return;
}
pub fn make_6(__ret: Ptr<p10_forward_in_template_A>, source: Ptr<p10_forward_in_template_A>) {
    p10_forward_in_template_A::move_from(__ret.clone(), { (source).clone() });
    return;
}
pub fn test_7() {
    let a: Value<p10_forward_in_template_A> =
        Rc::new(RefCell::new(<p10_forward_in_template_A>::uninit()));
    p10_forward_in_template_A::new(a.as_pointer(), { 1 });
    let copied: Value<p10_forward_in_template_A> =
        Rc::new(RefCell::new(<p10_forward_in_template_A>::uninit()));
    ({ make_5(copied.as_pointer(), a.as_pointer()) });
    assert!(
        ({
            (({ (*copied.borrow()).value } == 1) && ({ (*copied.borrow()).copies } == 1))
                && ({ (*copied.borrow()).moves } == 0)
        })
    );
    assert!(({ ({ (*a.borrow()).value } == 1) }));
    let moved: Value<p10_forward_in_template_A> =
        Rc::new(RefCell::new(<p10_forward_in_template_A>::uninit()));
    ({ make_6(moved.as_pointer(), a.as_pointer()) });
    assert!(
        ({
            (({ (*moved.borrow()).value } == 1) && ({ (*moved.borrow()).copies } == 0))
                && ({ (*moved.borrow()).moves } == 1)
        })
    );
    assert!(({ ({ (*a.borrow()).value } == 0) }));
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p11_move_if_noexcept_NoexceptMove {
    #[offset(0)]
    pub value: i32,
}
impl p11_move_if_noexcept_NoexceptMove {
    pub fn new(this: Ptr<p11_move_if_noexcept_NoexceptMove>, mut value: i32) {
        field!(this, value).write(value);
    }
    pub fn copy_from(
        this: Ptr<p11_move_if_noexcept_NoexceptMove>,
        other: Ptr<p11_move_if_noexcept_NoexceptMove>,
    ) {
        field!(this, value).write(other.with(|__s| __s.value));
    }
    pub fn move_from(
        this: Ptr<p11_move_if_noexcept_NoexceptMove>,
        other: Ptr<p11_move_if_noexcept_NoexceptMove>,
    ) {
        field!(this, value).write(other.with(|__s| __s.value));
        field!(other, value).write(0);
    }
}
impl Clone for p11_move_if_noexcept_NoexceptMove {
    fn clone(&self) -> Self {
        let __src: Value<p11_move_if_noexcept_NoexceptMove> =
            Rc::new(RefCell::new(p11_move_if_noexcept_NoexceptMove {
                value: self.value.clone(),
            }));
        let __dst: Value<p11_move_if_noexcept_NoexceptMove> =
            Rc::new(RefCell::new(<p11_move_if_noexcept_NoexceptMove>::uninit()));
        p11_move_if_noexcept_NoexceptMove::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p11_move_if_noexcept_ThrowingMove {
    #[offset(0)]
    pub value: i32,
}
impl p11_move_if_noexcept_ThrowingMove {
    pub fn new(this: Ptr<p11_move_if_noexcept_ThrowingMove>, mut value: i32) {
        field!(this, value).write(value);
    }
    pub fn copy_from(
        this: Ptr<p11_move_if_noexcept_ThrowingMove>,
        other: Ptr<p11_move_if_noexcept_ThrowingMove>,
    ) {
        field!(this, value).write(other.with(|__s| __s.value));
    }
    pub fn move_from(
        this: Ptr<p11_move_if_noexcept_ThrowingMove>,
        other: Ptr<p11_move_if_noexcept_ThrowingMove>,
    ) {
        field!(this, value).write(other.with(|__s| __s.value));
        field!(other, value).write(0);
    }
}
impl Clone for p11_move_if_noexcept_ThrowingMove {
    fn clone(&self) -> Self {
        let __src: Value<p11_move_if_noexcept_ThrowingMove> =
            Rc::new(RefCell::new(p11_move_if_noexcept_ThrowingMove {
                value: self.value.clone(),
            }));
        let __dst: Value<p11_move_if_noexcept_ThrowingMove> =
            Rc::new(RefCell::new(<p11_move_if_noexcept_ThrowingMove>::uninit()));
        p11_move_if_noexcept_ThrowingMove::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
pub fn test_8() {
    let moved_from: Value<p11_move_if_noexcept_NoexceptMove> =
        Rc::new(RefCell::new(<p11_move_if_noexcept_NoexceptMove>::uninit()));
    p11_move_if_noexcept_NoexceptMove::new(moved_from.as_pointer(), { 1 });
    let moved: Value<p11_move_if_noexcept_NoexceptMove> =
        Rc::new(RefCell::new(<p11_move_if_noexcept_NoexceptMove>::uninit()));
    p11_move_if_noexcept_NoexceptMove::move_from(moved.as_pointer(), { moved_from.as_pointer() });
    assert!(({ ({ (*moved.borrow()).value } == 1) && ({ (*moved_from.borrow()).value } == 0) }));
    let copied_from: Value<p11_move_if_noexcept_ThrowingMove> =
        Rc::new(RefCell::new(<p11_move_if_noexcept_ThrowingMove>::uninit()));
    p11_move_if_noexcept_ThrowingMove::new(copied_from.as_pointer(), { 2 });
    let copied: Value<p11_move_if_noexcept_ThrowingMove> =
        Rc::new(RefCell::new(<p11_move_if_noexcept_ThrowingMove>::uninit()));
    p11_move_if_noexcept_ThrowingMove::copy_from(copied.as_pointer(), { copied_from.as_pointer() });
    assert!(({ ({ (*copied.borrow()).value } == 2) && ({ (*copied_from.borrow()).value } == 2) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_3() });
    ({ test_4() });
    ({ test_7() });
    ({ test_8() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
