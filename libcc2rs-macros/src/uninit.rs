// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, Expr, Field, Fields, Index, Token, parse_macro_input};

// The value of `field` before its constructor runs. An array, whose length is
// not part of its type, has one array_len argument per dimension.
fn uninit(field: &Field) -> TokenStream2 {
    let lens = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("array_len"))
        .map(|attr| {
            attr.parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
                .unwrap()
        });
    let mut value = quote!(::libcc2rs::Uninit::uninit());
    for len in lens.iter().flatten().rev() {
        value = quote!((0..#len).map(|_| #value).collect());
    }
    value
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let Data::Struct(data) = &input.data else {
        panic!("derive(Uninit) is only supported on structs, `{name}` is not one");
    };
    let fields = match &data.fields {
        Fields::Named(fields) => {
            let idents = fields.named.iter().map(|field| &field.ident);
            let values = fields.named.iter().map(uninit);
            quote!({ #(#idents: #values,)* })
        }
        Fields::Unnamed(fields) => {
            let indices = (0..fields.unnamed.len()).map(Index::from);
            let values = fields.unnamed.iter().map(uninit);
            quote!({ #(#indices: #values,)* })
        }
        Fields::Unit => quote!({}),
    };
    quote! {
        impl #impl_generics ::libcc2rs::Uninit for #name #ty_generics #where_clause {
            fn uninit() -> Self {
                Self #fields
            }
        }
    }
    .into()
}
