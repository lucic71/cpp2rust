// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Data, DeriveInput, Error, Expr, Field, Ident, Lit, LitInt, Result, Token, Type, braced,
    parse_macro_input,
};

// Describes one storage array of a record and the bit-fields packed into it:
//
//     __bits_0 { a: u32 @ 0..1 unsigned, b: u32 @ 1..4 unsigned }
//
// The bit range is relative to the start of the storage array, and the type is
// the one the field was declared with, so that reads and writes match what the
// surrounding translated code expects. The `signed`/`unsigned` marker says how
// the stored bits are to be interpreted, which the type alone does not: a 3-bit
// unsigned field declared `unsigned` reads back as u32.
struct Spec {
    runs: Vec<Run>,
}

struct Run {
    storage: Ident,
    fields: Vec<BitField>,
}

struct BitField {
    name: Ident,
    ty: Type,
    lo: u64,
    hi: u64,
    signed: bool,
}

impl BitField {
    fn width(&self) -> u32 {
        (self.hi - self.lo) as u32
    }

    fn is_bool(&self) -> bool {
        matches!(&self.ty, Type::Path(path) if path.path.is_ident("bool"))
    }
}

impl Parse for Spec {
    fn parse(input: ParseStream) -> Result<Self> {
        let runs = Punctuated::<Run, Token![,]>::parse_terminated(input)?;
        Ok(Spec {
            runs: runs.into_iter().collect(),
        })
    }
}

impl Parse for Run {
    fn parse(input: ParseStream) -> Result<Self> {
        let storage: Ident = input.parse()?;
        let body;
        braced!(body in input);
        let fields = Punctuated::<BitField, Token![,]>::parse_terminated(&body)?;
        Ok(Run {
            storage,
            fields: fields.into_iter().collect(),
        })
    }
}

impl Parse for BitField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![@]>()?;
        let lo: LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let hi: LitInt = input.parse()?;
        let sign: Ident = input.parse()?;
        let signed = match sign.to_string().as_str() {
            "signed" => true,
            "unsigned" => false,
            _ => return Err(Error::new(sign.span(), "expected `signed` or `unsigned`")),
        };
        let lo = lo.base10_parse::<u64>()?;
        let hi = hi.base10_parse::<u64>()?;
        if hi <= lo {
            return Err(Error::new(
                name.span(),
                "bit range must contain at least one bit",
            ));
        }
        if hi - lo > 64 {
            return Err(Error::new(name.span(), "bit-field wider than 64 bits"));
        }
        Ok(BitField {
            name,
            ty,
            lo,
            hi,
            signed,
        })
    }
}

// One byte of a bit-field: which storage byte it lives in, where the bits sit
// inside that byte, and where they sit inside the field's value.
#[derive(Debug, PartialEq, Eq)]
struct ByteSpan {
    byte: usize,
    shift_in_byte: u32,
    shift_in_value: u32,
    bits: u32,
}

impl ByteSpan {
    // Mask of the bits this span occupies inside its storage byte.
    fn byte_mask(&self) -> u8 {
        (((1u16 << self.bits) - 1) as u8) << self.shift_in_byte
    }

    // Mask of the bits this span contributes, once shifted down to bit 0.
    fn value_mask(&self) -> u64 {
        (1u64 << self.bits) - 1
    }
}

// Splits a bit range into per-byte pieces, little-endian, which is the byte
// order clang uses for bit-field allocation on every target we support.
fn byte_spans(lo: u64, hi: u64) -> Vec<ByteSpan> {
    let mut spans = Vec::new();
    let mut bit = lo;
    while bit < hi {
        let byte = (bit / 8) as usize;
        let end = std::cmp::min(hi, (byte as u64 + 1) * 8);
        spans.push(ByteSpan {
            byte,
            shift_in_byte: (bit % 8) as u32,
            shift_in_value: (bit - lo) as u32,
            bits: (end - bit) as u32,
        });
        bit = end;
    }
    spans
}

// Bit width and signedness of an accessor type, used to tell whether a range
// check can be skipped.
fn accessor_bits(ty: &Type) -> Option<(u32, bool)> {
    let Type::Path(path) = ty else {
        return None;
    };
    let ident = path.path.get_ident()?.to_string();
    let signed = ident.starts_with('i');
    let bits = ident[1..].parse::<u32>().ok()?;
    Some((bits, signed))
}

