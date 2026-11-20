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
pub struct Counter {
    pub n: i32,
}
impl Counter {
    pub unsafe fn get(&self) -> i32 {
        return self.n;
    }
    pub unsafe fn add(&mut self, mut k: i32) {
        self.n += k;
    }
    pub unsafe fn self_(&mut self) -> *mut Counter {
        return (self as *mut Counter);
    }
    pub unsafe fn take(&mut self, mut other: *mut Counter) {
        self.n += (*other).n;
        (*other).n = 0;
    }
    pub unsafe fn copy_from(this: *mut Counter, _a0: *const Counter) {
        (&raw mut (*this).n).write((*_a0).n);
    }
    pub unsafe fn move_from(this: *mut Counter, _a0: *mut Counter) {
        (&raw mut (*this).n).write((*_a0).n);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct S {
    pub tag: i32,
    pub c: Counter,
    pub arr: [Counter; 2],
    pub v: Vec<i32>,
}
impl S {
    pub unsafe fn bump(&mut self) {
        (unsafe {
            let _k: i32 = self.tag;
            Counter::add(&mut self.c, _k)
        });
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).tag).write((*_a0).tag);
        Counter::copy_from((&raw mut (*this).c), { &(*_a0).c });
        {
            let __d = (&raw mut (*this).arr).cast::<Counter>();
            for __i in 0..2_usize {
                Counter::copy_from(__d.offset(__i as isize), { &(*_a0).arr[(__i)] });
            }
        };
        (&raw mut (*this).v).write((*_a0).v.clone());
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).tag).write((*_a0).tag);
        Counter::move_from((&raw mut (*this).c), { &mut (*_a0).c });
        {
            let __d = (&raw mut (*this).arr).cast::<Counter>();
            for __i in 0..2_usize {
                Counter::move_from(__d.offset(__i as isize), { &mut (*_a0).arr[(__i)] });
            }
        };
        (&raw mut (*this).v).write(std::mem::take(&mut (*_a0).v));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        self.tag = (*_a0).tag;
        self.c = (*_a0).c;
        {
            let mut __tmp_0 = { ((&mut self.arr as *mut [Counter; 2]) as *mut ::libc::c_void) };
            let mut __tmp_1 = { ((&mut (*_a0).arr as *mut [Counter; 2]) as *const ::libc::c_void) };
            if 8_usize != 0 {
                ::std::ptr::copy_nonoverlapping(__tmp_1, __tmp_0, 8_usize)
            }
            __tmp_0
        };
        {
            self.v = std::mem::take(&mut (*_a0).v);
        };
        return &mut (*(self as *mut S));
    }
}
pub unsafe fn run_0(mut o: *mut S) {
    (unsafe { Counter::add(&mut (*o).c, 2) });
    assert!(((unsafe { Counter::get(&(*o).c,) }) == (2)));
    (unsafe { Counter::add(&mut (*o).arr[(1) as usize], 5) });
    assert!(((unsafe { Counter::get(&(*o).arr[(1) as usize],) }) == (5)));
    (unsafe { Counter::add(&mut (*(unsafe { Counter::self_(&mut (*o).c) })), 1) });
    assert!(((unsafe { Counter::get(&(*o).c,) }) == (3)));
    assert!(((unsafe { Counter::self_(&mut (*o).c,) }) == (&mut (*o).c as *mut Counter)));
    (unsafe {
        let _other: *mut Counter = (&mut (*o).arr[(1) as usize] as *mut Counter);
        Counter::take(&mut (*o).arr[(0) as usize], _other)
    });
    assert!(
        ((unsafe { Counter::get(&(*o).arr[(0) as usize],) }) == (5))
            && ((unsafe { Counter::get(&(*o).arr[(1) as usize],) }) == (0))
    );
    (unsafe {
        let _other: *mut Counter = (&mut (*o).c as *mut Counter);
        Counter::take(&mut (*o).c, _other)
    });
    assert!(((unsafe { Counter::get(&(*o).c,) }) == (0)));
    (unsafe { S::bump(&mut (*o)) });
    assert!(((unsafe { Counter::get(&(*o).c,) }) == (1)));
    {
        let __a1 = (unsafe { Counter::get(&(*o).c) });
        (*o).v.push(__a1)
    };
    assert!((((*o).v.len()) == (1_usize)) && (((&mut (*o)).v[(0_usize)]) == (1)));
    assert!((((*o).tag) == (1)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut local: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut local;
        (&raw mut (*__d).tag).write(0_i32);
        {
            let __d = (&raw mut (*__d).c);
            (&raw mut (*__d).n).write(0_i32);
        };
        {
            let __d = (&raw mut (*__d).arr).cast::<Counter>();
            {
                let __d = __d.offset(0);
                (&raw mut (*__d).n).write(0_i32);
            };
            {
                let __d = __d.offset(1);
                (&raw mut (*__d).n).write(0_i32);
            };
        };
        (&raw mut (*__d).v).write(Vec::new());
    };
    local.tag = 1;
    (unsafe { run_0((&mut local as *mut S)) });
    let mut heap: *mut S = {
        let __p = Box::into_raw(Box::<S>::new_uninit()) as *mut S;
        {
            let __d = __p;
            (&raw mut (*__d).tag).write(0_i32);
            {
                let __d = (&raw mut (*__d).c);
                (&raw mut (*__d).n).write(0_i32);
            };
            {
                let __d = (&raw mut (*__d).arr).cast::<Counter>();
                {
                    let __d = __d.offset(0);
                    (&raw mut (*__d).n).write(0_i32);
                };
                {
                    let __d = __d.offset(1);
                    (&raw mut (*__d).n).write(0_i32);
                };
            };
            (&raw mut (*__d).v).write(Vec::new());
        };
        __p
    };
    (*heap).tag = 1;
    (unsafe { run_0(heap) });
    {
        let __p = heap;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
