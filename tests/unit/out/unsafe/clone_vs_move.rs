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
pub struct Bar {
    pub w: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {
    pub x: i32,
    pub y: *mut i32,
    pub z: *mut i32,
    pub a: [i32; 3],
    pub bar: Bar,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Refs {
    pub a: *mut i32,
    pub b: *mut i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: i32 = 1;
    let mut x2: i32 = x1;
    x2.prefix_inc();
    assert!(((x1) == (1)));
    assert!(((x2) == (2)));
    let mut x3: f64 = 3_f64;
    let mut x4: f64 = x3;
    x4.prefix_inc();
    assert!(((x3) == (3_f64)));
    assert!(((x4) == (4_f64)));
    let reference: *mut i32 = &mut x1;
    let mut x5: i32 = (*reference);
    x5.prefix_inc();
    assert!(((*reference) == (1)));
    assert!(((x5) == (2)));
    let mut pointer: *mut i32 = (&mut x1 as *mut i32);
    let mut x6: i32 = (*pointer);
    x6.prefix_inc();
    assert!(((*pointer) == (1)));
    assert!(((x6) == (2)));
    let mut other_pointer: *mut i32 = pointer;
    assert!(((other_pointer) == (pointer)));
    (*other_pointer).prefix_inc();
    assert!(((*other_pointer) == (*pointer)));
    let mut f1: Foo = Foo {
        x: 1,
        y: &mut x1,
        z: (&mut x1 as *mut i32),
        a: [0, 1, 2],
        bar: Bar { w: 10 },
    };
    assert!(((f1.x) == (1)));
    assert!(((*f1.y) == (2)));
    assert!(((f1.z) == (&mut x1 as *mut i32)));
    assert!(((*f1.z) == (2)));
    let mut f2: Foo = f1;
    f2.x.prefix_inc();
    (*f2.y).prefix_inc();
    assert!(((f2.x) == (2)));
    assert!(((*f2.y) == (3)));
    assert!(((f1.x) == (1)));
    assert!(((*f1.y) == (3)));
    (*f2.z).prefix_inc();
    assert!(((*f2.y) == (4)));
    assert!(((f2.z) == (&mut x1 as *mut i32)));
    assert!(((*f2.z) == (4)));
    assert!(((*f1.y) == (4)));
    assert!(((f1.z) == (&mut x1 as *mut i32)));
    assert!(((*f1.z) == (4)));
    f2.a[(0) as usize].prefix_inc();
    f2.a[(1) as usize].prefix_inc();
    f2.a[(2) as usize].prefix_inc();
    assert!(((f2.a[(0) as usize]) == (1)));
    assert!(((f2.a[(1) as usize]) == (2)));
    assert!(((f2.a[(2) as usize]) == (3)));
    assert!(((f1.a[(0) as usize]) == (0)));
    assert!(((f1.a[(1) as usize]) == (1)));
    assert!(((f1.a[(2) as usize]) == (2)));
    f2.bar.w = 20;
    assert!(((f2.bar.w) == (20)));
    assert!(((f1.bar.w) == (10)));
    let mut N: i32 = 5;
    let mut v1: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        {
            let a0_clone = i.clone();
            v1.push(a0_clone)
        };
        i.prefix_inc();
    }
    let mut v2: Vec<i32> = v1.clone();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(
            ((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<i32> = &mut v2;
                __v.as_mut_ptr().add(__idx)
            }) == (i))
        );
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*{
            let __idx = (i as usize) as usize;
            let __v: &mut Vec<i32> = &mut v2;
            __v.as_mut_ptr().add(__idx)
        })
        .prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(
            ((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<i32> = &mut v2;
                __v.as_mut_ptr().add(__idx)
            }) == ((i) + (1)))
        );
        assert!(
            ((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<i32> = &mut v1;
                __v.as_mut_ptr().add(__idx)
            }) == (i))
        );
        i.prefix_inc();
    }
    let mut m1: Vec<Vec<i32>> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        {
            let __a1 = (0..(10_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>();
            m1.push(__a1)
        };
        i.prefix_inc();
    }
    let mut m2: Vec<Vec<i32>> = m1.clone();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(
            (((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<Vec<i32>> = &mut m1;
                __v.as_mut_ptr().add(__idx)
            })
            .len())
                == (10_usize))
        );
        assert!(
            (((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<Vec<i32>> = &mut m2;
                __v.as_mut_ptr().add(__idx)
            })
            .len())
                == (10_usize))
        );
        let mut j: i32 = 0;
        'loop_: while ((j) < (10)) {
            assert!(
                ((*{
                    let __idx = (j as usize) as usize;
                    let __v: &mut Vec<i32> = &mut (*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<Vec<i32>> = &mut m1;
                        __v.as_mut_ptr().add(__idx)
                    });
                    __v.as_mut_ptr().add(__idx)
                }) == (0))
            );
            assert!(
                ((*{
                    let __idx = (j as usize) as usize;
                    let __v: &mut Vec<i32> = &mut (*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<Vec<i32>> = &mut m2;
                        __v.as_mut_ptr().add(__idx)
                    });
                    __v.as_mut_ptr().add(__idx)
                }) == (0))
            );
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        let mut j: i32 = 0;
        'loop_: while ((j) < (10)) {
            (*{
                let __idx = (j as usize) as usize;
                let __v: &mut Vec<i32> = &mut (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<Vec<i32>> = &mut m2;
                    __v.as_mut_ptr().add(__idx)
                });
                __v.as_mut_ptr().add(__idx)
            })
            .postfix_inc();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(
            (((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<Vec<i32>> = &mut m1;
                __v.as_mut_ptr().add(__idx)
            })
            .len())
                == (10_usize))
        );
        assert!(
            (((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<Vec<i32>> = &mut m2;
                __v.as_mut_ptr().add(__idx)
            })
            .len())
                == (10_usize))
        );
        let mut j: i32 = 0;
        'loop_: while ((j) < (10)) {
            assert!(
                ((*{
                    let __idx = (j as usize) as usize;
                    let __v: &mut Vec<i32> = &mut (*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<Vec<i32>> = &mut m1;
                        __v.as_mut_ptr().add(__idx)
                    });
                    __v.as_mut_ptr().add(__idx)
                }) == (0))
            );
            assert!(
                ((*{
                    let __idx = (j as usize) as usize;
                    let __v: &mut Vec<i32> = &mut (*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<Vec<i32>> = &mut m2;
                        __v.as_mut_ptr().add(__idx)
                    });
                    __v.as_mut_ptr().add(__idx)
                }) == (1))
            );
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut map1: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*map1.subscript(&i)) = i;
        i.prefix_inc();
    }
    let mut map2: UnsafeMap<i32, (i32, i32)> = map1.clone();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((*map2.subscript(&i)) == (i)));
        (*map2.subscript(&i)).prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((*map1.subscript(&i)) == (i)));
        assert!(((*map2.subscript(&i)) == ((i) + (1))));
        i.prefix_inc();
    }
    let mut pair1: (i32, i32) = (1, 2);
    let mut pair2: (i32, i32) = {
        let __a0 = (pair1).clone();
        ((*&__a0.0), (*&__a0.1))
    };
    pair2.0 = ((pair2.0) * (10));
    pair2.1 = ((pair2.1) * (10));
    assert!(((pair2.0) == (10)));
    assert!(((pair2.1) == (20)));
    assert!(((pair1.0) == (1)));
    assert!(((pair1.1) == (2)));
    let mut pair3: (Vec<i32>, i32) = (
        (0..(0_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        0,
    );
    let mut pair4: (Vec<i32>, i32) = {
        let __a0 = (pair3).clone();
        (
            <Vec<i32> as CppFromUnsafe<Vec<i32>>>::cpp_from(&__a0.0),
            (*&__a0.1),
        )
    };
    {
        let __a1 = 1;
        pair4.0.push(__a1)
    };
    pair4.1 = 1;
    assert!(((pair4.0.len()) == (1_usize)));
    assert!(((pair4.1) == (1)));
    assert!(((pair3.0.len()) == (0_usize)));
    assert!(((pair3.1) == (0)));
    let mut s1: Vec<libc::c_char> = vec![('a' as libc::c_char); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect();
    let mut s2: Vec<libc::c_char> = s1.clone();
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<libc::c_char> = &mut s2;
        __v.as_mut_ptr().add(__idx)
    }) = ('b' as libc::c_char);
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<libc::c_char> = &mut s2;
        __v.as_mut_ptr().add(__idx)
    }) = ('b' as libc::c_char);
    (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<libc::c_char> = &mut s2;
        __v.as_mut_ptr().add(__idx)
    }) = ('b' as libc::c_char);
    assert!(
        (((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<libc::c_char> = &mut s2;
            __v.as_mut_ptr().add(__idx)
        }) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<libc::c_char> = &mut s2;
            __v.as_mut_ptr().add(__idx)
        }) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<libc::c_char> = &mut s2;
            __v.as_mut_ptr().add(__idx)
        }) as i32)
            == (('b' as libc::c_char) as i32))
    );
    assert!(
        (((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<libc::c_char> = &mut s1;
            __v.as_mut_ptr().add(__idx)
        }) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<libc::c_char> = &mut s1;
            __v.as_mut_ptr().add(__idx)
        }) as i32)
            == (('a' as libc::c_char) as i32))
    );
    assert!(
        (((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<libc::c_char> = &mut s1;
            __v.as_mut_ptr().add(__idx)
        }) as i32)
            == (('a' as libc::c_char) as i32))
    );
    let mut b1: Bar = Bar { w: 1 };
    let mut b2: Bar = Bar { w: 2 };
    b2 = b1;
    b2.w.postfix_inc();
    assert!(((b1.w) == (1)));
    assert!(((b2.w) == (2)));
    let mut v4: Vec<i32> = Vec::new();
    v4 = v2.clone();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(
            ((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<i32> = &mut v4;
                __v.as_mut_ptr().add(__idx)
            }) == ((i) + (1)))
        );
        (*{
            let __idx = (i as usize) as usize;
            let __v: &mut Vec<i32> = &mut v4;
            __v.as_mut_ptr().add(__idx)
        })
        .prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(
            ((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<i32> = &mut v4;
                __v.as_mut_ptr().add(__idx)
            }) == ((i) + (2)))
        );
        assert!(
            ((*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<i32> = &mut v2;
                __v.as_mut_ptr().add(__idx)
            }) == ((i) + (1)))
        );
        i.prefix_inc();
    }
    let mut ra: i32 = 1;
    let mut rb: i32 = 2;
    let mut r1: Refs = Refs {
        a: &mut ra,
        b: &mut rb,
    };
    let mut r2: Refs = r1;
    (*r2.a) = 10;
    (*r2.b).prefix_inc();
    assert!(((ra) == (10)));
    assert!(((rb) == (3)));
    assert!(((*r1.a) == (10)));
    assert!(((*r1.b) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
