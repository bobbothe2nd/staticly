//! Macros for `staticly` hashing/lookup

use proc_macro::{Delimiter, Literal, Span, TokenStream, TokenTree};
use std::collections::HashSet;

macro_rules! error {
    ($span:expr, $($args:tt)*) => {{
        let msg = ::std::format!($($args)*);

        let mut tokens = ::proc_macro::TokenStream::new();

        tokens.extend([
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(':', ::proc_macro::Spacing::Joint)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(':', ::proc_macro::Spacing::Alone)),
            ::proc_macro::TokenTree::Ident(::proc_macro::Ident::new("core", $span)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(':', ::proc_macro::Spacing::Joint)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(':', ::proc_macro::Spacing::Alone)),
            ::proc_macro::TokenTree::Ident(::proc_macro::Ident::new("compile_error", $span)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new('!', ::proc_macro::Spacing::Alone)),
        ]);

        let args = ::proc_macro::TokenStream::from(::proc_macro::TokenTree::Literal(::proc_macro::Literal::string(&msg)));

        tokens.extend([
            ::proc_macro::TokenTree::Group(::proc_macro::Group::new(::proc_macro::Delimiter::Parenthesis, args)),
            ::proc_macro::TokenTree::Punct(::proc_macro::Punct::new(';', ::proc_macro::Spacing::Alone)),
        ]);

        tokens
    }};
}

const MAX_DEPTH: usize = 32;

#[proc_macro]
pub fn hash(input: TokenStream) -> TokenStream {
    hash_impl(input, 0)
}

fn hash_impl(input: TokenStream, depth: usize) -> TokenStream {
    if depth >= MAX_DEPTH {
        return error!(Span::call_site(), "hash recursion limit reached");
    }

    let mut tokens = input.into_iter();

    let Some(first) = tokens.next() else {
        return error!(Span::call_site(), "expected a value");
    };

    if tokens.next().is_some() {
        return error!(first.span(), "expected exactly one value");
    }

    match first {
        TokenTree::Ident(ident) => {
            let bytes = ident.to_string();
            expand_hash(bytes.as_bytes())
        }

        TokenTree::Literal(lit) => {
            if let Some(lit) = parse_string_literal(&lit) {
                expand_hash(lit.as_bytes())
            } else {
                error!(
                    lit.span(),
                    "expected a string literal",
                )
            }
        }

        TokenTree::Group(group) if group.delimiter() == Delimiter::None => {
            hash_impl(group.stream().to_owned(), depth + 1)
        }

        other => error!(
            other.span(),
            "expected an identifier or string literal, found {:?}", other,
        ),
    }
}

fn expand_hash(bytes: &[u8]) -> TokenStream {
    let hashed = staticly_hash::fnv1a(bytes);

    format!("{hashed}u64")
        .parse()
        .expect("generated hash should be a valid u64 literal")
}


#[proc_macro]
pub fn unique(input: TokenStream) -> TokenStream {
    let input = unwrap_none_groups(input, 0);

    let mut seen = HashSet::new();
    let mut expect_literal = true;

    for token in input {
        match token {
            TokenTree::Punct(punct)
                if punct == ',' && !expect_literal =>
            {
                expect_literal = true;
            }

            TokenTree::Literal(lit) if expect_literal => {
                let value = match parse_string_literal(&lit) {
                    Some(value) => value,
                    None => {
                        return error!(
                            lit.span(),
                            "expected a string literal",
                        );
                    }
                };

                if !seen.insert(value.clone()) {
                    return error!(
                        lit.span(),
                        "duplicate key `{value}`",
                    );
                }

                expect_literal = false;
            }

            token => {
                return error!(
                    token.span(),
                    "expected a string literal followed by `,`",
                );
            }
        }
    }

    TokenStream::new()
}

fn unwrap_none_groups(input: TokenStream, depth: usize) -> TokenStream {
    if depth >= MAX_DEPTH {
        return input;
    }

    input
        .into_iter()
        .flat_map(|token| match token {
            TokenTree::Group(group)
                if group.delimiter() == Delimiter::None =>
            {
                unwrap_none_groups(group.stream(), depth + 1)
            }
            other => TokenStream::from(other),
        })
        .collect()
}

fn parse_string_literal(lit: &Literal) -> Option<String> {
    let text = lit.to_string();

    if !text.starts_with('"') || !text.ends_with('"') {
        None
    } else {
        Some(text[1..text.len() - 1].to_string())
    }
}
