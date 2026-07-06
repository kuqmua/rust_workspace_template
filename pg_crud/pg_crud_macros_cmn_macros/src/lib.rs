use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts2;
use quote::{format_ident, quote};
use workspace_macro_helpers::{compile_error_ts, first_ident, strip_first_comma};
#[proc_macro]
pub fn bool_enum_to_tokens(input: TokenStream) -> TokenStream {
    let mut iter = Ts2::from(input).into_iter();
    let Some(name) = first_ident(&mut iter) else {
        return compile_error_ts("bool_enum_to_tokens expects enum name").into();
    };
    if !strip_first_comma(&mut iter) {
        return compile_error_ts("bool_enum_to_tokens expects comma after enum name").into();
    }
    let rest_text = iter.collect::<Ts2>().to_string();
    let Some(rest) = rest_text.strip_prefix("false =>") else {
        return compile_error_ts("bool_enum_to_tokens expects false => expr").into();
    };
    let Some((false_expr, true_part)) = rest.split_once(", true =>") else {
        return compile_error_ts("bool_enum_to_tokens expects true => expr").into();
    };
    let Ok(false_ts) = false_expr.trim().parse::<Ts2>() else {
        return compile_error_ts("bool_enum_to_tokens failed to parse false expr").into();
    };
    let Ok(true_ts) = true_part.trim().parse::<Ts2>() else {
        return compile_error_ts("bool_enum_to_tokens failed to parse true expr").into();
    };
    let name_ident = format_ident!("{name}");
    quote! {
        #[derive(Debug, Clone, Copy, Optml)]
        pub enum #name_ident {
            False,
            True,
        }
        impl ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut Ts2) {
                match &self {
                    Self::False => (#false_ts).to_tokens(tokens),
                    Self::True => (#true_ts).to_tokens(tokens),
                }
            }
        }
    }
    .into()
}
