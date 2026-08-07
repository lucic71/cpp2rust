// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use sprintf::parser::{ConversionType, FormatElement, NumericParam, parse_format_string};
use sprintf::{Printf, vsprintfp};

use crate::va_args::{VaArg, VaArgGet};

fn expand_star_params(fmt: &str, va: &[VaArg]) -> (String, Vec<VaArg>) {
    let mut out = String::with_capacity(fmt.len());
    let mut args = Vec::with_capacity(va.len());
    let mut pos = 0;
    let mut chars = fmt.chars().peekable();
    while let Some(c) = chars.next() {
        out.push(c);
        if c != '%' {
            continue;
        }
        if chars.peek() == Some(&'%') {
            out.push(chars.next().unwrap());
            continue;
        }
        while matches!(chars.peek(), Some('#' | '0' | '-' | ' ' | '+')) {
            out.push(chars.next().unwrap());
        }
        if chars.peek() == Some(&'*') {
            chars.next();
            let w = i32::get(&va[pos]);
            pos += 1;
            if w < 0 {
                out.push('-');
            }
            out.push_str(&w.unsigned_abs().to_string());
        } else {
            while matches!(chars.peek(), Some('0'..='9')) {
                out.push(chars.next().unwrap());
            }
        }
        if chars.peek() == Some(&'.') {
            chars.next();
            if chars.peek() == Some(&'*') {
                chars.next();
                let p = i32::get(&va[pos]);
                pos += 1;
                if p >= 0 {
                    out.push('.');
                    out.push_str(&p.to_string());
                }
            } else {
                out.push('.');
                while matches!(chars.peek(), Some('0'..='9')) {
                    out.push(chars.next().unwrap());
                }
            }
        }
        args.push(va[pos].clone());
        pos += 1;
    }
    (out, args)
}

pub fn format_c(fmt: &str, va: &[VaArg]) -> String {
    let (fmt, va) = expand_star_params(fmt, va);
    let elements = match parse_format_string(&fmt) {
        Ok(elements) => elements,
        Err(e) => panic!("format_c: cannot parse {fmt:?}: {e:?}"),
    };
    let mut args: Vec<Box<dyn Printf>> = Vec::new();
    let mut pos = 0;
    for element in &elements {
        if let FormatElement::Format(spec) = element {
            if spec.conversion_type == ConversionType::PercentSign {
                continue;
            }
            let arg = &va[pos];
            args.push(match spec.conversion_type {
                ConversionType::DecInt => match arg {
                    VaArg::Int(v) => Box::new(*v),
                    VaArg::UInt(v) => Box::new(*v),
                    VaArg::Long(v) => Box::new(*v),
                    VaArg::ULong(v) => Box::new(*v),
                    _ => panic!("format_c: integer conversion expects an integer argument"),
                },
                ConversionType::OctInt
                | ConversionType::HexIntLower
                | ConversionType::HexIntUpper => match arg {
                    VaArg::Int(v) => Box::new(*v as u32),
                    VaArg::UInt(v) => Box::new(*v),
                    VaArg::Long(v) => Box::new(*v as u64),
                    VaArg::ULong(v) => Box::new(*v),
                    VaArg::Ptr(p) => Box::new(p.to_int()),
                    VaArg::RawPtr(v) => Box::new(*v as usize),
                    VaArg::Double(_) => {
                        panic!("format_c: integer conversion expects an integer argument")
                    }
                },
                ConversionType::Char => Box::new(i32::get(arg) as u8 as char),
                ConversionType::String => match arg {
                    VaArg::Ptr(v) => Box::new(v.reinterpret_cast::<u8>().to_rust_string()),
                    VaArg::RawPtr(v) => {
                        let limit = match spec.precision {
                            NumericParam::Literal(n) if n >= 0 && n != i32::MAX => Some(n as usize),
                            _ => None,
                        };
                        let s = if let Some(n) = limit {
                            let p = *v as *const u8;
                            let len = (0..n).position(|i| unsafe { *p.add(i) } == 0).unwrap_or(n);
                            String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(p, len) })
                                .into_owned()
                        } else {
                            unsafe { std::ffi::CStr::from_ptr(*v as *const std::ffi::c_char) }
                                .to_string_lossy()
                                .into_owned()
                        };
                        Box::new(s)
                    }
                    _ => panic!("format_c: %s expects a string argument"),
                },
                ConversionType::DecFloatLower
                | ConversionType::DecFloatUpper
                | ConversionType::SciFloatLower
                | ConversionType::SciFloatUpper
                | ConversionType::CompactFloatLower
                | ConversionType::CompactFloatUpper => Box::new(f64::get(arg)),
                ConversionType::PercentSign => panic!("format_c: %% consumes no argument"),
            });
            pos += 1;
        }
    }
    let refs: Vec<&dyn Printf> = args.iter().map(|arg| arg.as_ref()).collect();
    match vsprintfp(&elements, &refs) {
        Ok(s) => s,
        Err(e) => panic!("format_c: cannot format {fmt:?}: {e:?}"),
    }
}

