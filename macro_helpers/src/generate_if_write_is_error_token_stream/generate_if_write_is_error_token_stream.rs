pub fn generate_if_write_is_error_token_stream(
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> super::ProcMacro2IfWriteIsErrTokenStream {
    quote::quote! {
        if {
            use std::fmt::Write as _;
            write!(#parameters_token_stream)
        }.is_err() {
            #ts
        }
    }
    .into()
}
