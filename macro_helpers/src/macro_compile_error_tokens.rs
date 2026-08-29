pub(super) fn macro_compile_error_tokens(
    message: super::compile_error_message::CompileErrorMessage<'_>,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let message_value = message.0;
    crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {compile_error!(#message_value);},
    )
}
