use proc_macro::TokenStream;
use proc_macro2::{Delimiter, TokenStream as Ts2, TokenTree};
use quote::{format_ident, quote};
use workspace_macro_helpers::{
    compile_error_ts, first_ident, split_top_level_commas, strip_first_comma,
};
fn gen_tp(input: Ts2) -> Ts2 {
    let mut iter = input.into_iter();
    let Some(name) = first_ident(&mut iter) else {
        return compile_error_ts("tp expects type name");
    };
    if !strip_first_comma(&mut iter) {
        return compile_error_ts("tp expects comma after type name");
    }
    let body = iter.collect::<Ts2>();
    let name_ident = format_ident!("{name}");
    quote! {
        #[derive(Debug, Clone, Copy, Optml)]
        pub struct #name_ident;
        impl ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut Ts2) {
                append_tokens(tokens, quote! {#body});
            }
        }
    }
}
#[proc_macro]
pub fn tp(input: TokenStream) -> TokenStream {
    gen_tp(input.into()).into()
}
#[proc_macro]
pub fn tp_parts(input: TokenStream) -> TokenStream {
    let mut parts = split_top_level_commas(input.into());
    if parts.len() < 2 {
        return compile_error_ts("tp_parts expects type name and at least one part").into();
    }
    let mut name_iter = parts.remove(0).into_iter();
    let Some(name) = first_ident(&mut name_iter) else {
        return compile_error_ts("tp_parts expects type name").into();
    };
    let name_ident = format_ident!("{name}");
    let part_streams = parts.into_iter().collect::<Vec<Ts2>>();
    quote! {
        #[derive(Debug, Clone, Copy, Optml)]
        pub struct #name_ident;
        impl ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut Ts2) {
                #(append_tokens(tokens, #part_streams);)*
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn ts_path_fn(input: TokenStream) -> TokenStream {
    let mut iter = Ts2::from(input).into_iter();
    let Some(name) = first_ident(&mut iter) else {
        return compile_error_ts("ts_path_fn expects function name").into();
    };
    if !strip_first_comma(&mut iter) {
        return compile_error_ts("ts_path_fn expects comma after function name").into();
    }
    let body = iter.collect::<Ts2>();
    let name_ident = format_ident!("{name}");
    quote! {
        fn #name_ident() -> Ts2 {
            quote! {#body}
        }
    }
    .into()
}
#[proc_macro]
pub fn tp_batch(input: TokenStream) -> TokenStream {
    let mut output = Ts2::new();
    for token in Ts2::from(input) {
        if let TokenTree::Group(group) = token
            && group.delimiter() == Delimiter::Parenthesis
        {
            output.extend(gen_tp(group.stream()));
        }
    }
    output.into()
}
