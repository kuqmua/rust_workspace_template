pub fn generate_impl_to_err_string_no_generics_token_stream(
    identifier: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    macro_helpers::domain_types::generate_impl_to_err_string_token_stream::generate_impl_to_err_string_token_stream(
        &proc_macro2::TokenStream::new(),
        identifier,
        &proc_macro2::TokenStream::new(),
        ts,
    )
}
