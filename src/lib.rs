#![feature(proc_macro_tracked_env)]
extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::LitInt;
use syn::token::Comma;

#[allow(dead_code)]
#[derive(derive_syn_parse::Parse)]
struct EnvIntArgs {
    env_name: Ident,
    comma: Comma,
    default: LitInt
}

#[proc_macro]
pub fn env_int(args: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let EnvIntArgs {env_name, default, .. }: EnvIntArgs = syn::parse(args).unwrap();
    let lit_int = proc_macro::tracked_env::var(&env_name.to_string()).map(|x| LitInt::new(&x, Span::call_site())).unwrap_or_else(|_| default);
    lit_int.to_token_stream().into()
}