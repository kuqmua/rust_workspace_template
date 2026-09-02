#[must_use]
pub fn generate_if_let_some_match_ok_assign_query_or_return_err_token_stream(
    expr_token_stream: &dyn quote::ToTokens,
    some_v_token_stream: &dyn quote::ToTokens,
    ok_v_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_context::NamesContext::new();
    #[allow(
        non_snake_case,
        reason = "generated Rust identifiers intentionally mirror emitted naming tokens"
    )]
    let (QuerySnakeCase, VSnakeCase) = (names.get_query_snake_case(), names.get_v_snake_case());
    let match_token_stream = crate::generate_match_ok_assign_or_return_err_token_stream::generate_match_ok_assign_or_return_err_token_stream(
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
