extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut value: i32 = 1;
    let mut forwarded: i32 = value;
    assert!(((forwarded) == (1)));
    let lvalue: *mut i32 = &mut value;
    (*lvalue) = 2;
    assert!(((value) == (2)));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct p6_forward_like_Movable {
    pub value: i32,
}
impl p6_forward_like_Movable {
    pub unsafe fn new(this: *mut p6_forward_like_Movable, mut value: i32) {
        (&raw mut (*this).value).write(value);
    }
    pub unsafe fn copy_from(
        this: *mut p6_forward_like_Movable,
        other: *const p6_forward_like_Movable,
    ) {
        (&raw mut (*this).value).write((*other).value);
    }
    pub unsafe fn move_from(
        this: *mut p6_forward_like_Movable,
        other: *mut p6_forward_like_Movable,
    ) {
        (&raw mut (*this).value).write((*other).value);
        (*other).value = 0;
    }
}
impl Clone for p6_forward_like_Movable {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: p6_forward_like_Movable = ::std::mem::MaybeUninit::uninit().assume_init();
            p6_forward_like_Movable::copy_from(
                &raw mut __v,
                self as *const p6_forward_like_Movable,
            );
            __v
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
#[placement]
pub struct p6_forward_like_Holder {
    pub member: p6_forward_like_Movable,
}
impl p6_forward_like_Holder {
    pub unsafe fn copy_from(this: *mut p6_forward_like_Holder, _a0: *const p6_forward_like_Holder) {
        p6_forward_like_Movable::copy_from((&raw mut (*this).member), { &(*_a0).member });
    }
    pub unsafe fn move_from(this: *mut p6_forward_like_Holder, _a0: *mut p6_forward_like_Holder) {
        p6_forward_like_Movable::move_from((&raw mut (*this).member), { &mut (*_a0).member });
    }
    pub unsafe fn new_1(this: *mut p6_forward_like_Holder) {
        p6_forward_like_Movable::new((&raw mut (*this).member), { 1 });
    }
}
impl Default for p6_forward_like_Holder {
    fn default() -> Self {
        unsafe {
            p6_forward_like_Holder {
                member: {
                    let mut __v: p6_forward_like_Movable =
                        ::std::mem::MaybeUninit::uninit().assume_init();
                    p6_forward_like_Movable::new(&raw mut __v, { 1 });
                    __v
                },
            }
        }
    }
}
pub unsafe fn take_1(__ret: *mut p6_forward_like_Movable, self_: *mut p6_forward_like_Holder) {
    p6_forward_like_Movable::copy_from(__ret.clone(), { &(*self_).member });
    return;
}
pub unsafe fn take_2(__ret: *mut p6_forward_like_Movable, self_: *mut p6_forward_like_Holder) {
    p6_forward_like_Movable::move_from(__ret.clone(), { &mut (*self_).member });
    return;
}
pub unsafe fn test_3() {
    let mut holder: p6_forward_like_Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    p6_forward_like_Holder::new_1(&raw mut holder);
    let mut copied: p6_forward_like_Movable = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { take_1(&raw mut copied, &mut holder) });
    assert!(((copied.value) == (1)) && ((holder.member.value) == (1)));
    let mut moved: p6_forward_like_Movable = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { take_2(&raw mut moved, &mut holder) });
    assert!(((moved.value) == (1)) && ((holder.member.value) == (0)));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct p9_move_Movable {
    pub value: i32,
}
impl p9_move_Movable {
    pub unsafe fn new(this: *mut p9_move_Movable, mut value: i32) {
        (&raw mut (*this).value).write(value);
    }
    pub unsafe fn move_from(this: *mut p9_move_Movable, other: *mut p9_move_Movable) {
        (&raw mut (*this).value).write((*other).value);
        (*other).value = 0;
    }
}
pub unsafe fn test_4() {
    let mut number: i32 = 1;
    let mut moved: i32 = number;
    assert!(((moved) == (1)) && ((number) == (1)));
    let mut target: i32 = 2;
    let mut pointer: *mut i32 = (&mut target as *mut i32);
    let mut moved_pointer: *mut i32 = pointer;
    assert!(
        ((moved_pointer) == (&mut target as *mut i32)) && ((pointer) == (&mut target as *mut i32))
    );
    let mut source: p9_move_Movable = ::std::mem::MaybeUninit::uninit().assume_init();
    p9_move_Movable::new(&raw mut source, { 3 });
    let mut destination: p9_move_Movable = ::std::mem::MaybeUninit::uninit().assume_init();
    p9_move_Movable::move_from(&raw mut destination, { &mut source });
    assert!(((destination.value) == (3)) && ((source.value) == (0)));
    let mut strings: Vec<Vec<libc::c_char>> = Vec::new();
    let mut text: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"text".as_ptr(),
        (0..).take_while(|&i| *c"text".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec();
    {
        let __a1 = std::mem::replace(&mut text, vec![0]);
        strings.push(__a1)
    };
    assert!(
        strings[(0_usize)] == {
            std::slice::from_raw_parts(
                c"text".as_ptr(),
                (0..).take_while(|&i| *c"text".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct p10_forward_in_template_A {
    pub value: i32,
    pub copies: i32,
    pub moves: i32,
}
impl p10_forward_in_template_A {
    pub unsafe fn new(this: *mut p10_forward_in_template_A, mut value: i32) {
        (&raw mut (*this).value).write(value);
        (&raw mut (*this).copies).write(0);
        (&raw mut (*this).moves).write(0);
    }
    pub unsafe fn copy_from(
        this: *mut p10_forward_in_template_A,
        other: *const p10_forward_in_template_A,
    ) {
        (&raw mut (*this).value).write((*other).value);
        (&raw mut (*this).copies).write((((*other).copies) + (1)));
        (&raw mut (*this).moves).write(0);
    }
    pub unsafe fn move_from(
        this: *mut p10_forward_in_template_A,
        other: *mut p10_forward_in_template_A,
    ) {
        (&raw mut (*this).value).write((*other).value);
        (&raw mut (*this).copies).write(0);
        (&raw mut (*this).moves).write((((*other).moves) + (1)));
        (*other).value = 0;
    }
}
impl Clone for p10_forward_in_template_A {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: p10_forward_in_template_A =
                ::std::mem::MaybeUninit::uninit().assume_init();
            p10_forward_in_template_A::copy_from(
                &raw mut __v,
                self as *const p10_forward_in_template_A,
            );
            __v
        }
    }
}
impl Default for p10_forward_in_template_A {
    fn default() -> Self {
        unsafe {
            p10_forward_in_template_A {
                value: 0_i32,
                copies: 0,
                moves: 0,
            }
        }
    }
}
pub unsafe fn make_5(
    __ret: *mut p10_forward_in_template_A,
    source: *mut p10_forward_in_template_A,
) {
    p10_forward_in_template_A::copy_from(__ret.clone(), { source });
    return;
}
pub unsafe fn make_6(
    __ret: *mut p10_forward_in_template_A,
    source: *mut p10_forward_in_template_A,
) {
    p10_forward_in_template_A::move_from(__ret.clone(), { source });
    return;
}
pub unsafe fn test_7() {
    let mut a: p10_forward_in_template_A = ::std::mem::MaybeUninit::uninit().assume_init();
    p10_forward_in_template_A::new(&raw mut a, { 1 });
    let mut copied: p10_forward_in_template_A = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { make_5(&raw mut copied, &mut a) });
    assert!((((copied.value) == (1)) && ((copied.copies) == (1))) && ((copied.moves) == (0)));
    assert!(((a.value) == (1)));
    let mut moved: p10_forward_in_template_A = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { make_6(&raw mut moved, &mut a) });
    assert!((((moved.value) == (1)) && ((moved.copies) == (0))) && ((moved.moves) == (1)));
    assert!(((a.value) == (0)));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct p11_move_if_noexcept_NoexceptMove {
    pub value: i32,
}
impl p11_move_if_noexcept_NoexceptMove {
    pub unsafe fn new(this: *mut p11_move_if_noexcept_NoexceptMove, mut value: i32) {
        (&raw mut (*this).value).write(value);
    }
    pub unsafe fn copy_from(
        this: *mut p11_move_if_noexcept_NoexceptMove,
        other: *const p11_move_if_noexcept_NoexceptMove,
    ) {
        (&raw mut (*this).value).write((*other).value);
    }
    pub unsafe fn move_from(
        this: *mut p11_move_if_noexcept_NoexceptMove,
        other: *mut p11_move_if_noexcept_NoexceptMove,
    ) {
        (&raw mut (*this).value).write((*other).value);
        (*other).value = 0;
    }
}
impl Clone for p11_move_if_noexcept_NoexceptMove {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: p11_move_if_noexcept_NoexceptMove =
                ::std::mem::MaybeUninit::uninit().assume_init();
            p11_move_if_noexcept_NoexceptMove::copy_from(
                &raw mut __v,
                self as *const p11_move_if_noexcept_NoexceptMove,
            );
            __v
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct p11_move_if_noexcept_ThrowingMove {
    pub value: i32,
}
impl p11_move_if_noexcept_ThrowingMove {
    pub unsafe fn new(this: *mut p11_move_if_noexcept_ThrowingMove, mut value: i32) {
        (&raw mut (*this).value).write(value);
    }
    pub unsafe fn copy_from(
        this: *mut p11_move_if_noexcept_ThrowingMove,
        other: *const p11_move_if_noexcept_ThrowingMove,
    ) {
        (&raw mut (*this).value).write((*other).value);
    }
    pub unsafe fn move_from(
        this: *mut p11_move_if_noexcept_ThrowingMove,
        other: *mut p11_move_if_noexcept_ThrowingMove,
    ) {
        (&raw mut (*this).value).write((*other).value);
        (*other).value = 0;
    }
}
impl Clone for p11_move_if_noexcept_ThrowingMove {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: p11_move_if_noexcept_ThrowingMove =
                ::std::mem::MaybeUninit::uninit().assume_init();
            p11_move_if_noexcept_ThrowingMove::copy_from(
                &raw mut __v,
                self as *const p11_move_if_noexcept_ThrowingMove,
            );
            __v
        }
    }
}
pub unsafe fn test_8() {
    let mut moved_from: p11_move_if_noexcept_NoexceptMove =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p11_move_if_noexcept_NoexceptMove::new(&raw mut moved_from, { 1 });
    let mut moved: p11_move_if_noexcept_NoexceptMove =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p11_move_if_noexcept_NoexceptMove::move_from(&raw mut moved, { &mut moved_from });
    assert!(((moved.value) == (1)) && ((moved_from.value) == (0)));
    let mut copied_from: p11_move_if_noexcept_ThrowingMove =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p11_move_if_noexcept_ThrowingMove::new(&raw mut copied_from, { 2 });
    let mut copied: p11_move_if_noexcept_ThrowingMove =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p11_move_if_noexcept_ThrowingMove::copy_from(&raw mut copied, { &copied_from });
    assert!(((copied.value) == (2)) && ((copied_from.value) == (2)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
