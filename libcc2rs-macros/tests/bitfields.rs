// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs_macros::bitfields;

// struct flags { unsigned char tag; unsigned a:1; unsigned b:3; int x; unsigned c:1; };
#[repr(C, align(4))]
#[derive(Copy, Clone, Default)]
#[bitfields(__bits_0 { a: u32 @ 0..1 unsigned, b: u32 @ 1..4 unsigned },
            __bits_1 { c: u32 @ 0..1 unsigned })]
pub struct flags {
    pub tag: u8,
    pub __bits_0: [u8; 1],
    pub x: i32,
    pub __bits_1: [u8; 1],
}

// struct mixed_sign { int s:3; unsigned u:5; unsigned wide:12; };
#[repr(C, align(4))]
#[derive(Copy, Clone, Default)]
#[bitfields(__bits_0 { s: i32 @ 0..3 signed, u: u32 @ 3..8 unsigned, wide: u32 @ 8..20 unsigned })]
pub struct mixed_sign {
    pub __bits_0: [u8; 3],
}

// struct packed_flags { unsigned a:1; unsigned b:3; unsigned wide:20; int sgn:4; unsigned tail; };
#[repr(C, align(4))]
#[derive(Copy, Clone, Default)]
#[bitfields(__bits_0 { a: u32 @ 0..1 unsigned, b: u32 @ 1..4 unsigned,
                       wide: u32 @ 4..24 unsigned, sgn: i32 @ 24..28 signed })]
pub struct packed_flags {
    pub __bits_0: [u8; 4],
    pub tail: u32,
}

// struct wide_bits { unsigned long long lo:40; };
#[repr(C, align(8))]
#[derive(Copy, Clone, Default)]
#[bitfields(__bits_0 { lo: u64 @ 0..40 unsigned })]
pub struct wide_bits {
    pub __bits_0: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
#[bitfields(__bits_0 { flag: bool @ 2..3 unsigned })]
pub struct bool_bits {
    pub __bits_0: [u8; 1],
}

// union bits { unsigned a:3; int whole; };
#[repr(C, align(4))]
#[bitfields(__bits_0 { a: u32 @ 0..3 unsigned })]
pub union bits {
    pub __bits_0: [u8; 1],
    pub whole: i32,
}

#[test]
fn union_storage() {
    let mut v = bits { whole: 0 }.with_a(5);
    assert_eq!(v.a(), 5);
    v.set_a(2);
    assert_eq!(unsafe { v.whole }, 2);
}

#[test]
fn layout_matches_c() {
    assert_eq!(std::mem::size_of::<flags>(), 12);
    assert_eq!(std::mem::size_of::<mixed_sign>(), 4);
    assert_eq!(std::mem::size_of::<packed_flags>(), 8);
    assert_eq!(std::mem::offset_of!(flags, x), 4);
}

#[test]
fn writes_land_in_the_bytes_c_uses() {
    let mut f = flags::default();
    f.set_b(7);
    let raw: [u8; 12] = unsafe { std::mem::transmute(f) };
    assert_eq!(raw[0], 0x00);
    assert_eq!(raw[1], 0x0E);
}

#[test]
fn fields_do_not_disturb_each_other() {
    let mut f = flags::default();
    f.set_a(1);
    f.set_b(5);
    f.tag = 0xFF;
    f.x = -3;
    f.set_c(1);
    assert_eq!((f.a(), f.b(), f.c(), f.tag, f.x), (1, 5, 1, 0xFF, -3));
    f.set_b(2);
    assert_eq!((f.a(), f.b(), f.c()), (1, 2, 1));
}

#[test]
fn signed_fields_round_trip() {
    let mut m = mixed_sign::default();
    m.set_s(-4);
    assert_eq!(m.s(), -4);
    m.set_s(3);
    m.set_u(31);
    m.set_wide(0xABC);
    assert_eq!((m.s(), m.u(), m.wide()), (3, 31, 0xABC));
    m.set_s(-1);
    assert_eq!((m.s(), m.u(), m.wide()), (-1, 31, 0xABC));
}

#[test]
fn fields_crossing_byte_boundaries() {
    let v = packed_flags::default()
        .with_a(1)
        .with_b(5)
        .with_wide(0xABCDE)
        .with_sgn(-3);
    assert_eq!(v.__bits_0, [0xEB, 0xCD, 0xAB, 0x0D]);
    assert_eq!((v.a(), v.b(), v.wide(), v.sgn()), (1, 5, 0xABCDE, -3));

    let w = packed_flags {
        __bits_0: [0x3C, 0x12, 0x00, 0x0F],
        tail: 0,
    };
    assert_eq!((w.a(), w.b(), w.wide(), w.sgn()), (0, 6, 0x00123, -1));
}

#[test]
fn fields_wider_than_int() {
    let mut v = wide_bits::default();
    v.set_lo(0xAB_CDEF_1234);
    assert_eq!(v.lo(), 0xAB_CDEF_1234);
    assert_eq!(v.__bits_0, [0x34, 0x12, 0xEF, 0xCD, 0xAB]);
}

#[test]
fn boolean_fields() {
    let mut v = bool_bits::default();
    assert!(!v.flag());
    v.set_flag(true);
    assert!(v.flag());
    assert_eq!(v.__bits_0, [0x04]);
    v.set_flag(false);
    assert_eq!(v.__bits_0, [0x00]);
}

#[test]
fn accessors_work_in_const_context() {
    const F: flags = flags {
        tag: 2,
        __bits_0: [0; 1],
        x: 7,
        __bits_1: [0; 1],
    }
    .with_a(1)
    .with_b(5);
    assert_eq!((F.tag, F.a(), F.b(), F.x, F.c()), (2, 1, 5, 7, 0));
}

#[test]
#[should_panic(expected = "bit-field b does not fit in 3 bits")]
fn unsigned_store_out_of_range_panics() {
    flags::default().set_b(8);
}

#[test]
#[should_panic(expected = "bit-field sgn does not fit in 4 bits")]
fn signed_store_out_of_range_panics() {
    packed_flags::default().set_sgn(-9);
}
