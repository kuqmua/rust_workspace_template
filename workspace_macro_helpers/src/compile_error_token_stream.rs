pub fn compile_error_token_stream<S>(
    message: S,
) -> crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens
where
    S: AsRef<str>,
{
    let compile_message = message.as_ref().to_owned();
    quote::quote! {compile_error!(#compile_message);}.into()
}
