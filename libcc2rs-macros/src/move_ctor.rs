// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn expand(input: TokenStream, is_unsafe: bool) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let body = if is_unsafe {
        quote! {{
            let mut __v: Self = ::std::mem::MaybeUninit::uninit().assume_init();
            Self::move_from(&raw mut __v, src);
            __v
        }}
    } else {
        quote! {{
            use ::libcc2rs::AsPointer as _;
            let __v = ::std::rc::Rc::new(::std::cell::RefCell::new(
                <Self as ::libcc2rs::Uninit>::uninit(),
            ));
            Self::move_from(__v.as_pointer(), src);
            ::std::rc::Rc::try_unwrap(__v).ok().unwrap().into_inner()
        }}
    };
    if is_unsafe {
        quote! {
            impl #impl_generics ::libcc2rs::MoveCtorUnsafe for #name #ty_generics #where_clause {
                unsafe fn move_ctor(src: *mut Self) -> Self {
                    unsafe { #body }
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics ::libcc2rs::MoveCtor for #name #ty_generics #where_clause {
                fn move_ctor(src: ::libcc2rs::Ptr<Self>) -> Self {
                    #body
                }
            }
        }
    }
    .into()
}
