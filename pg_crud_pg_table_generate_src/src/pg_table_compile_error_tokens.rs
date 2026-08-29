pub(super) fn pg_table_compile_error_tokens(
    message: crate::compile_error_message::CompileErrorMessage<'_>,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let message_value = message.0;
    macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {compile_error!(#message_value);},
    )
}
