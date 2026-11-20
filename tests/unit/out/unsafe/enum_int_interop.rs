extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Color = u32;
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
pub type Option = u32;
pub const Option_OPT_NONE: Option = 0;
pub const Option_OPT_A: Option = 10;
pub const Option_OPT_B: Option = 20;
pub const Option_OPT_C: Option = 30;
pub type Tag = u32;
pub const Tag_TAG_ZERO: Tag = 0;
pub const Tag_TAG_ONE: Tag = 1;
pub const Tag_TAG_TWO: Tag = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Entry {
    pub name: *const libc::c_char,
    pub color: Color,
    pub opt: Option,
}
impl Entry {
    pub unsafe fn new_1(this: *mut Entry) {}
    pub unsafe fn copy_from(this: *mut Entry, _a0: *const Entry) {
        (&raw mut (*this).name).write((*_a0).name);
        (&raw mut (*this).color).write((*_a0).color);
        (&raw mut (*this).opt).write((*_a0).opt);
    }
    pub unsafe fn move_from(this: *mut Entry, _a0: *mut Entry) {
        (&raw mut (*this).name).write((*_a0).name);
        (&raw mut (*this).color).write((*_a0).color);
        (&raw mut (*this).opt).write((*_a0).opt);
    }
}
pub static mut global_color_0: libcc2rs::LazyStorage<Color> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(Color_GREEN) });
pub static mut global_opt_1: libcc2rs::LazyStorage<Option> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(Option_OPT_B) });
pub static mut global_tag_2: libcc2rs::LazyStorage<Tag> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(Tag_TAG_TWO) });
pub static mut entries_3: libcc2rs::LazyStorage<[Entry; 3]> =
    libcc2rs::LazyStorage::new(|__p| unsafe {
        {
            let __d = __p.cast::<Entry>();
            {
                let __d = __d.offset(0);
                (&raw mut (*__d).name).write(c"first".as_ptr());
                (&raw mut (*__d).color).write(Color_RED);
                (&raw mut (*__d).opt).write(Option_OPT_NONE);
            };
            {
                let __d = __d.offset(1);
                (&raw mut (*__d).name).write(c"second".as_ptr());
                (&raw mut (*__d).color).write(Color_GREEN);
                (&raw mut (*__d).opt).write(Option_OPT_A);
            };
            {
                let __d = __d.offset(2);
                (&raw mut (*__d).name).write(c"third".as_ptr());
                (&raw mut (*__d).color).write(Color_BLUE);
                (&raw mut (*__d).opt).write(Option_OPT_C);
            };
        }
    });
pub unsafe fn as_int_4(mut c: Color) -> i32 {
    return (c as i32);
}
pub unsafe fn classify_option_5(mut option: i32) -> i32 {
    'switch: {
        match { option } {
            __v if __v == (Option_OPT_NONE as i32) => {
                return -1_i32;
            }
            __v if __v == (Option_OPT_A as i32) => {
                return 1;
            }
            __v if __v == (Option_OPT_B as i32) => {
                return 2;
            }
            __v if __v == (Option_OPT_C as i32) => {
                return 3;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn make_color_6(mut n: i32) -> Color {
    return ((n) as Color);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Color = Color_RED;
    assert!(((c as i32) == (Color_RED as i32)));
    assert!(((c as i32) == (0)));
    assert!(((c as i32) != (1)));
    if ((c as i32) == (Color_GREEN as i32)) {
        return 1;
    }
    'switch: {
        match { (c as i32) } {
            __v if __v == 0 => {
                break 'switch;
            }
            __v if __v == 1 => {
                return 1;
            }
            __v if __v == 2 => {
                return 2;
            }
            _ => {
                return 99;
            }
        }
    };
    let mut x: i32 = (c as i32);
    assert!(((x) == (0)));
    let mut y: i32 = ((c as i32) + (1));
    assert!(((y) == (1)));
    c = ((2) as Color);
    assert!(((c as i32) == (Color_BLUE as i32)));
    assert!(((c as i32) == (2)));
    c = (unsafe { make_color_6(1) });
    assert!(((c as i32) == (Color_GREEN as i32)));
    let mut cmp: Color = (((c as i32) + (1)) as Color);
    assert!(((cmp as i32) == (Color_BLUE as i32)));
    let mut o: Option = Option_OPT_A;
    assert!(((o as i32) == (Option_OPT_A as i32)));
    assert!(((o as i32) == (10)));
    let mut oi: i32 = (o as i32);
    assert!(((oi) == (10)));
    o = ((20) as Option);
    assert!(((o as i32) == (Option_OPT_B as i32)));
    let mut rc: i32 = (unsafe { classify_option_5((o as i32)) });
    assert!(((rc) == (2)));
    rc = (unsafe { classify_option_5(20) });
    assert!(((rc) == (2)));
    rc = (unsafe { classify_option_5((Option_OPT_C as i32)) });
    assert!(((rc) == (3)));
    let mut t: Tag = Tag_TAG_ONE;
    assert!(((t as i32) == (1)));
    assert!(((t as i32) == (Tag_TAG_ONE as i32)));
    let mut ti: i32 = (t as i32);
    assert!(((ti) == (1)));
    t = ((2) as Tag);
    assert!(((t as i32) == (Tag_TAG_TWO as i32)));
    'switch: {
        match { (t as i32) } {
            __v if __v == (Tag_TAG_ZERO as i32) => {
                return 90;
            }
            __v if __v == 1 => {
                return 91;
            }
            __v if __v == 2 => {
                break 'switch;
            }
            _ => {}
        }
    };
    let mut extra: i32 = (((Color_RED as i32) + (Color_GREEN as i32)) + (Color_BLUE as i32));
    assert!(((extra) == (((0) + (1)) + (2))));
    assert!((((*(*&raw const global_color_0).get()) as i32) == (Color_GREEN as i32)));
    assert!((((*(*&raw const global_opt_1).get()) as i32) == (Option_OPT_B as i32)));
    assert!((((*(*&raw const global_tag_2).get()) as i32) == (Tag_TAG_TWO as i32)));
    assert!((((*(*&raw const entries_3).get())[(0) as usize].color as i32) == (Color_RED as i32)));
    assert!(
        (((*(*&raw const entries_3).get())[(0) as usize].opt as i32) == (Option_OPT_NONE as i32))
    );
    assert!(
        (((*(*&raw const entries_3).get())[(1) as usize].color as i32) == (Color_GREEN as i32))
    );
    assert!((((*(*&raw const entries_3).get())[(1) as usize].opt as i32) == (Option_OPT_A as i32)));
    assert!((((*(*&raw const entries_3).get())[(2) as usize].color as i32) == (Color_BLUE as i32)));
    assert!((((*(*&raw const entries_3).get())[(2) as usize].opt as i32) == (Option_OPT_C as i32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const global_color_0).get();
    (*&raw const global_opt_1).get();
    (*&raw const global_tag_2).get();
    (*&raw const entries_3).get();
}
