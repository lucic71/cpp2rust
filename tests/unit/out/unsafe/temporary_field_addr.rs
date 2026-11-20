extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Inner {
    pub values: Vec<i32>,
}
impl Inner {
    pub unsafe fn copy_from(this: *mut Inner, _a0: *const Inner) {
        (&raw mut (*this).values).write((*_a0).values.clone());
    }
    pub unsafe fn move_from(this: *mut Inner, _a0: *mut Inner) {
        (&raw mut (*this).values).write(std::mem::take(&mut (*_a0).values));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Inner) -> *mut Inner {
        {
            self.values = std::mem::take(&mut (*_a0).values);
        };
        return &mut (*(self as *mut Inner));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Outer {
    pub values: Vec<i32>,
    pub inner: Inner,
    pub name: [libc::c_char; 8],
}
impl Outer {
    pub unsafe fn copy_from(this: *mut Outer, _a0: *const Outer) {
        (&raw mut (*this).values).write((*_a0).values.clone());
        Inner::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
        (&raw mut (*this).name).write(std::array::from_fn::<_, 8, _>(|__i: usize| {
            (*_a0).name[(__i)]
        }));
    }
    pub unsafe fn move_from(this: *mut Outer, _a0: *mut Outer) {
        (&raw mut (*this).values).write(std::mem::take(&mut (*_a0).values));
        Inner::move_from((&raw mut (*this).inner), { &mut (*_a0).inner });
        (&raw mut (*this).name).write(std::array::from_fn::<_, 8, _>(|__i: usize| {
            (*_a0).name[(__i)]
        }));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        {
            self.values = std::mem::take(&mut (*_a0).values);
        };
        (unsafe {
            let _arg0: *mut Inner = &mut (*_a0).inner;
            Inner::move_assign(&mut self.inner, _arg0)
        });
        {
            let mut __tmp_0 =
                { ((&mut self.name as *mut [libc::c_char; 8]) as *mut ::libc::c_void) };
            let mut __tmp_1 =
                { ((&mut (*_a0).name as *mut [libc::c_char; 8]) as *const ::libc::c_void) };
            if 8_usize != 0 {
                ::std::ptr::copy_nonoverlapping(__tmp_1, __tmp_0, 8_usize)
            }
            __tmp_0
        };
        return &mut (*(self as *mut Outer));
    }
}
pub unsafe fn make_0(__ret: *mut Outer, calls: *mut i32) {
    (*calls).prefix_inc();
    {
        let __d = __ret.clone();
        (&raw mut (*__d).values).write(Vec::from([1, 2, 3]));
        {
            let __d = (&raw mut (*__d).inner);
            (&raw mut (*__d).values).write(Vec::from([4, 5, 6]));
        };
        (&raw mut (*__d).name).write(std::mem::transmute(*b"abc\0\0\0\0\0"));
    };
    return;
}
pub unsafe fn sum_1(values: *const Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    'loop_: for value in IterRange::new((*values).begin(), (*values).end()) {
        let mut value: i32 = (*value.iter_deref());
        total += value;
    }
    return total;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut calls: i32 = 0;
    let mut member: i32 = (unsafe {
        sum_1(
            &{
                let mut __v: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
                (unsafe { make_0(&raw mut __v, &mut calls) });
                __v
            }
            .values,
        )
    });
    let mut nested: i32 = (unsafe {
        sum_1(
            &{
                let mut __v: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
                (unsafe { make_0(&raw mut __v, &mut calls) });
                __v
            }
            .inner
            .values,
        )
    });
    let mut element: i32 = {
        let mut __v: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { make_0(&raw mut __v, &mut calls) });
        __v
    }
    .values[(((calls) - (2)) as usize)];
    let mut length: usize = libc::strlen(
        ({
            let mut __v: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { make_0(&raw mut __v, &mut calls) });
            __v
        }
        .name
        .as_mut_ptr())
        .cast_const(),
    );
    let mut aggregate: i32 = (unsafe {
        sum_1(
            &Outer {
                values: Vec::from([7]),
                inner: Inner {
                    values: Vec::from([8, 9]),
                },
                name: std::mem::transmute(*b"x\0\0\0\0\0\0\0"),
            }
            .inner
            .values,
        )
    });
    (unsafe {
        libc::printf(
            c"%d %d %d %zu %d %d\n".as_ptr() as *const libc::c_char,
            (member),
            (nested),
            (element),
            (length),
            (aggregate),
            (calls),
        )
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
