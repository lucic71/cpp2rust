extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut end: *mut libc::c_char = std::ptr::null_mut();
    let mut s: *const libc::c_char = std::ptr::null();
    s = (c"42".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 10)) == (42_i64))
            as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (2_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"  -17abc".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 10)) == (-17_i32 as i64))
            as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (5_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"0xff".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 16)) == (255_i64))
            as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (4_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 0)) == (255_i64))
            as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (4_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"0755".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 0)) == (493_i64))
            as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (4_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"0x".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 16)) == (0_i64)) as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (1_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"9223372036854775808".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 10))
            == (9223372036854775807_i64)) as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (19_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"-9223372036854775809".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 10))
            == ((-9223372036854775807_i64) - (1_i64))) as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (20_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"junk".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 10)) == (0_i64)) as i32)
            != 0)
            && ((((end) == (s as *mut libc::c_char)) as i32) != 0)) as i32)
            != 0)
    );
    s = (c"z".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtoll(s, (&raw mut end as *mut *mut libc::c_char), 36)) == (35_i64))
            as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (1_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strtoll(
            (c"55".as_ptr().cast_mut()).cast_const(),
            std::ptr::null_mut(),
            10
        )) == (55_i64)) as i32)
            != 0)
    );
    s = (c"3.14".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtod(s, (&raw mut end as *mut *mut libc::c_char)))
            == (3.1400000000000001E+0)) as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (4_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"  -2.5e3xyz".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtod(s, (&raw mut end as *mut *mut libc::c_char))) == (-2.5E+3)) as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (8_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"1.e5".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtod(s, (&raw mut end as *mut *mut libc::c_char))) == (1.0E+5)) as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (4_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c".5".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtod(s, (&raw mut end as *mut *mut libc::c_char))) == (5.0E-1)) as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (2_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"1e".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtod(s, (&raw mut end as *mut *mut libc::c_char))) == (1.0E+0)) as i32)
            != 0)
            && (((((((end as usize) - (s as usize)) / ::std::mem::size_of::<libc::c_char>())
                as i64)
                == (1_i64)) as i32)
                != 0)) as i32)
            != 0)
    );
    s = (c"junk".as_ptr().cast_mut()).cast_const();
    assert!(
        (((((((libc::strtod(s, (&raw mut end as *mut *mut libc::c_char))) == (0.0E+0)) as i32)
            != 0)
            && ((((end) == (s as *mut libc::c_char)) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strtod(
            (c"+0.375e-1".as_ptr().cast_mut()).cast_const(),
            std::ptr::null_mut()
        )) == (3.7499999999999999E-2)) as i32)
            != 0)
    );
    return 0;
}
