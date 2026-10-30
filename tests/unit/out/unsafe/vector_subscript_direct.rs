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
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub unsafe fn sum(&self) -> i32 {
        return ((self.x) + (self.y));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub values: Vec<i32>,
    pub points: Vec<Point>,
}
impl Holder {
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            values: std::mem::take(&mut (*_a0).values),
            points: std::mem::take(&mut (*_a0).points),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.values = std::mem::take(&mut (*_a0).values);
        self.points = std::mem::take(&mut (*_a0).points);
        return &mut (*(self as *mut Holder));
    }
}
pub unsafe fn push_and_index_0(v: *mut Vec<i32>) -> i32 {
    {
        let __a1 = 42;
        (*v).push(__a1)
    };
    return (((*v).len() as i32) - (1));
}
pub unsafe fn sum_ref_1(v: *const Vec<i32>) -> i32 {
    let mut s: i32 = 0;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < ((*v).len())) {
        s += (*{
            let __idx = i as usize;
            let __v: &Vec<i32> = &(*v);
            __v.as_ptr().add(__idx)
        });
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = Vec::from([1, 2, 3]);
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut v;
        __v.as_mut_ptr().add(__idx)
    }) = 10;
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v;
            __v.as_mut_ptr().add(__idx)
        }) == (10))
    );
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut v;
        __v.as_mut_ptr().add(__idx)
    }) += 5;
    (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<i32> = &mut v;
        __v.as_mut_ptr().add(__idx)
    })
    .postfix_inc();
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut v;
            __v.as_mut_ptr().add(__idx)
        }) == (7))
            && ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<i32> = &mut v;
                __v.as_mut_ptr().add(__idx)
            }) == (4))
    );
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut v;
        __v.as_mut_ptr().add(__idx)
    }) = (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut v;
        __v.as_mut_ptr().add(__idx)
    });
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut v;
            __v.as_mut_ptr().add(__idx)
        }) == (7))
    );
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut v;
        __v.as_mut_ptr().add(__idx)
    }) = 0;
    assert!(
        ((*{
            let __idx = ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<i32> = &mut v;
                __v.as_mut_ptr().add(__idx)
            }) as usize) as usize;
            let __v: &mut Vec<i32> = &mut v;
            __v.as_mut_ptr().add(__idx)
        }) == (7))
    );
    let mut i: i32 = 0;
    (*{
        let __idx = (i.postfix_inc() as usize) as usize;
        let __v: &mut Vec<i32> = &mut v;
        __v.as_mut_ptr().add(__idx)
    }) = 3;
    assert!(
        ((i) == (1))
            && ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut v;
                __v.as_mut_ptr().add(__idx)
            }) == (3))
    );
    assert!(
        ((*{
            let __idx = ((unsafe { push_and_index_0(&mut v) }) as usize) as usize;
            let __v: &mut Vec<i32> = &mut v;
            __v.as_mut_ptr().add(__idx)
        }) == (42))
    );
    (*{
        let __idx = ((unsafe { push_and_index_0(&mut v) }) as usize) as usize;
        let __v: &mut Vec<i32> = &mut v;
        __v.as_mut_ptr().add(__idx)
    }) = 5;
    assert!(
        ((v.len()) == (5_usize))
            && ((*{
                let __idx = 4_usize as usize;
                let __v: &mut Vec<i32> = &mut v;
                __v.as_mut_ptr().add(__idx)
            }) == (5))
    );
    let mut p: *mut i32 = (&mut (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<i32> = &mut v;
        __v.as_mut_ptr().add(__idx)
    }) as *mut i32);
    (*p) = 9;
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut v;
            __v.as_mut_ptr().add(__idx)
        }) == (9))
    );
    assert!(((unsafe { sum_ref_1(&v,) }) == (((((3) + (0)) + (9)) + (42)) + (5))));
    let mut h: Holder = <Holder>::default();
    {
        let __a0 = 2_usize as usize;
        h.values.resize_with(__a0, || 0_i32)
    };
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut h.values;
        __v.as_mut_ptr().add(__idx)
    }) = 6;
    {
        let __a1 = Point { x: 1, y: 2 };
        h.points.push(__a1)
    };
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<Point> = &mut h.points;
        __v.as_mut_ptr().add(__idx)
    })
    .y = 5;
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut h.values;
            __v.as_mut_ptr().add(__idx)
        }) == (6))
    );
    assert!(
        ((unsafe {
            Point::sum(
                &(*{
                    let __idx = 0_usize as usize;
                    let __v: &mut Vec<Point> = &mut h.points;
                    __v.as_mut_ptr().add(__idx)
                }),
            )
        }) == (6))
    );
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut (*hp).values;
        __v.as_mut_ptr().add(__idx)
    }) = ((*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut (*hp).values;
        __v.as_mut_ptr().add(__idx)
    }) + (1));
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<Point> = &mut (*hp).points;
        __v.as_mut_ptr().add(__idx)
    })
    .x = (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut (*hp).values;
        __v.as_mut_ptr().add(__idx)
    });
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut h.values;
            __v.as_mut_ptr().add(__idx)
        }) == (7))
            && (((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<Point> = &mut h.points;
                __v.as_mut_ptr().add(__idx)
            })
            .x) == (7))
    );
    let mut q: Point = (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<Point> = &mut h.points;
        __v.as_mut_ptr().add(__idx)
    });
    q.x = 0;
    assert!(
        (((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<Point> = &mut h.points;
            __v.as_mut_ptr().add(__idx)
        })
        .x) == (7))
    );
    let mut grid: Vec<Vec<i32>> = Vec::new();
    {
        let __a1 = vec![0; 3_usize as usize];
        grid.push(__a1)
    };
    {
        let __a1 = vec![0; 3_usize as usize];
        grid.push(__a1)
    };
    (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<i32> = &mut (*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<Vec<i32>> = &mut grid;
            __v.as_mut_ptr().add(__idx)
        });
        __v.as_mut_ptr().add(__idx)
    }) = 8;
    assert!(
        ((*{
            let __idx = 2_usize as usize;
            let __v: &mut Vec<i32> = &mut (*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<Vec<i32>> = &mut grid;
                __v.as_mut_ptr().add(__idx)
            });
            __v.as_mut_ptr().add(__idx)
        }) == (8))
            && ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<i32> = &mut (*{
                    let __idx = 0_usize as usize;
                    let __v: &mut Vec<Vec<i32>> = &mut grid;
                    __v.as_mut_ptr().add(__idx)
                });
                __v.as_mut_ptr().add(__idx)
            }) == (0))
    );
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<Vec<i32>> = &mut grid;
            __v.as_mut_ptr().add(__idx)
        });
        __v.as_mut_ptr().add(__idx)
    }) = (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<i32> = &mut (*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<Vec<i32>> = &mut grid;
            __v.as_mut_ptr().add(__idx)
        });
        __v.as_mut_ptr().add(__idx)
    });
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut (*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<Vec<i32>> = &mut grid;
            __v.as_mut_ptr().add(__idx)
        });
        __v.as_mut_ptr().add(__idx)
    }) += (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<Vec<i32>> = &mut grid;
            __v.as_mut_ptr().add(__idx)
        });
        __v.as_mut_ptr().add(__idx)
    });
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<Vec<i32>> = &mut grid;
                __v.as_mut_ptr().add(__idx)
            });
            __v.as_mut_ptr().add(__idx)
        }) == (8))
            && ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut (*{
                    let __idx = 1_usize as usize;
                    let __v: &mut Vec<Vec<i32>> = &mut grid;
                    __v.as_mut_ptr().add(__idx)
                });
                __v.as_mut_ptr().add(__idx)
            }) == (8))
    );
    let mut a: Vec<i32> = Vec::from([4, 5, 6]);
    (*{
        let __idx = 1_usize as usize;
        let __v: &mut Vec<i32> = &mut a;
        __v.as_mut_ptr().add(__idx)
    }) = ((*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut a;
        __v.as_mut_ptr().add(__idx)
    }) + (*{
        let __idx = 2_usize as usize;
        let __v: &mut Vec<i32> = &mut a;
        __v.as_mut_ptr().add(__idx)
    }));
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<i32> = &mut a;
            __v.as_mut_ptr().add(__idx)
        }) == (10))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
