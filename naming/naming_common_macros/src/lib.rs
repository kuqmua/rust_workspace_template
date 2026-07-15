#[proc_macro]
pub fn case_trait_pair(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 4 {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_PARTS_ERROR,
        )
        .into_inner()
        .into();
    }
    let Some(str_trait) = workspace_macro_helpers::first_identifier_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_STR_TRAIT_ERROR,
        )
        .into_inner()
        .into();
    };
    let Some(ts_trait) = workspace_macro_helpers::first_identifier_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_TS_TRAIT_ERROR,
        )
        .into_inner()
        .into();
    };
    let str_trait_identifier = quote::format_ident!("{str_trait}");
    let ts_trait_identifier = quote::format_ident!("{ts_trait}");
    let Some(bound_token_stream) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_BOUND_ERROR,
        )
        .into_inner()
        .into();
    };
    let Some(closure_text) =
        workspace_macro_helpers::part_at(&parts, 3).map(|part| part.to_string())
    else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_CLOSURE_ERROR,
        )
        .into_inner()
        .into();
    };
    let Some((param_part, body_part)) = closure_text
        .split_once('|')
        .and_then(|(_, rest)| rest.split_once('|'))
    else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_CLOSURE_ERROR,
        )
        .into_inner()
        .into();
    };
    let param_identifier = quote::format_ident!("{}", param_part.trim());
    let Ok(body_token_stream) = body_part.trim().parse::<proc_macro2::TokenStream>() else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_PARSE_BODY_ERROR,
        )
        .into_inner()
        .into();
    };
    quote::quote! {
        pub trait #str_trait_identifier {
            fn case(&self) -> String;
        }
        impl<T> #str_trait_identifier for T
        where
            T: #bound_token_stream,
        {
            fn case(&self) -> String {
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
                to_token_stream_or_panic(&#str_trait_identifier::case(self)).0
            }
        }
    }
    .into()
}
