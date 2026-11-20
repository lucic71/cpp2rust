extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p11_defaulted_move_constructor_is_trivial_MoveOnly {
    pub value: i32,
}
impl p11_defaulted_move_constructor_is_trivial_MoveOnly {
    pub unsafe fn new(
        this: *mut p11_defaulted_move_constructor_is_trivial_MoveOnly,
        mut value: i32,
    ) {
        (&raw mut (*this).value).write(value);
    }
    pub unsafe fn move_from(
        this: *mut p11_defaulted_move_constructor_is_trivial_MoveOnly,
        _a0: *mut p11_defaulted_move_constructor_is_trivial_MoveOnly,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub unsafe fn pass_0(
    __ret: *mut p11_defaulted_move_constructor_is_trivial_MoveOnly,
    object: *mut p11_defaulted_move_constructor_is_trivial_MoveOnly,
) {
    p11_defaulted_move_constructor_is_trivial_MoveOnly::move_from(__ret.clone(), {
        &mut (*object)
    });
    return;
}
pub unsafe fn test_1() {
    let mut result: p11_defaulted_move_constructor_is_trivial_MoveOnly =
        ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe {
        pass_0 ( &raw mut result , (|__p: *mut p11_defaulted_move_constructor_is_trivial_MoveOnly | { p11_defaulted_move_constructor_is_trivial_MoveOnly :: new ( __p , { 7 } , )  ;
 __p } )((&mut ::std::mem::MaybeUninit::<p11_defaulted_move_constructor_is_trivial_MoveOnly >::uninit()).as_mut_ptr())  , )
    });
    assert!(((result.value) == (7)));
}
pub unsafe fn read_by_value_2(mut it: UnsafeRandomAccessIterator<i32>) -> i32 {
    return (*it.iter_deref());
}
pub unsafe fn test_3() {
    let mut source: Vec<i32> = Vec::from([1, 2, 3]);
    let mut moved: Vec<i32> = source.clone();
    assert!(((moved.len()) == (3_usize)));
    assert!(((source.len()) == (3_usize)));
    let mut values: Vec<i32> = Vec::from([7, 8]);
    let mut it: UnsafeRandomAccessIterator<i32> = values.begin();
    assert!(((unsafe { read_by_value_2(it.clone(),) }) == (7)));
    assert!(((*it.iter_deref()) == (7)));
    let mut moved_it: UnsafeRandomAccessIterator<i32> = it.clone();
    assert!(((*moved_it.iter_deref()) == (7)));
    assert!(((*it.iter_deref()) == (7)));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p14_memberwise_copy_and_move_in_declaration_order_Logged {
    pub id: i32,
}
impl p14_memberwise_copy_and_move_in_declaration_order_Logged {
    pub unsafe fn new(
        this: *mut p14_memberwise_copy_and_move_in_declaration_order_Logged,
        mut id: i32,
    ) {
        (&raw mut (*this).id).write(id);
    }
    pub unsafe fn copy_from(
        this: *mut p14_memberwise_copy_and_move_in_declaration_order_Logged,
        other: *const p14_memberwise_copy_and_move_in_declaration_order_Logged,
    ) {
        (&raw mut (*this).id).write((*other).id);
        (unsafe { libc::printf(c"copy %d\n".as_ptr() as *const libc::c_char, ((*this).id)) });
    }
    pub unsafe fn move_from(
        this: *mut p14_memberwise_copy_and_move_in_declaration_order_Logged,
        other: *mut p14_memberwise_copy_and_move_in_declaration_order_Logged,
    ) {
        (&raw mut (*this).id).write((*other).id);
        (unsafe { libc::printf(c"move %d\n".as_ptr() as *const libc::c_char, ((*this).id)) });
        (*other).id = -1_i32;
    }
}
impl Clone for p14_memberwise_copy_and_move_in_declaration_order_Logged {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: p14_memberwise_copy_and_move_in_declaration_order_Logged =
                ::std::mem::MaybeUninit::uninit().assume_init();
            p14_memberwise_copy_and_move_in_declaration_order_Logged::copy_from(
                &raw mut __v,
                self as *const p14_memberwise_copy_and_move_in_declaration_order_Logged,
            );
            __v
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct p14_memberwise_copy_and_move_in_declaration_order_Holder {
    pub first: p14_memberwise_copy_and_move_in_declaration_order_Logged,
    pub number: i32,
    pub elements: [p14_memberwise_copy_and_move_in_declaration_order_Logged; 2],
    pub last: p14_memberwise_copy_and_move_in_declaration_order_Logged,
}
impl p14_memberwise_copy_and_move_in_declaration_order_Holder {
    pub unsafe fn copy_from(
        this: *mut p14_memberwise_copy_and_move_in_declaration_order_Holder,
        _a0: *const p14_memberwise_copy_and_move_in_declaration_order_Holder,
    ) {
        p14_memberwise_copy_and_move_in_declaration_order_Logged::copy_from(
            (&raw mut (*this).first),
            { &(*_a0).first },
        );
        (&raw mut (*this).number).write((*_a0).number);
        {
            let __d = (&raw mut (*this).elements)
                .cast::<p14_memberwise_copy_and_move_in_declaration_order_Logged>();
            for __i in 0..2_usize {
                p14_memberwise_copy_and_move_in_declaration_order_Logged::copy_from(
                    __d.offset(__i as isize),
                    { &(*_a0).elements[(__i)] },
                );
            }
        };
        p14_memberwise_copy_and_move_in_declaration_order_Logged::copy_from(
            (&raw mut (*this).last),
            { &(*_a0).last },
        );
    }
    pub unsafe fn move_from(
        this: *mut p14_memberwise_copy_and_move_in_declaration_order_Holder,
        _a0: *mut p14_memberwise_copy_and_move_in_declaration_order_Holder,
    ) {
        p14_memberwise_copy_and_move_in_declaration_order_Logged::move_from(
            (&raw mut (*this).first),
            { &mut (*_a0).first },
        );
        (&raw mut (*this).number).write((*_a0).number);
        {
            let __d = (&raw mut (*this).elements)
                .cast::<p14_memberwise_copy_and_move_in_declaration_order_Logged>();
            for __i in 0..2_usize {
                p14_memberwise_copy_and_move_in_declaration_order_Logged::move_from(
                    __d.offset(__i as isize),
                    { &mut (*_a0).elements[(__i)] },
                );
            }
        };
        p14_memberwise_copy_and_move_in_declaration_order_Logged::move_from(
            (&raw mut (*this).last),
            { &mut (*_a0).last },
        );
    }
}
pub unsafe fn test_4() {
    let mut source: p14_memberwise_copy_and_move_in_declaration_order_Holder =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut source;
        p14_memberwise_copy_and_move_in_declaration_order_Logged::new((&raw mut (*__d).first), {
            1
        });
        (&raw mut (*__d).number).write(5);
        {
            let __d = (&raw mut (*__d).elements)
                .cast::<p14_memberwise_copy_and_move_in_declaration_order_Logged>();
            p14_memberwise_copy_and_move_in_declaration_order_Logged::new(__d.offset(0), { 2 });
            p14_memberwise_copy_and_move_in_declaration_order_Logged::new(__d.offset(1), { 3 });
        };
        p14_memberwise_copy_and_move_in_declaration_order_Logged::new((&raw mut (*__d).last), {
            4
        });
    };
    let mut copy: p14_memberwise_copy_and_move_in_declaration_order_Holder =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p14_memberwise_copy_and_move_in_declaration_order_Holder::copy_from(&raw mut copy, { &source });
    assert!((((copy.first.id) == (1)) && ((copy.number) == (5))) && ((copy.last.id) == (4)));
    assert!(((copy.elements[(0) as usize].id) == (2)) && ((copy.elements[(1) as usize].id) == (3)));
    assert!(((source.first.id) == (1)) && ((source.last.id) == (4)));
    let mut moved: p14_memberwise_copy_and_move_in_declaration_order_Holder =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p14_memberwise_copy_and_move_in_declaration_order_Holder::move_from(&raw mut moved, {
        &mut source
    });
    assert!((((moved.first.id) == (1)) && ((moved.number) == (5))) && ((moved.last.id) == (4)));
    assert!(
        ((moved.elements[(0) as usize].id) == (2)) && ((moved.elements[(1) as usize].id) == (3))
    );
    assert!(((source.first.id) == (-1_i32)) && ((source.elements[(1) as usize].id) == (-1_i32)));
    assert!(((source.last.id) == (-1_i32)) && ((source.number) == (5)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_4() });
    (unsafe { test_1() });
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
