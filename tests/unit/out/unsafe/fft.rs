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
pub struct Complex {
    pub re: f64,
    pub img: f64,
}
impl Complex {
    pub unsafe fn new_1(this: *mut Complex) {}
    pub unsafe fn copy_from(this: *mut Complex, _a0: *const Complex) {
        (&raw mut (*this).re).write((*_a0).re);
        (&raw mut (*this).img).write((*_a0).img);
    }
    pub unsafe fn move_from(this: *mut Complex, _a0: *mut Complex) {
        (&raw mut (*this).re).write((*_a0).re);
        (&raw mut (*this).img).write((*_a0).img);
    }
}
pub unsafe fn Product_0(__ret: *mut Complex, z1: *mut Complex, z2: *mut Complex) {
    let mut ac: f64 = (((*z1).re) * ((*z2).re));
    let mut bd: f64 = (((*z1).img) * ((*z2).img));
    let mut ad: f64 = (((*z1).re) * ((*z2).img));
    let mut bc: f64 = (((*z1).img) * ((*z2).re));
    {
        let __d = __ret.clone();
        (&raw mut (*__d).re).write(((ac) - (bd)));
        (&raw mut (*__d).img).write(((ad) + (bc)));
    };
    return;
}
pub unsafe fn Sum_1(__ret: *mut Complex, z1: *mut Complex, z2: *mut Complex) {
    let mut ac: f64 = (((*z1).re) + ((*z2).re));
    let mut bd: f64 = (((*z1).img) + ((*z2).img));
    {
        let __d = __ret.clone();
        (&raw mut (*__d).re).write(ac);
        (&raw mut (*__d).img).write(bd);
    };
    return;
}
pub unsafe fn Neg_2(__ret: *mut Complex, z1: *mut Complex) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).re).write(-(*z1).re);
        (&raw mut (*__d).img).write(-(*z1).img);
    };
    return;
}
pub unsafe fn fft_3(a: *mut Option<Box<[Complex]>>, mut N: i32) -> Option<Box<[Complex]>> {
    let mut y: Option<Box<[Complex]>> = Some(
        (0..(N as usize))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    if ((N) == (1)) {
        (*y.as_deref()
            .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
            .add(0_usize)) = Complex {
            re: (*(*a)
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add(0_usize))
            .re,
            img: (*(*a)
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add(0_usize))
            .img,
        };
        return y.take();
    }
    let mut w: Option<Box<[Complex]>> = Some(
        (0..(N as usize))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut alpha: f64 =
                ((((-2_i32 as f64) * (3.141592653589793_f64)) * (i as f64)) / (N as f64));
            (*w.as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add((i as usize))) = Complex {
                re: alpha.cos(),
                img: alpha.sin(),
            };
            i.postfix_inc();
        }
    }
    let mut A0: Option<Box<[Complex]>> = Some(
        (0..(((N) / (2)) as usize))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    let mut A1: Option<Box<[Complex]>> = Some(
        (0..(((N) / (2)) as usize))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < ((N) / (2))) {
            (*A0.as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add((i as usize))) = Complex {
                re: (*(*a)
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                    .add((((i) * (2)) as usize)))
                .re,
                img: (*(*a)
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                    .add((((i) * (2)) as usize)))
                .img,
            };
            (*A1.as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add((i as usize))) = Complex {
                re: (*(*a)
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                    .add(((((i) * (2)) + (1)) as usize)))
                .re,
                img: (*(*a)
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                    .add(((((i) * (2)) + (1)) as usize)))
                .img,
            };
            i.postfix_inc();
        }
    }
    let mut y0: Option<Box<[Complex]>> = (unsafe { fft_3(&mut A0, ((N) / (2))) });
    let mut y1: Option<Box<[Complex]>> = (unsafe { fft_3(&mut A1, ((N) / (2))) });
    {
        let mut k: i32 = 0;
        'loop_: while ((k) < ((N) / (2))) {
            let mut yk: Complex = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let mut __tmp_0: Complex = ::std::mem::MaybeUninit::uninit().assume_init();
                Complex::copy_from(&raw mut __tmp_0, {
                    &(*y0
                        .as_deref()
                        .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                        .add((k as usize)))
                });
                let _z1 = &raw mut __tmp_0;
                let mut __tmp_1: Complex = ::std::mem::MaybeUninit::uninit().assume_init();
                (unsafe {
                    let mut __tmp_2: Complex = ::std::mem::MaybeUninit::uninit().assume_init();
                    Complex::copy_from(&raw mut __tmp_2, {
                        &(*w.as_deref()
                            .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                            .add((k as usize)))
                    });
                    let _z1 = &raw mut __tmp_2;
                    let mut __tmp_3: Complex = ::std::mem::MaybeUninit::uninit().assume_init();
                    Complex::copy_from(&raw mut __tmp_3, {
                        &(*y1
                            .as_deref()
                            .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                            .add((k as usize)))
                    });
                    let _z2 = &raw mut __tmp_3;
                    Product_0(&raw mut __tmp_1, _z1, _z2)
                });
                let _z2 = &raw mut __tmp_1;
                Sum_1(&raw mut yk, _z1, _z2)
            });
            (*y.as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add((k as usize))) = Complex {
                re: yk.re,
                img: yk.img,
            };
            let mut yk_n2: Complex = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let mut __tmp_4: Complex = ::std::mem::MaybeUninit::uninit().assume_init();
                Complex::copy_from(&raw mut __tmp_4, {
                    &(*y0
                        .as_deref()
                        .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                        .add((k as usize)))
                });
                let _z1 = &raw mut __tmp_4;
                let mut __tmp_5: Complex = ::std::mem::MaybeUninit::uninit().assume_init();
                (unsafe {
                    Neg_2(
                        &raw mut __tmp_5,
                        (|__p: *mut Complex| {
                            (unsafe {
                                let mut __tmp_6: Complex =
                                    ::std::mem::MaybeUninit::uninit().assume_init();
                                Complex::copy_from(&raw mut __tmp_6, {
                                    &(*w.as_deref()
                                        .map_or(::std::ptr::null_mut(), |s| {
                                            s.as_ptr() as *mut Complex
                                        })
                                        .add((k as usize)))
                                });
                                let _z1 = &raw mut __tmp_6;
                                let mut __tmp_7: Complex =
                                    ::std::mem::MaybeUninit::uninit().assume_init();
                                Complex::copy_from(&raw mut __tmp_7, {
                                    &(*y1
                                        .as_deref()
                                        .map_or(::std::ptr::null_mut(), |s| {
                                            s.as_ptr() as *mut Complex
                                        })
                                        .add((k as usize)))
                                });
                                let _z2 = &raw mut __tmp_7;
                                Product_0(__p, _z1, _z2)
                            });
                            __p
                        })(
                            (&mut ::std::mem::MaybeUninit::<Complex>::uninit()).as_mut_ptr()
                        ),
                    )
                });
                let _z2 = &raw mut __tmp_5;
                Sum_1(&raw mut yk_n2, _z1, _z2)
            });
            (*y.as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add((((k) + ((N) / (2))) as usize))) = Complex {
                re: yk_n2.re,
                img: yk_n2.img,
            };
            k.postfix_inc();
        }
    }
    return y.take();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut N: i32 = 4;
    let mut a: Option<Box<[Complex]>> = Some(
        (0..(N as usize))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            (*a.as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add((i as usize))) = Complex {
                re: ((i as f64) + (1_f64)),
                img: 0_f64,
            };
            i.postfix_inc();
        }
    }
    let mut b: Option<Box<[Complex]>> = (unsafe { fft_3(&mut a, N) });
    let mut reals: Option<Box<[i32]>> =
        Some((0..(N as usize)).map(|_| 0_i32).collect::<Box<[_]>>());
    let mut imgs: Option<Box<[i32]>> = Some((0..(N as usize)).map(|_| 0_i32).collect::<Box<[_]>>());
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            (*reals
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                .add((i as usize))) = (((*b
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add((i as usize)))
            .re
            .round()) as i32);
            (*imgs
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                .add((i as usize))) = (((*b
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut Complex)
                .add((i as usize)))
            .img
            .round()) as i32);
            i.prefix_inc();
        }
    }
    assert!(
        ((((((((*reals
            .as_deref()
            .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
            .add(0_usize))
            == (10))
            && ((*imgs
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                .add(0_usize))
                == (0)))
            && ((*reals
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                .add(1_usize))
                == (-2_i32)))
            && ((*imgs
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                .add(1_usize))
                == (2)))
            && ((*reals
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                .add(2_usize))
                == (-2_i32)))
            && ((*imgs
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                .add(2_usize))
                == (0)))
            && ((*reals
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                .add(3_usize))
                == (-2_i32)))
            && ((*imgs
                .as_deref()
                .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                .add(3_usize))
                == (-2_i32))
    );
    let mut l: i64 = -5000000000_i64;
    let mut ll: i64 = -6000000000_i64;
    assert!(((l.abs()) == (5000000000_i64)));
    assert!(((ll.abs()) == (6000000000_i64)));
    let mut small: i64 = (-7_i32 as i64);
    let mut small_ll: i64 = (-8_i32 as i64);
    let mut narrowed: i32 = ((small.abs()) as i32);
    let mut narrowed_ll: i32 = ((small_ll.abs()) as i32);
    assert!(((narrowed) == (7)));
    assert!(((narrowed_ll) == (8)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
