extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Step_0_ {
    pub value: i32,
}
impl Step_0_ {
    pub unsafe fn advance(&mut self) {
        self.value = -1_i32;
    }
    pub unsafe fn scaled(&self) -> i32 {
        return 0;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Step_2_ {
    pub value: i32,
}
impl Step_2_ {
    pub unsafe fn advance(&mut self) {
        self.value += 2;
    }
    pub unsafe fn scaled(&self) -> i32 {
        return ((self.value) * (2));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Step_3_ {
    pub value: i32,
}
impl Step_3_ {
    pub unsafe fn advance(&mut self) {
        self.value += 3;
    }
    pub unsafe fn scaled(&self) -> i32 {
        return ((self.value) * (3));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Step_neg1_ {
    pub value: i32,
}
impl Step_neg1_ {
    pub unsafe fn advance(&mut self) {
        self.value += -1;
    }
    pub unsafe fn scaled(&self) -> i32 {
        return ((self.value) * (-1));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Step_1_ {
    pub value: i32,
}
impl Step_1_ {
    pub unsafe fn advance(&mut self) {
        self.value += 1;
    }
    pub unsafe fn scaled(&self) -> i32 {
        return ((self.value) * (1));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Choice_true_ {}
impl Choice_true_ {
    pub unsafe fn pick(&self, mut a: i32, mut b: i32) -> i32 {
        return if true { a } else { b };
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Choice_false_ {}
impl Choice_false_ {
    pub unsafe fn pick(&self, mut a: i32, mut b: i32) -> i32 {
        return if false { a } else { b };
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Buffer_int__2_ {
    pub data: [i32; 2],
}
impl Buffer_int__2_ {
    pub unsafe fn size(&self) -> i32 {
        return 2;
    }
    pub unsafe fn sum(&self) -> i32 {
        let mut total: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (2)) {
            total += self.data[(i) as usize];
            i.prefix_inc();
        }
        return total;
    }
}
impl Default for Buffer_int__2_ {
    fn default() -> Self {
        Buffer_int__2_ { data: [0_i32; 2] }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Buffer_int__3_ {
    pub data: [i32; 3],
}
impl Buffer_int__3_ {
    pub unsafe fn size(&self) -> i32 {
        return 3;
    }
    pub unsafe fn sum(&self) -> i32 {
        let mut total: i32 = 0;
        let mut i: i32 = 0;
        'loop_: while ((i) < (3)) {
            total += self.data[(i) as usize];
            i.prefix_inc();
        }
        return total;
    }
}
impl Default for Buffer_int__3_ {
    fn default() -> Self {
        Buffer_int__3_ { data: [0_i32; 3] }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Mark__a___true_ {}
impl Mark__a___true_ {
    pub unsafe fn count(&self, mut s: *const libc::c_char) -> i32 {
        let mut n: i32 = 0;
        'loop_: while ((*s) != 0) {
            if (((*s) as i32) == (('a' as libc::c_char) as i32)) {
                n.prefix_inc();
                if !(true) {
                    break;
                }
            }
            s.prefix_inc();
        }
        return n;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Mark__a___false_ {}
impl Mark__a___false_ {
    pub unsafe fn count(&self, mut s: *const libc::c_char) -> i32 {
        let mut n: i32 = 0;
        'loop_: while ((*s) != 0) {
            if (((*s) as i32) == (('a' as libc::c_char) as i32)) {
                n.prefix_inc();
                if !(false) {
                    break;
                }
            }
            s.prefix_inc();
        }
        return n;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Mark__b___true_ {}
impl Mark__b___true_ {
    pub unsafe fn count(&self, mut s: *const libc::c_char) -> i32 {
        let mut n: i32 = 0;
        'loop_: while ((*s) != 0) {
            if (((*s) as i32) == (('b' as libc::c_char) as i32)) {
                n.prefix_inc();
                if !(true) {
                    break;
                }
            }
            s.prefix_inc();
        }
        return n;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Range_0__10_ {}
impl Range_0__10_ {
    pub unsafe fn contains(&self, mut x: i32) -> bool {
        return ((x) >= (0)) && ((x) <= (10));
    }
    pub unsafe fn width(&self) -> i32 {
        return ((10) - (0));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Range_5__6_ {}
impl Range_5__6_ {
    pub unsafe fn contains(&self, mut x: i32) -> bool {
        return ((x) >= (5)) && ((x) <= (6));
    }
    pub unsafe fn width(&self) -> i32 {
        return ((6) - (5));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Range_neg3__0_ {}
impl Range_neg3__0_ {
    pub unsafe fn contains(&self, mut x: i32) -> bool {
        return ((x) >= (-3)) && ((x) <= (0));
    }
    pub unsafe fn width(&self) -> i32 {
        return ((0) - (-3));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Mix___xc8___neg3__10000000000_ {}
impl Mix___xc8___neg3__10000000000_ {
    pub unsafe fn total(&self) -> i64 {
        return (((((b'\xc8' as u8) as i32) + (-3_i16 as i32)) as i64) + (10000000000_i64));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Mix___x01___1__1_ {}
impl Mix___x01___1__1_ {
    pub unsafe fn total(&self) -> i64 {
        return ((((('\x01' as u8) as i32) + (1_i16 as i32)) as i64) + (1_i64));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s2: Step_2_ = Step_2_ { value: 1 };
    let mut s3: Step_3_ = Step_3_ { value: 1 };
    (unsafe { Step_2_::advance(&mut s2) });
    (unsafe { Step_3_::advance(&mut s3) });
    assert!(((s2.value) == (3)));
    assert!(((s3.value) == (4)));
    assert!(((unsafe { Step_2_::scaled(&s2,) }) == (6)));
    assert!(((unsafe { Step_3_::scaled(&s3,) }) == (12)));
    let mut sn: Step_neg1_ = Step_neg1_ { value: 5 };
    let mut sp: Step_1_ = Step_1_ { value: 5 };
    (unsafe { Step_neg1_::advance(&mut sn) });
    (unsafe { Step_1_::advance(&mut sp) });
    assert!(((sn.value) == (4)));
    assert!(((sp.value) == (6)));
    assert!(((unsafe { Step_neg1_::scaled(&sn,) }) == (-4_i32)));
    assert!(((unsafe { Step_1_::scaled(&sp,) }) == (6)));
    let mut s0: Step_0_ = Step_0_ { value: 5 };
    (unsafe { Step_0_::advance(&mut s0) });
    assert!(((s0.value) == (-1_i32)));
    assert!(((unsafe { Step_0_::scaled(&s0,) }) == (0)));
    let mut yes: Choice_true_ = <Choice_true_>::default();
    let mut no: Choice_false_ = <Choice_false_>::default();
    assert!(((unsafe { Choice_true_::pick(&yes, 1, 2,) }) == (1)));
    assert!(((unsafe { Choice_false_::pick(&no, 1, 2,) }) == (2)));
    let mut b2: Buffer_int__2_ = Buffer_int__2_ { data: [1, 2] };
    let mut b3: Buffer_int__3_ = Buffer_int__3_ { data: [1, 2, 3] };
    assert!(
        ((unsafe { Buffer_int__2_::size(&b2,) }) == (2))
            && ((unsafe { Buffer_int__2_::sum(&b2,) }) == (3))
    );
    assert!(
        ((unsafe { Buffer_int__3_::size(&b3,) }) == (3))
            && ((unsafe { Buffer_int__3_::sum(&b3,) }) == (6))
    );
    let mut v2: Vec<Step_2_> = Vec::new();
    let mut v3: Vec<Step_3_> = Vec::new();
    v2.push(Step_2_ { value: 0 });
    v3.push(Step_3_ { value: 0 });
    (unsafe { Step_2_::advance(&mut v2[(0_usize)]) });
    (unsafe { Step_3_::advance(&mut v3[(0_usize)]) });
    assert!(((v2.len()) == (1_usize)) && ((v2[(0_usize)].value) == (2)));
    assert!(((v3.len()) == (1_usize)) && ((v3[(0_usize)].value) == (3)));
    let mut a2: Vec<Step_2_> = vec![Step_2_ { value: 1 }, Step_2_ { value: 2 }];
    let mut a3: Vec<Step_3_> = vec![
        Step_3_ { value: 1 },
        Step_3_ { value: 2 },
        Step_3_ { value: 3 },
    ];
    'loop_: for s in 0..(a2.len()) {
        let mut s = a2.as_mut_ptr().add(s);
        (unsafe { Step_2_::advance(&mut (*s)) });
    }
    'loop_: for s in 0..(a3.len()) {
        let mut s = a3.as_mut_ptr().add(s);
        (unsafe { Step_3_::advance(&mut (*s)) });
    }
    assert!(((a2.len()) == (2_usize)) && ((unsafe { Step_2_::scaled(&a2[(1_usize)],) }) == (8)));
    assert!(((a3.len()) == (3_usize)) && ((unsafe { Step_3_::scaled(&a3[(2_usize)],) }) == (18)));
    let mut vb2: Vec<Buffer_int__2_> = Vec::new();
    let mut vb3: Vec<Buffer_int__3_> = Vec::new();
    {
        let a0_clone = b2.clone();
        vb2.push(a0_clone)
    };
    {
        let a0_clone = b3.clone();
        vb3.push(a0_clone)
    };
    assert!(
        ((unsafe { Buffer_int__2_::size(&vb2[(0_usize)],) }) == (2))
            && ((unsafe { Buffer_int__2_::sum(&vb2[(0_usize)],) }) == (3))
    );
    assert!(
        ((unsafe { Buffer_int__3_::size(&vb3[(0_usize)],) }) == (3))
            && ((unsafe { Buffer_int__3_::sum(&vb3[(0_usize)],) }) == (6))
    );
    let mut ayes: Vec<Choice_true_> = vec![<Choice_true_>::default()];
    let mut ano: Vec<Choice_false_> = vec![<Choice_false_>::default()];
    assert!(((unsafe { Choice_true_::pick(&ayes[(0_usize)], 1, 2,) }) == (1)));
    assert!(((unsafe { Choice_false_::pick(&ano[(0_usize)], 1, 2,) }) == (2)));
    let mut all_a: Mark__a___true_ = <Mark__a___true_>::default();
    let mut first_a: Mark__a___false_ = <Mark__a___false_>::default();
    let mut all_b: Mark__b___true_ = <Mark__b___true_>::default();
    assert!(((unsafe { Mark__a___true_::count(&all_a, c"banana".as_ptr(),) }) == (3)));
    assert!(((unsafe { Mark__a___false_::count(&first_a, c"banana".as_ptr(),) }) == (1)));
    assert!(((unsafe { Mark__b___true_::count(&all_b, c"banana".as_ptr(),) }) == (1)));
    let mut wide: Range_0__10_ = <Range_0__10_>::default();
    let mut narrow: Range_5__6_ = <Range_5__6_>::default();
    let mut neg: Range_neg3__0_ = <Range_neg3__0_>::default();
    assert!(
        (unsafe { Range_0__10_::contains(&wide, 7,) })
            && (!(unsafe { Range_5__6_::contains(&narrow, 7,) }))
    );
    assert!(
        ((unsafe { Range_0__10_::width(&wide,) }) == (10))
            && ((unsafe { Range_5__6_::width(&narrow,) }) == (1))
    );
    assert!(
        (unsafe { Range_neg3__0_::contains(&neg, -2_i32,) })
            && ((unsafe { Range_neg3__0_::width(&neg,) }) == (3))
    );
    let mut big: Mix___xc8___neg3__10000000000_ = <Mix___xc8___neg3__10000000000_>::default();
    let mut small: Mix___x01___1__1_ = <Mix___x01___1__1_>::default();
    assert!(((unsafe { Mix___xc8___neg3__10000000000_::total(&big,) }) == (10000000197_i64)));
    assert!(((unsafe { Mix___x01___1__1_::total(&small,) }) == (3_i64)));
    let mut vwide: Vec<Range_0__10_> = vec![wide];
    let mut vnarrow: Vec<Range_5__6_> = vec![narrow];
    assert!(
        (unsafe { Range_0__10_::contains(&vwide[(0_usize)], 7,) })
            && (!(unsafe { Range_5__6_::contains(&vnarrow[(0_usize)], 7,) }))
    );
    let mut amarks: Vec<Mark__a___true_> =
        vec![<Mark__a___true_>::default(), <Mark__a___true_>::default()];
    let mut afirst: Vec<Mark__a___false_> =
        vec![<Mark__a___false_>::default(), <Mark__a___false_>::default()];
    assert!(((unsafe { Mark__a___true_::count(&amarks[(1_usize)], c"aaa".as_ptr(),) }) == (3)));
    assert!(((unsafe { Mark__a___false_::count(&afirst[(1_usize)], c"aaa".as_ptr(),) }) == (1)));
    let mut vbig: Vec<Mix___xc8___neg3__10000000000_> = vec![big, big];
    assert!(
        ((vbig.len()) == (2_usize))
            && ((unsafe { Mix___xc8___neg3__10000000000_::total(&vbig[(1_usize)],) })
                == (10000000197_i64))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
