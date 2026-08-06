// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn f1(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_alphanumeric()) as i32
}

fn f2(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_alphabetic()) as i32
}

fn f3(a0: i32) -> i32 {
    ((a0 == 0x20) || (a0 == 0x09)) as i32
}

fn f4(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_control()) as i32
}

fn f5(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_digit()) as i32
}

fn f6(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_graphic()) as i32
}

fn f7(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_lowercase()) as i32
}

fn f8(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_graphic() || __c == 0x20) as i32
}

fn f9(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_punctuation()) as i32
}

fn f10(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_whitespace() || __c == 0x0b) as i32
}

fn f11(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_uppercase()) as i32
}

fn f12(a0: i32) -> i32 {
    u8::try_from(a0).is_ok_and(|__c| __c.is_ascii_hexdigit()) as i32
}

fn f13(a0: i32) -> i32 {
    match u8::try_from(a0) {
        Ok(__c) => __c.to_ascii_lowercase() as i32,
        Err(_) => a0,
    }
}

fn f14(a0: i32) -> i32 {
    match u8::try_from(a0) {
        Ok(__c) => __c.to_ascii_uppercase() as i32,
        Err(_) => a0,
    }
}