fn scan_int(inp: &[u8], base: u32, unsigned: bool) -> Option<(i64, usize)> {
    let mut pos = 0;
    let mut negative = false;
    if pos < inp.len() && (inp[pos] == b'+' || inp[pos] == b'-') {
        negative = inp[pos] == b'-';
        pos += 1;
    }
    if base == 16 && (inp[pos..].starts_with(b"0x") || inp[pos..].starts_with(b"0X")) {
        pos += 2;
    }
    let start = pos;
    let mut value: i64 = 0;
    while pos < inp.len() {
        let Some(digit) = (inp[pos] as char).to_digit(base) else {
            break;
        };
        value = value.wrapping_mul(base as i64).wrapping_add(digit as i64);
        pos += 1;
    }
    if pos == start {
        return None;
    }
    if negative && !unsigned {
        value = value.wrapping_neg();
    }
    Some((value, pos))
}

fn scan_store(arg: &VaArg, value: i64, length: u8) {
    let VaArg::Ptr(p) = arg else {
        panic!("scan_c: output argument must be a pointer");
    };
    match length {
        1 => p.reinterpret_cast::<i16>().write(value as i16),
        8 => p.reinterpret_cast::<i64>().write(value),
        _ => p.reinterpret_cast::<i32>().write(value as i32),
    }
}

pub fn scan_c(input: &str, fmt: &str, va: &[VaArg]) -> i32 {
    let inp = input.as_bytes();
    let f = fmt.as_bytes();
    let mut ip = 0;
    let mut fp = 0;
    let mut matched = 0;
    let mut next_arg = 0;
    while fp < f.len() {
        let c = f[fp];
        if c.is_ascii_whitespace() {
            fp += 1;
            while ip < inp.len() && inp[ip].is_ascii_whitespace() {
                ip += 1;
            }
            continue;
        }
        if c != b'%' {
            if ip < inp.len() && inp[ip] == c {
                ip += 1;
                fp += 1;
                continue;
            }
            break;
        }
        fp += 1;
        if fp < f.len() && f[fp] == b'%' {
            if ip < inp.len() && inp[ip] == b'%' {
                ip += 1;
                fp += 1;
                continue;
            }
            break;
        }
        let mut width = 0_usize;
        while fp < f.len() && f[fp].is_ascii_digit() {
            width = width * 10 + (f[fp] - b'0') as usize;
            fp += 1;
        }
        let mut length = 4_u8;
        while fp < f.len() && (f[fp] == b'l' || f[fp] == b'h') {
            length = match (length, f[fp]) {
                (4, b'l') => 8,
                (4, b'h') => 1,
                (other, _) => other,
            };
            fp += 1;
        }
        if fp >= f.len() {
            break;
        }
        let conv = f[fp];
        fp += 1;
        if conv != b'c' {
            while ip < inp.len() && inp[ip].is_ascii_whitespace() {
                ip += 1;
            }
        }
        let end = match width {
            0 => inp.len(),
            w => inp.len().min(ip + w),
        };
        match conv {
            b'd' | b'i' | b'u' => {
                let Some((value, used)) = scan_int(&inp[ip..end], 10, conv == b'u') else {
                    break;
                };
                scan_store(&va[next_arg], value, length);
                ip += used;
            }
            b'x' | b'X' => {
                let Some((value, used)) = scan_int(&inp[ip..end], 16, true) else {
                    break;
                };
                scan_store(&va[next_arg], value, length);
                ip += used;
            }
            b'c' => {
                if ip >= inp.len() {
                    break;
                }
                let VaArg::Ptr(p) = &va[next_arg] else {
                    panic!("scan_c: output argument must be a pointer");
                };
                p.reinterpret_cast::<u8>().write(inp[ip]);
                ip += 1;
            }
            b's' => {
                let start = ip;
                let mut stop = ip;
                while stop < end && !inp[stop].is_ascii_whitespace() {
                    stop += 1;
                }
                if stop == start {
                    break;
                }
                let VaArg::Ptr(p) = &va[next_arg] else {
                    panic!("scan_c: output argument must be a pointer");
                };
                let n = stop - start;
                p.reinterpret_cast::<u8>().with_slice_mut(n + 1, |s| {
                    s[..n].copy_from_slice(&inp[start..stop]);
                    s[n] = 0;
                });
                ip = stop;
            }
            other => panic!("scan_c: unsupported conversion %{}", other as char),
        }
        matched += 1;
        next_arg += 1;
    }
    matched
}

