pub fn wrap_into_scopes_token_stream(
    ts: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {(#ts)}.into()
}
