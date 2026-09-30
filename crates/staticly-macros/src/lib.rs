//! Macros for `staticly` hashing/lookup

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use std::collections::HashSet;
use syn::{
    Error, Lit, LitStr, Token, parse::Parse, parse2, punctuated::Punctuated,
};

const MAX_DEPTH: usize = 3;

#[proc_macro]
pub fn hash(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    hash_impl(input, 0)
}

fn hash_impl(input: TokenStream2, depth: usize) -> TokenStream {
    if depth > MAX_DEPTH {
        return Error::new(Span::call_site(), "hash recursion limit reached")
            .into_compile_error()
            .into();
    }

    let mut tokens = input.into_iter();

    let Some(first) = tokens.next() else {
        return Error::new(Span::call_site(), "expected a value")
            .into_compile_error()
            .into();
    };

    if tokens.next().is_some() {
        return Error::new(first.span(), "expected exactly one value")
            .into_compile_error()
            .into();
    }

    match first {
        TokenTree::Ident(ident) => {
            let bytes = ident.to_string();
            expand_hash(bytes.as_bytes())
        }

        TokenTree::Literal(lit) => {
            let lit = Lit::new(lit);

            match lit {
                Lit::Str(lit) => expand_hash(lit.value().as_bytes()),

                Lit::ByteStr(lit) => expand_hash(&lit.value()),

                Lit::CStr(lit) => expand_hash(&lit.value().as_bytes()),

                Lit::Byte(lit) => expand_hash(&[lit.value()]),

                _ => Error::new(
                    lit.span(),
                    "expected a string literal",
                )
                .into_compile_error()
                .into(),
            }
        }

        TokenTree::Group(group) => {
            hash_impl(group.stream().to_owned(), depth + 1)
        }

        other => Error::new(
            other.span(),
            format!("expected an identifier or string literal, found {:?}", other),
        )
        .into_compile_error()
        .into(),
    }
}

fn expand_hash(bytes: &[u8]) -> TokenStream {
    let hashed = staticly_hash::fnv1a(bytes, None);

    format!("{hashed}u64")
        .parse()
        .expect("generated hash should be a valid u64 literal")
}

#[proc_macro]
pub fn unique(input: TokenStream) -> TokenStream {
    struct UniqueInput(Punctuated<LitStr, Token![,]>);

    impl Parse for UniqueInput {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(
                Punctuated::parse_terminated(input)?
            ))
        }
    }

    let keys = match parse2::<UniqueInput>(input.into())
    {
        Ok(keys) => keys,
        Err(error) => return error.to_compile_error().into(),
    };

    let mut seen = HashSet::new();

    for key in &keys.0 {
        let value = key.value();

        if !seen.insert(value.clone()) {
            return syn::Error::new(
                key.span(),
                format!("duplicate key `{value}`"),
            )
            .to_compile_error()
            .into();
        }
    }

    TokenStream::new()
}
