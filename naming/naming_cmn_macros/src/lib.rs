use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts2;
use quote::{format_ident, quote};
use workspace_macro_helpers::{compile_error_ts, first_ident_at, part_at, split_top_level_commas};
#[proc_macro]
pub fn case_trait_pair(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input.into());
    if parts.len() != 4 {
        return compile_error_ts(
            "case_trait_pair expects str trait, ts trait, bound, closure expr",
        )
        .into();
    }
    let Some(str_trait) = first_ident_at(&parts, 0) else {
        return compile_error_ts("case_trait_pair expects string trait name").into();
    };
    let Some(ts_trait) = first_ident_at(&parts, 1) else {
        return compile_error_ts("case_trait_pair expects token trait name").into();
    };
    let str_trait_ident = format_ident!("{str_trait}");
    let ts_trait_ident = format_ident!("{ts_trait}");
    let Some(bound_ts) = part_at(&parts, 2) else {
        return compile_error_ts("case_trait_pair expects bound").into();
    };
    let Some(closure_text) = part_at(&parts, 3).map(|part| part.to_string()) else {
        return compile_error_ts("case_trait_pair expects closure").into();
    };
    let Some((param_part, body_part)) = closure_text
        .split_once('|')
        .and_then(|(_, rest)| rest.split_once('|'))
    else {
        return compile_error_ts("case_trait_pair expects closure").into();
    };
    let param_ident = format_ident!("{}", param_part.trim());
    let Ok(body_ts) = body_part.trim().parse::<Ts2>() else {
        return compile_error_ts("case_trait_pair failed to parse body").into();
    };
    quote! {
        pub trait #str_trait_ident {
            fn case(&self) -> String;
        }
        impl<T> #str_trait_ident for T
        where
            T: #bound_ts,
        {
            fn case(&self) -> String {
                let #param_ident = self;
                #body_ts
            }
        }
        pub trait #ts_trait_ident {
            fn case_or_panic(&self) -> Ts2;
        }
        impl<T> #ts_trait_ident for T
        where
            T: #str_trait_ident,
        {
            fn case_or_panic(&self) -> Ts2 {
                to_ts_or_panic(&#str_trait_ident::case(self))
            }
        }
    }
    .into()
}
