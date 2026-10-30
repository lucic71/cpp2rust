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
pub struct Elem {
    pub id: i32,
    pub hist: [i32; 4],
    pub coeffs: Vec<i16>,
}
impl Elem {
    pub unsafe fn move_from(_a0: *mut Elem) -> Self {
        let mut this = Self {
            id: (*_a0).id,
            hist: std::array::from_fn::<_, 4, _>(|__i: usize| (*_a0).hist[(__i)]),
            coeffs: std::mem::take(&mut (*_a0).coeffs),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Elem) -> *mut Elem {
        self.id = (*_a0).id;
        {
            if 16_usize != 0 {
                ::std::ptr::copy_nonoverlapping(
                    ((&mut (*_a0).hist as *mut [i32; 4]) as *const ::libc::c_void),
                    ((&mut self.hist as *mut [i32; 4]) as *mut ::libc::c_void),
                    16_usize as usize,
                )
            }
            ((&mut self.hist as *mut [i32; 4]) as *mut ::libc::c_void)
        };
        self.coeffs = std::mem::take(&mut (*_a0).coeffs);
        return &mut (*(self as *mut Elem));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct State {
    pub count: i32,
    pub grid: [[i32; 3]; 2],
    pub arr: Vec<i32>,
    pub elems: Vec<Elem>,
    pub rows: Vec<Vec<i32>>,
}
impl State {
    pub unsafe fn move_from(_a0: *mut State) -> Self {
        let mut this = Self {
            count: (*_a0).count,
            grid: std::array::from_fn::<_, 2, _>(|__i: usize| {
                std::array::from_fn::<_, 3, _>(|__i2: usize| (*_a0).grid[(__i)][(__i2)])
            }),
            arr: std::mem::take(&mut (*_a0).arr),
            elems: std::mem::take(&mut (*_a0).elems),
            rows: std::mem::take(&mut (*_a0).rows),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut State) -> *mut State {
        self.count = (*_a0).count;
        {
            if 24_usize != 0 {
                ::std::ptr::copy_nonoverlapping(
                    ((&mut (*_a0).grid as *mut [[i32; 3]; 2]) as *const ::libc::c_void),
                    ((&mut self.grid as *mut [[i32; 3]; 2]) as *mut ::libc::c_void),
                    24_usize as usize,
                )
            }
            ((&mut self.grid as *mut [[i32; 3]; 2]) as *mut ::libc::c_void)
        };
        self.arr = std::mem::take(&mut (*_a0).arr);
        self.elems = std::mem::take(&mut (*_a0).elems);
        self.rows = std::mem::take(&mut (*_a0).rows);
        return &mut (*(self as *mut State));
    }
}
impl Default for State {
    fn default() -> Self {
        unsafe {
            State {
                count: 0_i32,
                grid: [[0_i32; 3]; 2],
                arr: std::array::from_fn::<_, 3_usize, _>(|_| 0_i32).to_vec(),
                elems: Default::default(),
                rows: Default::default(),
            }
        }
    }
}
pub unsafe fn sum_0(mut p: *const i32, mut n: i32) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        s += (*p.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub unsafe fn fill_1(mut s: *mut State) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (2)) {
        let mut j: i32 = 0;
        'loop_: while ((j) < (3)) {
            (*s).grid[(i) as usize][(j) as usize] = (((i) * (3)) + (j));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut (*s).arr;
        __v.as_mut_ptr().add(__idx)
    }) = 7;
    {
        let __a0 = 3_usize as usize;
        (*s).elems.resize_with(__a0, || <Elem>::default())
    };
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (*{
            let __idx = (i as usize) as usize;
            let __v: &mut Vec<Elem> = &mut (*s).elems;
            __v.as_mut_ptr().add(__idx)
        })
        .id = i;
        (*{
            let __idx = (i as usize) as usize;
            let __v: &mut Vec<Elem> = &mut (*s).elems;
            __v.as_mut_ptr().add(__idx)
        })
        .hist[(i) as usize] = ((10) * (i));
        {
            let __a0 = 4_usize as usize;
            (*{
                let __idx = (i as usize) as usize;
                let __v: &mut Vec<Elem> = &mut (*s).elems;
                __v.as_mut_ptr().add(__idx)
            })
            .coeffs
            .resize(__a0, (i as i16))
        };
        i.prefix_inc();
    }
    {
        let __a1 = Vec::from([1, 2, 3]);
        (*s).rows.push(__a1)
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: State = <State>::default();
    (unsafe { fill_1((&mut s as *mut State)) });
    assert!(
        ((s.grid[(1) as usize][(2) as usize]) == (5))
            && ((unsafe { sum_0((s.grid[(1) as usize].as_mut_ptr()).cast_const(), 3,) }) == (12))
    );
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut s.arr;
            __v.as_mut_ptr().add(__idx)
        }) == (7))
    );
    let mut e: *mut Elem = (&mut (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<Elem> = &mut s.elems;
        __v.as_mut_ptr().add(__idx)
    }) as *mut Elem);
    let mut h: *mut i32 = (*e).hist.as_mut_ptr();
    let mut c: *mut i16 = (&mut (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i16> = &mut (*e).coeffs;
        __v.as_mut_ptr().add(__idx)
    }) as *mut i16);
    let mut id: *mut i32 = (&mut (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<Elem> = &mut s.elems;
        __v.as_mut_ptr().add(__idx)
    })
    .id as *mut i32);
    let mut k: i32 = 0;
    'loop_: while ((k) < (4)) {
        (*h.offset((k) as isize)) += k;
        k.prefix_inc();
    }
    (*c) = (((*c) as i32) + 5) as i16;
    (*id) = 11;
    s.count = 3;
    assert!(
        ((unsafe {
            sum_0(
                ((*{
                    let __idx = 2_usize as usize;
                    let __v: &mut Vec<Elem> = &mut s.elems;
                    __v.as_mut_ptr().add(__idx)
                })
                .hist
                .as_mut_ptr())
                .cast_const(),
                4,
            )
        }) == (26))
            && (((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<i16> = &mut (*{
                    let __idx = 2_usize as usize;
                    let __v: &mut Vec<Elem> = &mut s.elems;
                    __v.as_mut_ptr().add(__idx)
                })
                .coeffs;
                __v.as_mut_ptr().add(__idx)
            }) as i32)
                == (7))
    );
    assert!(
        (((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<Elem> = &mut s.elems;
            __v.as_mut_ptr().add(__idx)
        })
        .id) == (11))
            && ((h.offset((3) as isize))
                == (&mut (*{
                    let __idx = 2_usize as usize;
                    let __v: &mut Vec<Elem> = &mut s.elems;
                    __v.as_mut_ptr().add(__idx)
                })
                .hist[(3) as usize] as *mut i32))
    );
    let mut row: *mut i32 = (&mut (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<Vec<i32>> = &mut s.rows;
            __v.as_mut_ptr().add(__idx)
        });
        __v.as_mut_ptr().add(__idx)
    }) as *mut i32);
    (*row.offset((2) as isize)) = 9;
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<Vec<i32>> = &mut s.rows;
                __v.as_mut_ptr().add(__idx)
            });
            __v.as_mut_ptr().add(__idx)
        }) == (9))
    );
    let mut t: State = s.clone();
    (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<Elem> = &mut t.elems;
        __v.as_mut_ptr().add(__idx)
    })
    .hist[(0) as usize] = 100;
    t.grid[(0) as usize][(0) as usize] = 100;
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<Vec<i32>> = &mut t.rows;
            __v.as_mut_ptr().add(__idx)
        });
        __v.as_mut_ptr().add(__idx)
    }) = 100;
    assert!(
        ((((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<Elem> = &mut s.elems;
            __v.as_mut_ptr().add(__idx)
        })
        .hist[(0) as usize])
            == (0))
            && ((s.grid[(0) as usize][(0) as usize]) == (0)))
            && ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut (*{
                    let __idx = 0_usize as usize;
                    let __v: &mut Vec<Vec<i32>> = &mut s.rows;
                    __v.as_mut_ptr().add(__idx)
                });
                __v.as_mut_ptr().add(__idx)
            }) == (1))
    );
    assert!(
        (((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i16> = &mut (*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<Elem> = &mut t.elems;
                __v.as_mut_ptr().add(__idx)
            })
            .coeffs;
            __v.as_mut_ptr().add(__idx)
        }) as i32)
            == (7))
    );
    let mut p: *mut State = (&mut t as *mut State);
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<Elem> = &mut (*p).elems;
        __v.as_mut_ptr().add(__idx)
    })
    .hist[(3) as usize] = (((*p).grid[(1) as usize][(1) as usize])
        + (*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut (*p).arr;
            __v.as_mut_ptr().add(__idx)
        }));
    assert!(
        (((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<Elem> = &mut t.elems;
            __v.as_mut_ptr().add(__idx)
        })
        .hist[(3) as usize])
            == (11))
    );
    {
        let a0_clone = (*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<Elem> = &mut (*p).elems;
            __v.as_mut_ptr().add(__idx)
        })
        .clone();
        (*p).elems.push(a0_clone)
    };
    assert!(
        (((*p).elems.len()) == (4_usize))
            && (((*{
                let __idx = 3_usize as usize;
                let __v: &mut Vec<Elem> = &mut (*p).elems;
                __v.as_mut_ptr().add(__idx)
            })
            .hist[(0) as usize])
                == (100))
    );
    let mut elems: Vec<Elem> = (0..(2_usize) as usize)
        .map(|_| <Elem>::default())
        .collect::<Vec<_>>();
    let mut first: *mut Elem = (&mut (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<Elem> = &mut elems;
        __v.as_mut_ptr().add(__idx)
    }) as *mut Elem);
    {
        let __a0 = 2_usize as usize;
        (*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<Elem> = &mut elems;
            __v.as_mut_ptr().add(__idx)
        })
        .coeffs
        .resize_with(__a0, || 0_i16)
    };
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i16> = &mut (*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<Elem> = &mut elems;
            __v.as_mut_ptr().add(__idx)
        })
        .coeffs;
        __v.as_mut_ptr().add(__idx)
    }) = 4_i16;
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<Elem> = &mut elems;
        __v.as_mut_ptr().add(__idx)
    })
    .hist[(2) as usize] = (((*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i16> = &mut (*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<Elem> = &mut elems;
            __v.as_mut_ptr().add(__idx)
        })
        .coeffs;
        __v.as_mut_ptr().add(__idx)
    }) as i32)
        + (1));
    assert!(
        (((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<Elem> = &mut elems;
            __v.as_mut_ptr().add(__idx)
        })
        .hist[(2) as usize])
            == (5))
            && (((*first).hist[(2) as usize]) == (0))
    );
    let mut moved: State = State::move_from({ &mut t });
    assert!(
        ((moved.grid[(0) as usize][(0) as usize]) == (100))
            && ((moved.grid[(1) as usize][(2) as usize]) == (5))
    );
    assert!((((moved.elems.len()) == (4_usize)) && (t.elems.is_empty())) && (t.rows.is_empty()));
    let mut assigned: State = <State>::default();
    (unsafe { State::move_assign(&mut assigned, &mut moved) });
    assert!(
        ((assigned.grid[(0) as usize][(0) as usize]) == (100))
            && ((assigned.grid[(1) as usize][(1) as usize]) == (4))
    );
    assert!(
        (((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i16> = &mut (*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<Elem> = &mut assigned.elems;
                __v.as_mut_ptr().add(__idx)
            })
            .coeffs;
            __v.as_mut_ptr().add(__idx)
        }) as i32)
            == (7))
            && (moved.elems.is_empty())
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
