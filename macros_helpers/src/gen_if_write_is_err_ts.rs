#[derive(Debug, Clone)]
pub struct ProcMacro2IfWriteIsErrTs(proc_macro2::TokenStream);
impl quote::ToTokens for ProcMacro2IfWriteIsErrTs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
impl From<proc_macro2::TokenStream> for ProcMacro2IfWriteIsErrTs {
    fn from(value: proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
pub fn gen_if_write_is_err_ts(
    prms_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> ProcMacro2IfWriteIsErrTs {
    quote::quote! {
        if {
            use std::fmt::Write as _;
            write!(#prms_ts)
        }.is_err() {
            #ts
        }
    }
    .into()
}
