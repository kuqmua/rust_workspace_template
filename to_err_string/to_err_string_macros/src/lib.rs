use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts2;
use quote::{format_ident, quote};
use workspace_macro_helpers::{
    closure_ident_and_body, compile_error_ts, split_fat_arrow, split_top_level_commas,
};
#[proc_macro]
pub fn impl_to_err_string_with(input: TokenStream) -> TokenStream {
    let Some((types_raw, closure)) = split_fat_arrow(input.into()) else {
        return compile_error_ts("impl_to_err_string_with expects types => |value| body").into();
    };
    let Some((value, body)) = closure_ident_and_body(closure) else {
        return compile_error_ts("impl_to_err_string_with expects closure").into();
    };
    let value_ident = format_ident!("{value}");
    let types = split_top_level_commas(types_raw)
        .into_iter()
        .collect::<Vec<Ts2>>();
    quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> String {
                let #value_ident = self;
                #body
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_const(input: TokenStream) -> TokenStream {
    let mut types = Vec::new();
    let mut msgs = Vec::new();
    for part in split_top_level_commas(input.into()) {
        if part.is_empty() {
            continue;
        }
        let Some((ty, msg)) = split_fat_arrow(part) else {
            return compile_error_ts("impl_to_err_string_const expects type => msg").into();
        };
        types.push(ty);
        msgs.push(msg);
    }
    quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> String {
                static_str_to_owned(#msgs)
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_as_ref_str(input: TokenStream) -> TokenStream {
    let types = split_top_level_commas(input.into())
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<Ts2>>();
    quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> String {
                as_ref_str_to_owned(self)
            }
        })*
    }
    .into()
}
