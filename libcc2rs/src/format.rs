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
                            NumericParam::Literal(n) if n >= 0 && n != i32::MAX => {
                                Some(n as usize)
                            }
                            _ => None,
                        };
                        let s = if let Some(n) = limit {
                            let p = *v as *const u8;
                            let len = (0..n)
                                .position(|i| unsafe { *p.add(i) } == 0)
                                .unwrap_or(n);
                            String::from_utf8_lossy(unsafe {
                                std::slice::from_raw_parts(p, len)
                            })
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