fn record_fields(item: &DeriveInput) -> Result<Vec<&Field>> {
    match &item.data {
        Data::Struct(data) => Ok(data.fields.iter().collect()),
        Data::Union(data) => Ok(data.fields.named.iter().collect()),
        Data::Enum(_) => Err(Error::new(
            item.ident.span(),
            "#[bitfields] applies to structs and unions",
        )),
    }
}

fn storage_len(item: &DeriveInput, storage: &Ident) -> Result<u64> {
    for field in record_fields(item)? {
        if field.ident.as_ref() != Some(storage) {
            continue;
        }
        let Type::Array(array) = &field.ty else {
            return Err(Error::new(
                field.ty.span(),
                "bit-field storage must be a `[u8; N]` field",
            ));
        };
        let Expr::Lit(lit) = &array.len else {
            return Err(Error::new(
                array.len.span(),
                "bit-field storage length must be a literal",
            ));
        };
        let Lit::Int(len) = &lit.lit else {
            return Err(Error::new(
                array.len.span(),
                "bit-field storage length must be an integer literal",
            ));
        };
        return len.base10_parse::<u64>();
    }
    Err(Error::new(
        storage.span(),
        format!("no field named `{storage}` in this record"),
    ))
}

// Rejects ranges that leave their storage array or collide with each other. A
// wrong offset is otherwise a silent miscompile.
fn check_run(item: &DeriveInput, run: &Run) -> Result<()> {
    let bits = storage_len(item, &run.storage)? * 8;
    let mut claimed = vec![None; bits as usize];
    for field in &run.fields {
        if field.hi > bits {
            return Err(Error::new(
                field.name.span(),
                format!(
                    "bit-field `{}` ends at bit {} but `{}` only holds {} bits",
                    field.name, field.hi, run.storage, bits
                ),
            ));
        }
        for bit in field.lo..field.hi {
            if let Some(other) = &claimed[bit as usize] {
                return Err(Error::new(
                    field.name.span(),
                    format!("bit-field `{}` overlaps `{other}` at bit {bit}", field.name),
                ));
            }
            claimed[bit as usize] = Some(field.name.clone());
        }
    }
    Ok(())
}

