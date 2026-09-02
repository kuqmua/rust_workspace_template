pub(super) fn generate_read_inner_into_read_or_update_with_new_or_try_new_unwraped_token_stream(
    method_name_token_stream: &dyn quote::ToTokens,
    type_token_stream: &dyn quote::ToTokens,
    path_token_stream: &dyn quote::ToTokens,
    return_type_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_context::NamesContext::new();
    #[allow(
        non_snake_case,
        reason = "generated Rust identifiers intentionally mirror emitted naming tokens"
    )]
    let (VSnakeCase,) = (names.get_v_snake_case(),);
    quote::quote! {
        fn #method_name_token_stream(
            #VSnakeCase: #type_token_stream
        ) -> #path_token_stream::#return_type_token_stream {
            #ts
        }
    }
    .into()
}
