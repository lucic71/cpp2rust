extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Degrees {
    pub value: i32,
}
impl Degrees {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut input: [i32; 3] = [1, 2, 3];
    let mut output: [i32; 3] = [0_i32; 3];
    {
        let mut __first = input.as_mut_ptr();
        let __last = input.as_mut_ptr().offset((3) as isize);
        let mut __out = output.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut widened: Vec<i64> = vec![0_i64; 3_usize as usize];
    {
        let mut __first = input.as_mut_ptr();
        let __last = input.as_mut_ptr().offset((3) as isize);
        let mut __out = widened.begin();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as i64);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    assert!(
        ((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i64> = &mut widened;
            __v.as_mut_ptr().add(__idx)
        }) == (1_i64))
            && ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<i64> = &mut widened;
                __v.as_mut_ptr().add(__idx)
            }) == (3_i64))
    );
    let mut constructed: Vec<Degrees> = vec![Degrees::new({ 0 },); 3_usize as usize];
    {
        let mut __first = input.as_mut_ptr();
        let __last = input.as_mut_ptr().offset((3) as isize);
        let mut __out = constructed.begin();
        while __first != __last {
            *__out.iter_deref() = <Degrees as CppFromUnsafe<i32>>::cpp_from(__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    assert!(
        (((*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<Degrees> = &mut constructed;
            __v.as_mut_ptr().add(__idx)
        })
        .value)
            == (1))
            && (((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<Degrees> = &mut constructed;
                __v.as_mut_ptr().add(__idx)
            })
            .value)
                == (3))
    );
    let mut wide: [i64; 2] = [7_i64, 8_i64];
    let mut floating: Vec<f64> = vec![0_f64; 2_usize as usize];
    {
        let mut __first = wide.as_mut_ptr();
        let __last = wide.as_mut_ptr().offset((2) as isize);
        let mut __out = floating.begin();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as f64);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    assert!(
        ((*{
            let __idx = 1_usize as usize;
            let __v: &mut Vec<f64> = &mut floating;
            __v.as_mut_ptr().add(__idx)
        }) == (8_f64))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
