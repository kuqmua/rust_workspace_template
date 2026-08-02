#[derive(optml::Optml, Debug, Clone, newtype::FromInner, newtype::ToTokens)]
pub struct ProcMacro2IfWriteIsErrTokenStream(proc_macro2::TokenStream);
pub fn generate_if_write_is_err_token_stream(
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> ProcMacro2IfWriteIsErrTokenStream {
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
