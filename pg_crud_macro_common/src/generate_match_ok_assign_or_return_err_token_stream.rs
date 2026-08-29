#[must_use]
pub fn generate_match_ok_assign_or_return_err_token_stream(
    expr_token_stream: &dyn quote::ToTokens,
    assign_target_token_stream: &dyn quote::ToTokens,
    ok_v_token_stream: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_ctx::NamesCtx::new();
    #[allow(
        non_snake_case,
        reason = "generated Rust identifiers intentionally mirror emitted naming tokens"
    )]
    let (ErrorSnakeCase,) = (names.get_error_snake_case(),);
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