use crate::rc::Ptr;

pub fn strtoll_refcount(a0: Ptr<u8>, a1: Ptr<Ptr<u8>>, a2: i32) -> i64 {
    let s = a0.to_rust_string();
    let b = s.as_bytes();
    let mut pos = 0;
    while pos < b.len() && b[pos].is_ascii_whitespace() {
        pos += 1;
    }
    let mut negative = false;
    if pos < b.len() && (b[pos] == b'+' || b[pos] == b'-') {
        negative = b[pos] == b'-';
        pos += 1;
    }
    let mut base = a2 as u32;
    let mut after_zero = None;
    if base == 0 {
        if b[pos..].starts_with(b"0x") || b[pos..].starts_with(b"0X") {
            base = 16;
        } else if pos < b.len() && b[pos] == b'0' {
            base = 8;
        } else {
            base = 10;
        }
    }
    if !(2..=36).contains(&base) {
        crate::cpp2rust_errno().write(::libc::EINVAL);
        if !a1.is_null() {
            a1.write(a0.clone());
        }
        return 0;
    }
    if base == 16 && (b[pos..].starts_with(b"0x") || b[pos..].starts_with(b"0X")) {
        after_zero = Some(pos + 1);
        pos += 2;
    }
    let digits_start = pos;
    while pos < b.len() && (b[pos] as char).is_digit(base) {
        pos += 1;
    }
    if pos == digits_start {
        match after_zero {
            Some(end) => {
                if !a1.is_null() {
                    a1.write(a0.clone().offset(end as isize));
                }
                return 0;
            }
            None => {
                if !a1.is_null() {
                    a1.write(a0.clone());
                }
                return 0;
            }
        }
    }
    if !a1.is_null() {
        a1.write(a0.clone().offset(pos as isize));
    }
    let num = match negative {
        true => format!("-{}", &s[digits_start..pos]),
        false => s[digits_start..pos].to_string(),
    };
    match i64::from_str_radix(&num, base) {
        Ok(value) => value,
        Err(e) => {
            crate::cpp2rust_errno().write(::libc::ERANGE);
            match e.kind() {
                std::num::IntErrorKind::NegOverflow => i64::MIN,
                _ => i64::MAX,
            }
        }
    }
}

pub fn strtod_refcount(a0: Ptr<u8>, a1: Ptr<Ptr<u8>>) -> f64 {
    let s = a0.to_rust_string();
    let b = s.as_bytes();
    let mut pos = 0;
    while pos < b.len() && b[pos].is_ascii_whitespace() {
        pos += 1;
    }
    let start = pos;
    if pos < b.len() && (b[pos] == b'+' || b[pos] == b'-') {
        pos += 1;
    }
    let int_start = pos;
    while pos < b.len() && b[pos].is_ascii_digit() {
        pos += 1;
    }
    let int_digits = pos - int_start;
    let mut frac_digits = 0;
    if pos < b.len() && b[pos] == b'.' {
        pos += 1;
        let frac_start = pos;
        while pos < b.len() && b[pos].is_ascii_digit() {
            pos += 1;
        }
        frac_digits = pos - frac_start;
    }
    if int_digits == 0 && frac_digits == 0 {
        if !a1.is_null() {
            a1.write(a0.clone());
        }
        return 0.0;
    }
    if pos < b.len() && (b[pos] == b'e' || b[pos] == b'E') {
        let mut exp_pos = pos + 1;
        if exp_pos < b.len() && (b[exp_pos] == b'+' || b[exp_pos] == b'-') {
            exp_pos += 1;
        }
        let exp_start = exp_pos;
        while exp_pos < b.len() && b[exp_pos].is_ascii_digit() {
            exp_pos += 1;
        }
        if exp_pos > exp_start {
            pos = exp_pos;
        }
    }
    if !a1.is_null() {
        a1.write(a0.clone().offset(pos as isize));
    }
    s[start..pos]
        .parse::<f64>()
        .expect("strtod: scanned prefix must parse")
}
