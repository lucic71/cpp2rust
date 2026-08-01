// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Ptr};

#[derive(Default, Clone)]
pub struct Tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: Ptr<u8>,
}

impl Tm {
    pub fn from_zoned(dt: &jiff::Zoned) -> Self {
        let mut tm = Tm {
            tm_sec: dt.second() as i32,
            tm_min: dt.minute() as i32,
            tm_hour: dt.hour() as i32,
            tm_mday: dt.day() as i32,
            tm_mon: dt.month() as i32 - 1,
            tm_year: dt.year() as i32 - 1900,
            tm_wday: dt.weekday().to_sunday_zero_offset() as i32,
            tm_yday: dt.day_of_year() as i32 - 1,
            tm_isdst: 0,
            tm_gmtoff: dt.offset().seconds() as i64,
            ..Default::default()
        };
        #[cfg(target_os = "linux")]
        let zone: &'static [u8] = b"GMT";
        #[cfg(target_os = "macos")]
        let zone: &'static [u8] = b"UTC";
        tm.tm_zone = Ptr::from_string_literal(zone);
        tm
    }

    pub fn to_civil(&self) -> Result<jiff::civil::DateTime, jiff::Error> {
        jiff::civil::DateTime::new(
            (self.tm_year + 1900) as i16,
            (self.tm_mon + 1) as i8,
            self.tm_mday as i8,
            self.tm_hour as i8,
            self.tm_min as i8,
            self.tm_sec as i8,
            0,
        )
    }
}

impl ByteRepr for Tm {
    fn byte_size() -> usize {
        56
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.tm_sec.to_bytes(&mut buf[0..4]);
        self.tm_min.to_bytes(&mut buf[4..8]);
        self.tm_hour.to_bytes(&mut buf[8..12]);
        self.tm_mday.to_bytes(&mut buf[12..16]);
        self.tm_mon.to_bytes(&mut buf[16..20]);
        self.tm_year.to_bytes(&mut buf[20..24]);
        self.tm_wday.to_bytes(&mut buf[24..28]);
        self.tm_yday.to_bytes(&mut buf[28..32]);
        self.tm_isdst.to_bytes(&mut buf[32..36]);
        self.tm_gmtoff.to_bytes(&mut buf[40..48]);
        self.tm_zone.to_bytes(&mut buf[48..56]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tm_sec: i32::from_bytes(&buf[0..4]),
            tm_min: i32::from_bytes(&buf[4..8]),
            tm_hour: i32::from_bytes(&buf[8..12]),
            tm_mday: i32::from_bytes(&buf[12..16]),
            tm_mon: i32::from_bytes(&buf[16..20]),
            tm_year: i32::from_bytes(&buf[20..24]),
            tm_wday: i32::from_bytes(&buf[24..28]),
            tm_yday: i32::from_bytes(&buf[28..32]),
            tm_isdst: i32::from_bytes(&buf[32..36]),
            tm_gmtoff: i64::from_bytes(&buf[40..48]),
            tm_zone: <Ptr<u8>>::from_bytes(&buf[48..56]),
        }
    }
}

#[derive(Default, Clone)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

impl ByteRepr for Timeval {
    fn byte_size() -> usize {
        16
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.tv_sec.to_bytes(&mut buf[0..8]);
        self.tv_usec.to_bytes(&mut buf[8..16]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tv_sec: i64::from_bytes(&buf[0..8]),
            tv_usec: i64::from_bytes(&buf[8..16]),
        }
    }
}

#[derive(Default, Clone)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl ByteRepr for Timespec {
    fn byte_size() -> usize {
        16
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.tv_sec.to_bytes(&mut buf[0..8]);
        self.tv_nsec.to_bytes(&mut buf[8..16]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tv_sec: i64::from_bytes(&buf[0..8]),
            tv_nsec: i64::from_bytes(&buf[8..16]),
        }
    }
}

impl ByteRepr for ::libc::tm {}
impl ByteRepr for ::libc::timeval {}
impl ByteRepr for ::libc::timespec {}
