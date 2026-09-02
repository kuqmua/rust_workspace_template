#[proc_macro]
pub fn impl_to_err_string_with(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Some((types_raw, closure)) = workspace_macro_helpers::split_fat_arrow::split_fat_arrow(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(
            token_stream,
        ),
    ) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_062,
        )
        .into_inner()
        .into();
    };
    let Some((value, body)) =
        workspace_macro_helpers::closure_identifier_and_body::closure_identifier_and_body(closure)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_061,
        )
        .into_inner()
        .into();
    };
    let value_identifier = quote::format_ident!("{value}");
    let types = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(types_raw)
        .into_iter()
        .collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #(impl crate::to_err_string::ToErrString for #types {
            fn to_err_string(&self) -> crate::error_text::ErrorText {
                let #value_identifier = self;
                crate::error_text::ErrorText::try_from(#body).unwrap_or_else(crate::error_text::ErrorText::from)
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_const(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_pairs_res = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(
            token_stream,
        ),
    )
    .into_iter()
    .filter(|part| !part.is_empty())
    .map(|part| {
        workspace_macro_helpers::split_fat_arrow::split_fat_arrow(part)
            .map(|(ty, message)| (ty.into_inner(), message.into_inner()))
            .ok_or_else(|| {
                workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                    constants_str::COMPILE_ERROR_CE_060,
                )
            })
    })
    .collect::<Result<Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream)>, _>>();
    let pairs = match parsed_pairs_res {
        Ok(v) => v,
        Err(error) => return error.into_inner().into(),
    };
    let (types, messages): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    quote::quote! {
        #(impl crate::to_err_string::ToErrString for #types {
            fn to_err_string(&self) -> crate::error_text::ErrorText {
                crate::static_str_to_owned::static_str_to_owned(
                    crate::static_str_to_owned_input::StaticStrToOwnedInput::from(#messages),
                )
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_as_ref_str(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let types = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(
            token_stream,
        ),
    )
    .into_iter()
    .filter(|part| !part.is_empty())
    .collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #(impl crate::to_err_string::ToErrString for #types {
            fn to_err_string(&self) -> crate::error_text::ErrorText {
                crate::as_ref_str_to_owned::as_ref_str_to_owned(self)
            }
        })*
    }
    .into()
}
