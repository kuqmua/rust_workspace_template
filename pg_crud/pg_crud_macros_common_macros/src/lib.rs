#[proc_macro]
pub fn bool_enum_to_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iter = proc_macro2::TokenStream::from(input).into_iter();
    let Some(name) = workspace_macro_helpers::first_identifier(&mut iter) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_045,
        )
        .into_inner()
        .into();
    };
    if !workspace_macro_helpers::strip_first_comma(&mut iter) {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_044,
        )
        .into_inner()
        .into();
    }
    let rest_text = iter.collect::<proc_macro2::TokenStream>().to_string();
    let Some(rest) = rest_text.strip_prefix(str_constants::FALSE_FAT_ARROW) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_046,
        )
        .into_inner()
        .into();
    };
    let Some((false_expr, true_part)) = rest.split_once(str_constants::TRUE_FAT_ARROW) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_047,
        )
        .into_inner()
        .into();
    };
    let Ok(false_token_stream) = false_expr.trim().parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_048,
        )
        .into_inner()
        .into();
    };
    let Ok(true_token_stream) = true_part.trim().parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_049,
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
