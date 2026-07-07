#[proc_macro]
pub fn bool_enum_to_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iter = proc_macro2::TokenStream::from(input).into_iter();
    let Some(name) = workspace_macro_helpers::first_ident(&mut iter) else {
        return workspace_macro_helpers::compile_error_ts("bool_enum_to_tokens expects enum name")
            .0
            .into();
    };
    if !workspace_macro_helpers::strip_first_comma(&mut iter) {
        return workspace_macro_helpers::compile_error_ts(
            "bool_enum_to_tokens expects comma after enum name",
        )
        .0
        .into();
    }
    let rest_text = iter.collect::<proc_macro2::TokenStream>().to_string();
    let Some(rest) = rest_text.strip_prefix("false =>") else {
        return workspace_macro_helpers::compile_error_ts(
            "bool_enum_to_tokens expects false => expr",
        )
        .0
        .into();
    };
    let Some((false_expr, true_part)) = rest.split_once(", true =>") else {
        return workspace_macro_helpers::compile_error_ts(
            "bool_enum_to_tokens expects true => expr",
        )
        .0
        .into();
    };
    let Ok(false_ts) = false_expr.trim().parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_ts(
            "bool_enum_to_tokens failed to parse false expr",
        )
        .0
        .into();
    };
    let Ok(true_ts) = true_part.trim().parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_ts(
            "bool_enum_to_tokens failed to parse true expr",
        )
        .0
        .into();
    };
    let name_ident = quote::format_ident!("{name}");
    quote::quote! {
        #[derive(Debug, Clone, Copy, optml::Optml)]
        pub enum #name_ident {
            False,
            True,
        }
        impl quote::ToTokens for #name_ident {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match &self {
                    Self::False => (#false_ts).to_tokens(tokens),
                    Self::True => (#true_ts).to_tokens(tokens),
                }
            }
        }
    }
    .into()
}
