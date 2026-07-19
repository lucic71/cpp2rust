extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_time_0() {
    let t1: Value<i64> = Rc::new(RefCell::new({
        let __out = Ptr::<i64>::null();
        match nix::time::clock_gettime(nix::time::ClockId::CLOCK_REALTIME) {
            Ok(__ts) => {
                let __s = __ts.tv_sec();
                if !__out.is_null() {
                    __out.write(__s);
                }
                __s
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    let t2: Value<i64> = Rc::new(RefCell::new(0_i64));
    let t3: Value<i64> = Rc::new(RefCell::new({
        let __out = (t2.as_pointer());
        match nix::time::clock_gettime(nix::time::ClockId::CLOCK_REALTIME) {
            Ok(__ts) => {
                let __s = __ts.tv_sec();
                if !__out.is_null() {
                    __out.write(__s);
                }
                __s
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    assert!(((((*t1.borrow()) > 1500000000_i64) as i32) != 0));
    assert!(((((*t2.borrow()) == (*t3.borrow())) as i32) != 0));
    assert!(((((*t3.borrow()) >= (*t1.borrow())) as i32) != 0));
}
pub fn print_tm_1(t: i64) {
    let t: Value<i64> = Rc::new(RefCell::new(t));
    let tm: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((!(({
            let __res = (tm.as_pointer()).clone();
            match jiff::Timestamp::from_second((t.as_pointer()).read()) {
                Ok(__ts) => {
                    let __dt = __ts.to_zoned(jiff::tz::TimeZone::UTC);
                    __res.with_mut(|__tm| *__tm = Tm::from_zoned(&__dt));
                    __res
                }
                Err(_) => {
                    libcc2rs::cpp2rust_errno().write(::libc::EOVERFLOW);
                    Ptr::null()
                }
            }
        })
        .is_null())) as i32)
            != 0)
    );
    println!(
        "{}-{}-{} {}:{}:{} wday={} yday={} {} gmtoff={} isdst={}",
        (*(*tm.borrow()).tm_year.borrow()),
        (*(*tm.borrow()).tm_mon.borrow()),
        (*(*tm.borrow()).tm_mday.borrow()),
        (*(*tm.borrow()).tm_hour.borrow()),
        (*(*tm.borrow()).tm_min.borrow()),
        (*(*tm.borrow()).tm_sec.borrow()),
        (*(*tm.borrow()).tm_wday.borrow()),
        (*(*tm.borrow()).tm_yday.borrow()),
        (*(*tm.borrow()).tm_zone.borrow()),
        (*(*tm.borrow()).tm_gmtoff.borrow()),
        (*(*tm.borrow()).tm_isdst.borrow())
    );
}
pub fn test_clock_gettime_2() {
    let ts: Value<libcc2rs::Timespec> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match nix::time::clock_gettime(nix::time::ClockId::CLOCK_REALTIME) {
            Ok(__ts) => {
                (ts.as_pointer()).with_mut(|__t| {
                    *__t.tv_sec.borrow_mut() = __ts.tv_sec() as i64;
                    *__t.tv_nsec.borrow_mut() = __ts.tv_nsec() as i64;
                });
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(((((*(*ts.borrow()).tv_sec.borrow()) > 1500000000_i64) as i32) != 0));
    assert!(
        (((((((*(*ts.borrow()).tv_nsec.borrow()) >= 0_i64) as i32) != 0)
            && ((((*(*ts.borrow()).tv_nsec.borrow()) < 1000000000_i64) as i32) != 0))
            as i32)
            != 0)
    );
    let mono: Value<libcc2rs::Timespec> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match nix::time::clock_gettime(nix::time::ClockId::CLOCK_MONOTONIC) {
            Ok(__ts) => {
                (mono.as_pointer()).with_mut(|__t| {
                    *__t.tv_sec.borrow_mut() = __ts.tv_sec() as i64;
                    *__t.tv_nsec.borrow_mut() = __ts.tv_nsec() as i64;
                });
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(((((*(*mono.borrow()).tv_sec.borrow()) >= 0_i64) as i32) != 0));
    assert!(
        (((((((*(*mono.borrow()).tv_nsec.borrow()) >= 0_i64) as i32) != 0)
            && ((((*(*mono.borrow()).tv_nsec.borrow()) < 1000000000_i64) as i32) != 0))
            as i32)
            != 0)
    );
}
pub fn test_gmtime_r_3() {
    ({ print_tm_1(0_i64) });
    ({ print_tm_1(1_i64) });
    ({ print_tm_1(86399_i64) });
    ({ print_tm_1(86400_i64) });
    ({ print_tm_1(951782400_i64) });
    ({ print_tm_1(951868799_i64) });
    ({ print_tm_1(1704067199_i64) });
    ({ print_tm_1(1704067200_i64) });
    ({ print_tm_1(1721126096_i64) });
    ({ print_tm_1(4102444800_i64) });
}
pub fn print_local_tm_4(t: i64) {
    let t: Value<i64> = Rc::new(RefCell::new(t));
    let tm: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((!(({
            let __res = (tm.as_pointer()).clone();
            match jiff::Timestamp::from_second((t.as_pointer()).read()) {
                Ok(__ts) => {
                    let __dt = __ts.to_zoned(jiff::tz::TimeZone::system());
                    let __info = __dt.time_zone().to_offset_info(__ts);
                    let __zone: Vec<u8> = __info.abbreviation().bytes().chain([0]).collect();
                    let __isdst = if __info.dst().is_dst() { 1 } else { 0 };
                    __res.with_mut(|__tm| {
                        *__tm = Tm::from_zoned(&__dt);
                        *__tm.tm_isdst.borrow_mut() = __isdst;
                        *__tm.tm_zone.borrow_mut() = Ptr::alloc_array(__zone.into_boxed_slice());
                    });
                    __res
                }
                Err(_) => {
                    libcc2rs::cpp2rust_errno().write(::libc::EOVERFLOW);
                    Ptr::null()
                }
            }
        })
        .is_null())) as i32)
            != 0)
    );
    println!(
        "{}-{}-{} {}:{}:{} wday={} yday={} {} gmtoff={} isdst={}",
        (*(*tm.borrow()).tm_year.borrow()),
        (*(*tm.borrow()).tm_mon.borrow()),
        (*(*tm.borrow()).tm_mday.borrow()),
        (*(*tm.borrow()).tm_hour.borrow()),
        (*(*tm.borrow()).tm_min.borrow()),
        (*(*tm.borrow()).tm_sec.borrow()),
        (*(*tm.borrow()).tm_wday.borrow()),
        (*(*tm.borrow()).tm_yday.borrow()),
        (*(*tm.borrow()).tm_zone.borrow()),
        (*(*tm.borrow()).tm_gmtoff.borrow()),
        (*(*tm.borrow()).tm_isdst.borrow())
    );
}
pub fn test_localtime_r_5() {
    ({ print_local_tm_4(0_i64) });
    ({ print_local_tm_4(951782400_i64) });
    ({ print_local_tm_4(1704067199_i64) });
    ({ print_local_tm_4(1721126096_i64) });
    ({ print_local_tm_4(1735689600_i64) });
}
pub fn test_strftime_6() {
    let t: Value<i64> = Rc::new(RefCell::new(1721126096_i64));
    let tm: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((!(({
            let __res = (tm.as_pointer()).clone();
            match jiff::Timestamp::from_second((t.as_pointer()).read()) {
                Ok(__ts) => {
                    let __dt = __ts.to_zoned(jiff::tz::TimeZone::UTC);
                    __res.with_mut(|__tm| *__tm = Tm::from_zoned(&__dt));
                    __res
                }
                Err(_) => {
                    libcc2rs::cpp2rust_errno().write(::libc::EOVERFLOW);
                    Ptr::null()
                }
            }
        })
        .is_null())) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
            let __text = match __dt {
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"%Y-%m-%d %H:%M:%S")
                        .to_rust_string()
                        .as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 64]>() {
                0
            } else {
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__text.len() + 1, |__s| {
                    __s[..__text.len()].copy_from_slice(__text.as_bytes());
                    __s[__text.len()] = 0;
                });
                __text.len()
            }
        } > 0_usize) as i32)
            != 0)
    );
    println!("{}", (buf.as_pointer() as Ptr::<u8>));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
            let __text = match __dt {
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"%a, %d %b %Y %T")
                        .to_rust_string()
                        .as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 64]>() {
                0
            } else {
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__text.len() + 1, |__s| {
                    __s[..__text.len()].copy_from_slice(__text.as_bytes());
                    __s[__text.len()] = 0;
                });
                __text.len()
            }
        } > 0_usize) as i32)
            != 0)
    );
    println!("{}", (buf.as_pointer() as Ptr::<u8>));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
            let __text = match __dt {
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"day %j 100%%")
                        .to_rust_string()
                        .as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 64]>() {
                0
            } else {
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__text.len() + 1, |__s| {
                    __s[..__text.len()].copy_from_slice(__text.as_bytes());
                    __s[__text.len()] = 0;
                });
                __text.len()
            }
        } > 0_usize) as i32)
            != 0)
    );
    println!("{}", (buf.as_pointer() as Ptr::<u8>));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
            let __text = match __dt {
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"%e").to_rust_string().as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 64]>() {
                0
            } else {
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__text.len() + 1, |__s| {
                    __s[..__text.len()].copy_from_slice(__text.as_bytes());
                    __s[__text.len()] = 0;
                });
                __text.len()
            }
        } > 0_usize) as i32)
            != 0)
    );
    println!("{}", (buf.as_pointer() as Ptr::<u8>));
    let small: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        ((({
            let __dt = (tm.as_pointer()).with(|__tm| __tm.to_civil());
            let __text = match __dt {
                Ok(__d) => jiff::fmt::strtime::format(
                    Ptr::from_string_literal(b"%Y-%m-%d")
                        .to_rust_string()
                        .as_str(),
                    __d,
                )
                .unwrap_or_default(),
                Err(_) => String::new(),
            };
            if __text.is_empty() || __text.len() + 1 > ::std::mem::size_of::<[u8; 4]>() {
                0
            } else {
                (small.as_pointer() as Ptr<u8>).with_slice_mut(__text.len() + 1, |__s| {
                    __s[..__text.len()].copy_from_slice(__text.as_bytes());
                    __s[__text.len()] = 0;
                });
                __text.len()
            }
        } == 0_usize) as i32)
            != 0)
    );
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_time_0() });
    ({ test_clock_gettime_2() });
    ({ test_gmtime_r_3() });
    ({ test_localtime_r_5() });
    ({ test_strftime_6() });
    return 0;
}
