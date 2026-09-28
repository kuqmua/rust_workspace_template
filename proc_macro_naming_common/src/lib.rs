#[proc_macro]
pub fn case_trait_pair(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(
            token_stream,
        ),
    );
    if parts.len() != 4 {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_PARTS_ERROR,
        )
        .into_inner()
        .into();
    }
    let Some(str_trait) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 0)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_STR_TRAIT_ERROR,
        )
        .into_inner()
        .into();
    };
    let Some(ts_trait) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 1)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_TS_TRAIT_ERROR,
        )
        .into_inner()
        .into();
    };
    let str_trait_identifier = quote::format_ident!("{str_trait}");
    let ts_trait_identifier = quote::format_ident!("{ts_trait}");
    let Some(bound_token_stream) = workspace_macro_helpers::part_at::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_BOUND_ERROR,
        )
        .into_inner()
        .into();
    };
    let Some((parameter_identifier, body_tokens)) =
        workspace_macro_helpers::part_at::part_at(&parts, 3)
            .map(|part| {
                workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(
                    part.into_iter()
                        .skip_while(|token| {
                            !matches!(token, proc_macro2::TokenTree::Punct(punctuation) if punctuation.as_char() == '|')
                        })
                        .collect::<proc_macro2::TokenStream>(),
                )
            })
            .and_then(workspace_macro_helpers::closure_identifier_and_body::closure_identifier_and_body)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_CLOSURE_ERROR,
        )
        .into_inner()
        .into();
    };
    let param_identifier = quote::format_ident!("{parameter_identifier}");
    let Ok(body_token_stream) = body_tokens.to_string().parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_PARSE_BODY_ERROR,
        )
        .into_inner()
        .into();
    };
    let interpolation_marker = proc_macro2::Punct::new('#', proc_macro2::Spacing::Alone);
    quote::quote! {
        pub trait #str_trait_identifier {
            fn case(&self) -> String;
            fn try_case(&self) -> Result<String, crate::case_string::CaseStringTryFromStringError> {
                Ok(self.case())
            }
        }
        impl<T> #str_trait_identifier for T
        where
            T: #bound_token_stream,
        {
            fn case(&self) -> String {
                #str_trait_identifier::try_case(self).unwrap_or_else(|error| error.to_string())
            }

            fn try_case(&self) -> Result<String, crate::case_string::CaseStringTryFromStringError> {
                let #param_identifier = self;
                #body_token_stream
            }
        }
        pub trait #ts_trait_identifier {
            fn case_or_panic(&self) -> proc_macro2::TokenStream;
        }
        impl<T> #ts_trait_identifier for T
        where
            T: #str_trait_identifier,
        {
            fn case_or_panic(&self) -> proc_macro2::TokenStream {
                match #str_trait_identifier::try_case(self) {
                    Ok(case_string) => crate::to_token_stream_or_panic::to_token_stream_or_panic(&case_string).into_inner(),
                    Err(error) => {
                        let message = error.to_string();
                        quote::quote! {compile_error!(#interpolation_marker message);}
                    }
                }
            }
        }
    }
    .into()
}