fn read_expr(storage: &Ident, field: &BitField) -> TokenStream2 {
    let parts = byte_spans(field.lo, field.hi).into_iter().map(|span| {
        let byte = Literal::usize_unsuffixed(span.byte);
        let shift_in_byte = Literal::u32_unsuffixed(span.shift_in_byte);
        let shift_in_value = Literal::u32_unsuffixed(span.shift_in_value);
        let value_mask = Literal::u64_unsuffixed(span.value_mask());
        quote! {
            ((((self.#storage[#byte] as u64) >> #shift_in_byte) & #value_mask) << #shift_in_value)
        }
    });
    quote! { ( #( #parts )|* ) }
}

fn getter_body(storage: &Ident, field: &BitField) -> TokenStream2 {
    let raw = read_expr(storage, field);
    let ty = &field.ty;
    if field.is_bool() {
        return quote! { #raw != 0 };
    }
    if field.signed {
        // Shift the sign bit up to bit 63 and back down arithmetically.
        let pad = Literal::u32_unsuffixed(64 - field.width());
        return quote! { (((#raw << #pad) as i64) >> #pad) as #ty };
    }
    quote! { #raw as #ty }
}

fn range_assert(field: &BitField) -> TokenStream2 {
    if field.is_bool() {
        return quote! {};
    }
    let width = field.width();
    let name = field.name.to_string();
    if let Some((bits, signed)) = accessor_bits(&field.ty)
        && width >= bits && signed == field.signed {
            return quote! {};
        }
    if field.signed {
        let (min, max) = if width == 64 {
            (i64::MIN, i64::MAX)
        } else {
            (-(1i64 << (width - 1)), (1i64 << (width - 1)) - 1)
        };
        let min = Literal::i64_unsuffixed(min);
        let max = Literal::i64_unsuffixed(max);
        let message = format!("bit-field {name} does not fit in {width} bits");
        return quote! {
            assert!(v >= #min && v <= #max, #message);
        };
    }
    let max = Literal::u64_unsuffixed(if width == 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    });
    let message = format!("bit-field {name} does not fit in {width} bits");
    quote! {
        #[allow(unused_comparisons)]
        {
            assert!(v >= 0 && v <= #max, #message);
        }
    }
}

fn setter_body(storage: &Ident, field: &BitField) -> TokenStream2 {
    let writes = byte_spans(field.lo, field.hi).into_iter().map(|span| {
        let byte = Literal::usize_unsuffixed(span.byte);
        let shift_in_byte = Literal::u32_unsuffixed(span.shift_in_byte);
        let shift_in_value = Literal::u32_unsuffixed(span.shift_in_value);
        let mask = Literal::u8_suffixed(span.byte_mask());
        quote! {
            self.#storage[#byte] = (self.#storage[#byte] & !#mask)
                | ((((__v >> #shift_in_value) as u8) << #shift_in_byte) & #mask);
        }
    });
    let guard = range_assert(field);
    quote! {
        #guard
        let __v = v as u64;
        #( #writes )*
    }
}

pub fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let spec = parse_macro_input!(args as Spec);
    let item = parse_macro_input!(item as DeriveInput);

    // Touching a union's storage is unsafe, even to read the bytes back.
    let unsafe_body = matches!(item.data, Data::Union(_));
    let wrap = |body: TokenStream2| {
        if unsafe_body {
            quote! { unsafe { #body } }
        } else {
            body
        }
    };

    let mut accessors = Vec::new();
    for run in &spec.runs {
        if let Err(error) = check_run(&item, run) {
            return error.to_compile_error().into();
        }
        for field in &run.fields {
            let name = &field.name;
            let setter = Ident::new(&format!("set_{name}"), name.span());
            let with = Ident::new(&format!("with_{name}"), name.span());
            let ty = &field.ty;
            let getter_body = wrap(getter_body(&run.storage, field));
            let setter_body = wrap(setter_body(&run.storage, field));
            accessors.push(quote! {
                #[inline]
                pub const fn #name(&self) -> #ty {
                    #getter_body
                }
                #[inline]
                pub const fn #setter(&mut self, v: #ty) {
                    #setter_body
                }
                #[inline]
                pub const fn #with(mut self, v: #ty) -> Self {
                    self.#setter(v);
                    self
                }
            });
        }
    }

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    quote! {
        #item
        impl #impl_generics #name #ty_generics #where_clause {
            #( #accessors )*
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spans(lo: u64, hi: u64) -> Vec<(usize, u32, u32, u32, u8)> {
        byte_spans(lo, hi)
            .into_iter()
            .map(|span| {
                (
                    span.byte,
                    span.shift_in_byte,
                    span.shift_in_value,
                    span.bits,
                    span.byte_mask(),
                )
            })
            .collect()
    }

    #[test]
    fn single_bit() {
        assert_eq!(spans(0, 1), vec![(0, 0, 0, 1, 0x01)]);
        assert_eq!(spans(3, 4), vec![(0, 3, 0, 1, 0x08)]);
    }

    #[test]
    fn within_one_byte() {
        assert_eq!(spans(1, 4), vec![(0, 1, 0, 3, 0x0E)]);
        assert_eq!(spans(8, 16), vec![(1, 0, 0, 8, 0xFF)]);
    }

    #[test]
    fn crosses_bytes() {
        // 20 bits starting at bit 4: 4 bits in byte 0, 8 in byte 1, 8 in byte 2.
        assert_eq!(
            spans(4, 24),
            vec![(0, 4, 0, 4, 0xF0), (1, 0, 4, 8, 0xFF), (2, 0, 12, 8, 0xFF)]
        );
    }

    #[test]
    fn full_width() {
        assert_eq!(byte_spans(0, 64).len(), 8);
        assert_eq!(byte_spans(0, 64).iter().map(|s| s.bits).sum::<u32>(), 64);
    }

    #[test]
    fn value_masks_cover_the_field() {
        for (lo, hi) in [(0, 1), (1, 4), (4, 24), (7, 9), (0, 32), (13, 45)] {
            let mut seen = 0u64;
            for span in byte_spans(lo, hi) {
                seen |= span.value_mask() << span.shift_in_value;
            }
            let width = hi - lo;
            let expected = if width == 64 {
                u64::MAX
            } else {
                (1u64 << width) - 1
            };
            assert_eq!(seen, expected, "range {lo}..{hi}");
        }
    }
}
