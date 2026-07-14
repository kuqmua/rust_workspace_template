#[proc_macro]
pub fn bool_enum_to_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iter = proc_macro2::TokenStream::from(input).into_iter();
    let Some(name) = workspace_macro_helpers::first_identifier(&mut iter) else {
        return workspace_macro_helpers::compile_error_token_stream(
            "bool_enum_to_tokens expects enum name",
        )
        .into_inner()
        .into();
    };
    if !workspace_macro_helpers::strip_first_comma(&mut iter) {
        return workspace_macro_helpers::compile_error_token_stream(
            "bool_enum_to_tokens expects comma after enum name",
        )
        .into_inner()
        .into();
    }
    let rest_text = iter.collect::<proc_macro2::TokenStream>().to_string();
    let Some(rest) = rest_text.strip_prefix("false =>") else {
        return workspace_macro_helpers::compile_error_token_stream(
            "bool_enum_to_tokens expects false => expr",
        )
        .into_inner()
        .into();
    };
    let Some((false_expr, true_part)) = rest.split_once(", true =>") else {
        return workspace_macro_helpers::compile_error_token_stream(
            "bool_enum_to_tokens expects true => expr",
        )
        .into_inner()
        .into();
    };
    let Ok(false_token_stream) = false_expr.trim().parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_token_stream(
            "bool_enum_to_tokens failed to parse false expr",
        )
        .into_inner()
        .into();
    };
    let Ok(true_token_stream) = true_part.trim().parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_token_stream(
            "bool_enum_to_tokens failed to parse true expr",
        )
        .into_inner()
        .into();
    };
    let name_identifier = quote::format_ident!("{name}");
    quote::quote! {
        #[derive(Debug, Clone, Copy, optml::Optml)]
        pub enum #name_identifier {
            False,
            True,
        }
        impl quote::ToTokens for #name_identifier {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match &self {
                    Self::False => (#false_token_stream).to_tokens(tokens),
                    Self::True => (#true_token_stream).to_tokens(tokens),
                }
            }
        }
    }
    .into()
}
