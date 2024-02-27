//! Derive macro implementation
#![doc(html_root_url = "https://docs.rs/docsplay/0.1.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![allow(clippy::try_err)]

#[allow(unused_extern_crates)]
extern crate proc_macro;

mod attr;
mod expand;
mod fmt;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(
    Display,
    attributes(ignore_extra_doc_attributes, prefix_enum_doc_attributes, display)
)]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
