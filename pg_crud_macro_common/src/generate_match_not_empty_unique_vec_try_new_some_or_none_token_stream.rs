#[must_use]
pub fn generate_match_not_empty_unique_vec_try_new_some_or_none_token_stream(
    import: &crate::import::Import,
    expr_token_stream: &dyn quote::ToTokens,
    ok_v_token_stream: &dyn quote::ToTokens,
    panic_uuid: crate::panic_uuid_ref::PanicUuidRef<'_>,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let panic_uuid_token_stream =
        generate_quotes::dq_token_stream::dq_token_stream(panic_uuid.as_ref());
    quote::quote! {
        match #expr_token_stream {
            Ok(#ok_v_token_stream) => Some(#ok_v_token_stream),
            Err(error) => match error {
                #import::not_empty_unique_vec::NotEmptyUniqueVecTryNewError::IsEmpty {..} => None,
                #import::not_empty_unique_vec::NotEmptyUniqueVecTryNewError::NotUnique {..} => {
                    panic!("7bc2a614 generated non-unique value invariant failed: {}", #panic_uuid_token_stream)
                }
            }
        }
    }
    .into()
}
