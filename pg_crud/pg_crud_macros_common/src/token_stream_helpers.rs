#[must_use]
pub fn parse_strs_to_ts2_vec(
    v: crate::ParseTokenStreamStrings,
    uuid: crate::ParseErrorIdRef<'_>,
) -> crate::GeneratedRustTokenStreamVec {
    v.into_generated_vec(uuid)
}
#[must_use]
pub fn generate_mod_with_pub_use_token_stream(
    mod_name: &dyn quote::ToTokens,
    content_token_stream: &crate::GeneratedRustTokenStreamVec,
) -> macros_helpers::generated_rust_token_stream::GeneratedRustTokenStream {
    quote::quote! {
        #[allow(unused_qualifications)]
        #[allow(unused_variables)]
        #[allow(clippy::absolute_paths)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        mod #mod_name {
            #content_token_stream
        }
        pub use #mod_name::*;
    }
    .into()
}
#[must_use]
pub fn common_d_token_stream_builder()
-> macros_helpers::derive_token_stream_builder::DTokenStreamBuilder {
    macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
        .make_pub()
        .d_debug()
        .d_clone()
        .d_partial_eq()
        .d_serde_serialize()
        .d_serde_deserialize()
}
#[must_use]
pub fn serde_error_enum_d_token_stream_builder()
-> macros_helpers::derive_token_stream_builder::DTokenStreamBuilder {
    macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
        .make_pub()
        .d_debug()
        .d_serde_serialize()
        .d_serde_deserialize()
        .d_thiserror_error()
        .d_location_location()
}
#[must_use]
pub fn error_enum_d_token_stream_builder()
-> macros_helpers::derive_token_stream_builder::DTokenStreamBuilder {
    macros_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
        .make_pub()
        .d_debug()
        .d_thiserror_error()
        .d_location_location()
}
#[must_use]
pub fn generate_match_ok_assign_or_return_err_token_stream(
    expr_token_stream: &dyn quote::ToTokens,
    assign_target_token_stream: &dyn quote::ToTokens,
    ok_v_token_stream: &dyn quote::ToTokens,
) -> macros_helpers::generated_rust_token_stream::GeneratedRustTokenStream {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (ErrorSnakeCase,) = (&names.ErrorSnakeCase,);
    quote::quote! {
        match #expr_token_stream {
            Ok(#ok_v_token_stream) => {
                #assign_target_token_stream = #ok_v_token_stream;
            }
            Err(#ErrorSnakeCase) => {
                return Err(#ErrorSnakeCase);
            }
        }
    }
    .into()
}
#[must_use]
pub fn generate_match_ok_or_return_err_token_stream(
    expr_token_stream: &dyn quote::ToTokens,
    ok_v_token_stream: &dyn quote::ToTokens,
) -> macros_helpers::generated_rust_token_stream::GeneratedRustTokenStream {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (ErrorSnakeCase,) = (&names.ErrorSnakeCase,);
    quote::quote! {
        match #expr_token_stream {
            Ok(#ok_v_token_stream) => #ok_v_token_stream,
            Err(#ErrorSnakeCase) => {
                return Err(#ErrorSnakeCase);
            }
        }
    }
    .into()
}
#[must_use]
pub fn generate_match_not_empty_unique_vec_try_new_some_or_none_token_stream(
    import: &crate::Import,
    expr_token_stream: &dyn quote::ToTokens,
    ok_v_token_stream: &dyn quote::ToTokens,
    panic_uuid: crate::PanicUuidRef<'_>,
) -> macros_helpers::generated_rust_token_stream::GeneratedRustTokenStream {
    let panic_uuid_token_stream = generate_quotes::dq_token_stream(panic_uuid.as_ref());
    quote::quote! {
        match #expr_token_stream {
            Ok(#ok_v_token_stream) => Some(#ok_v_token_stream),
            Err(error) => match error {
                #import::NotEmptyUniqueVecTryNewError::IsEmpty {..} => None,
                #import::NotEmptyUniqueVecTryNewError::NotUnique {..} => panic!(#panic_uuid_token_stream)
            }
        }
    }
    .into()
}
#[must_use]
pub fn generate_if_let_some_match_ok_assign_query_or_return_err_token_stream(
    expr_token_stream: &dyn quote::ToTokens,
    some_v_token_stream: &dyn quote::ToTokens,
    ok_v_token_stream: &dyn quote::ToTokens,
) -> macros_helpers::generated_rust_token_stream::GeneratedRustTokenStream {
    let names = crate::NamesCtx::new();
    #[allow(non_snake_case)]
    let (QuerySnakeCase, VSnakeCase) = (&names.QuerySnakeCase, &names.VSnakeCase);
    let match_token_stream = generate_match_ok_assign_or_return_err_token_stream(
        expr_token_stream,
        &QuerySnakeCase,
        ok_v_token_stream,
    );
    quote::quote! {
        if let Some(#some_v_token_stream) = &#VSnakeCase.0 {
            #match_token_stream
        }
        Ok(#QuerySnakeCase)
    }
    .into()
}
