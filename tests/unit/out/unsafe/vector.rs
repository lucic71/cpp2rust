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
    pub x: i32,
}
pub unsafe fn copy_0(mut copy_vector: Vec<i32>) {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v1: Vec<i32> = Vec::new();
    assert!(((v1.len()) == (0_usize)));
    assert!(v1.is_empty());
    {
        let __a1 = 1;
        v1.push(__a1)
    };
    assert!(!(v1.is_empty()));
    v1.pop();
    assert!(v1.is_empty());
    let mut s1: usize = v1.len();
    {
        let __a0 = 100_usize as usize;
        v1.resize_with(__a0, || 0_i32)
    };
    assert!(((v1.len()) == (100_usize)));
    assert!(
        ((*{
            let __idx = 99_usize as usize;
            let __v: &mut Vec<i32> = &mut v1;
            __v.as_mut_ptr().add(__idx)
        }) == (0))
    );
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut v1;
        __v.as_mut_ptr().add(__idx)
    }) = 40;
    (*{
        let __idx = 99_usize as usize;
        let __v: &mut Vec<i32> = &mut v1;
        __v.as_mut_ptr().add(__idx)
    }) = 50;
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v1;
            __v.as_mut_ptr().add(__idx)
        }) == (40))
    );
    assert!(
        ((*{
            let __idx = 99_usize as usize;
            let __v: &mut Vec<i32> = &mut v1;
            __v.as_mut_ptr().add(__idx)
        }) == (50))
    );
    let mut v2: Vec<i32> = Vec::new();
    assert!(((v2.len()) == (0_usize)));
    {
        let __a1 = 1;
        v2.push(__a1)
    };
    {
        let __a1 = 2;
        v2.push(__a1)
    };
    {
        let __a1 = 3;
        v2.push(__a1)
    };
    assert!(((v2.len()) == (3_usize)));
    {
        let __it = <UnsafeRandomAccessIterator<i32>>::from(v2.begin());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
        v2.remove(pos);
        __it
    };
    assert!(((v2.len()) == (2_usize)));
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (2))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (3))
    );
    {
        let __it = <UnsafeRandomAccessIterator<i32>>::from(v2.begin());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
        v2.insert(pos, 100);
        __it
    };
    (unsafe { copy_0(v2.clone()) });
    assert!(((v2.len()) == (3_usize)));
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (100))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (2))
    );
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (3))
    );
    let mut x: i32 = 7;
    {
        let __it = <UnsafeRandomAccessIterator<i32>>::from(v2.begin().iter_add(1_isize));
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
        v2.insert(pos, x);
        __it
    };
    assert!(((v2.len()) == (4_usize)));
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (100))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (7))
    );
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (2))
    );
    {
        let __off = <UnsafeRandomAccessIterator<i32>>::from(v2.end())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
            as usize;
        v2.splice(__off..__off, std::iter::repeat_n(9, 2_usize));
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(__off))
    };
    assert!(((v2.len()) == (6_usize)));
    assert!(
        ((*{
            let __idx = 4_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (9))
    );
    assert!(
        ((*{
            let __idx = 5_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (9))
    );
    {
        let __off = <UnsafeRandomAccessIterator<i32>>::from(v2.begin().iter_add(2_isize))
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
            as usize;
        v2.splice(__off..__off, Vec::from([11, 12, 13]));
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(__off))
    };
    assert!(((v2.len()) == (9_usize)));
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (7))
    );
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (11))
    );
    assert!(
        ((*{
            let __idx = 3_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (12))
    );
    assert!(
        ((*{
            let __idx = 4_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (13))
    );
    assert!(
        ((*{
            let __idx = 5_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (2))
    );
    assert!(
        ((*{
            let __idx = 8_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (9))
    );
    let mut src: Vec<i32> = Vec::from([21, 22]);
    {
        let __off = <UnsafeRandomAccessIterator<i32>>::from(v2.begin())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
            as usize;
        let mut __tmp = Vec::new();
        let mut __it = src.begin();
        while __it != src.end() {
            __tmp.push((*__it.iter_deref()).clone());
            __it.iter_inc();
        }
        v2.splice(__off..__off, __tmp);
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(__off))
    };
    assert!(((v2.len()) == (11_usize)));
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (21))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (22))
    );
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (100))
    );
    {
        let __it = <UnsafeRandomAccessIterator<i32>>::from(v2.begin());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
        v2.remove(pos);
        __it
    };
    {
        let __it = <UnsafeRandomAccessIterator<i32>>::from(v2.begin());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
        v2.remove(pos);
        __it
    };
    let mut i: i32 = 0;
    'loop_: while ((i) < (4)) {
        {
            let __it = <UnsafeRandomAccessIterator<i32>>::from(v2.begin().iter_add(1_isize));
            let pos =
                __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
            v2.remove(pos);
            __it
        };
        i.prefix_inc();
    }
    v2.pop();
    v2.pop();
    assert!(((v2.len()) == (3_usize)));
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (100))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (2))
    );
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        }) == (3))
    );
    let mut s2: usize = v2.len();
    let mut v3: Vec<i32> = vec![1; 100_usize as usize];
    assert!(((v3.len()) == (100_usize)));
    let mut i: i32 = 0;
    'loop_: while ((i) < (100)) {
        assert!(
            ((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<i32> = &mut v3;
                __v.as_mut_ptr().add(__idx)
            }) == (1))
        );
        i.prefix_inc();
    }
    let mut v4: Vec<*mut i32> = (0..(100_usize) as usize)
        .map(|_| std::ptr::null_mut())
        .collect::<Vec<_>>();
    assert!(((v4.len()) == (100_usize)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (v4.len())) {
        assert!(
            (*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<*mut i32> = &mut v4;
                __v.as_mut_ptr().add(__idx)
            })
            .is_null()
        );
        i.prefix_inc();
    }
    let mut v5: Vec<*const i32> = (0..(100_usize) as usize)
        .map(|_| std::ptr::null())
        .collect::<Vec<_>>();
    assert!(((v5.len()) == (100_usize)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (v5.len())) {
        assert!(
            (*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<*const i32> = &mut v5;
                __v.as_mut_ptr().add(__idx)
            })
            .is_null()
        );
        i.prefix_inc();
    }
    let mut v6: Vec<f64> = vec![2_f64; s2 as usize];
    assert!(((v6.len()) == (s2)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s2)) {
        assert!(
            ((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<f64> = &mut v6;
                __v.as_mut_ptr().add(__idx)
            }) == (2_f64))
        );
        i.prefix_inc();
    }
    let mut v7: Vec<(*const i32, i32)> = (0..(200_usize) as usize)
        .map(|_| <(*const i32, i32)>::default())
        .collect::<Vec<_>>();
    assert!(((v7.len()) == (200_usize)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (200_u32)) {
        assert!(
            (((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<(*const i32, i32)> = &mut v7;
                __v.as_mut_ptr().add(__idx)
            })
            .0)
                .is_null())
                && (((*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<(*const i32, i32)> = &mut v7;
                    __v.as_mut_ptr().add(__idx)
                })
                .1) == (0))
        );
        i.prefix_inc();
    }
    let mut p1: *const f64 = (v6.as_mut_ptr()).cast_const();
    assert!(((*p1) == (2_f64)));
    let mut p2: *mut i32 = v3.as_mut_ptr();
    assert!(((*p2) == (1)));
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v3;
            __v.as_mut_ptr().add(__idx)
        }) == (1))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v3;
            __v.as_mut_ptr().add(__idx)
        }) == (1))
    );
    (*p2) = (99_f64 as i32);
    assert!(((*p2) == (99)));
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v3;
            __v.as_mut_ptr().add(__idx)
        }) == (99))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v3;
            __v.as_mut_ptr().add(__idx)
        }) == (1))
    );
    p2.prefix_inc();
    (*p2) = 98;
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v3;
            __v.as_mut_ptr().add(__idx)
        }) == (99))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v3;
            __v.as_mut_ptr().add(__idx)
        }) == (98))
    );
    assert!(((v3.capacity()) == (100_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 200_usize as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(200_usize as usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (200_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 50_usize as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(50_usize as usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (200_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 200_usize as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(200_usize as usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (200_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 201_usize as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(201_usize as usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (201_usize)));
    assert!(((v3.len()) == (100_usize)));
    assert!(((*((v2).last_mut().unwrap())) == (3)));
    assert!(((*((v3).last_mut().unwrap())) == (1)));
    assert!((*((v4).last_mut().unwrap())).is_null());
    assert!((*((v5).last_mut().unwrap())).is_null());
    assert!(((*((v6).last_mut().unwrap())) == (2_f64)));
    let ref0: *mut f64 = ((v6).last_mut().unwrap());
    (*ref0) = 5_f64;
    assert!(((*((v6).last_mut().unwrap())) == (5_f64)));
    let mut x0: f64 = (*((v6).last_mut().unwrap()));
    assert!(((x0) == (5_f64)));
    x0 = 6_f64;
    assert!(((*((v6).last_mut().unwrap())) == (5_f64)));
    let mut idx: i32 = 0;
    assert!(((*(&mut (v6)[(idx as usize) as usize] as *mut f64)) == (2_f64)));
    assert!(((*(&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64)) == (5_f64)));
    let ref1: *mut f64 = (&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64);
    (*ref1) += 1.5_f64;
    assert!(((*(&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64)) == (6.5_f64)));
    let mut x1: f64 = (*(&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64));
    assert!(((x1) == (6.5_f64)));
    x1 -= 1.5_f64;
    assert!(((*(&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64)) == (6.5_f64)));
    assert!(
        ((((s1).wrapping_add(s2))
            .wrapping_add(((*(&mut (v2)[0_usize as usize] as *mut i32)) as usize)))
            == (103_usize))
    );
    let mut a: S = S { x: 1 };
    let mut pa: *mut S = (&mut a as *mut S);
    let mut v8: Vec<*const *mut S> = Vec::new();
    {
        let __a1 = (&mut pa as *mut *mut S).cast_const();
        v8.push(__a1)
    };
    assert!(
        (((*(*(*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<*const *mut S> = &mut v8;
            __v.as_mut_ptr().add(__idx)
        })))
        .x) == (1))
    );
    let mut ppa: *mut *mut S = (&mut pa as *mut *mut S);
    let mut v9: Vec<*mut *mut *mut S> = Vec::new();
    {
        let __a1 = (&mut ppa as *mut *mut *mut S);
        v9.push(__a1)
    };
    (*(*(*(*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<*mut *mut *mut S> = &mut v9;
        __v.as_mut_ptr().add(__idx)
    }))))
    .x = 2;
    assert!(((a.x) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
