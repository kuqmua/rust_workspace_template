#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(from_inner, to_tokens)]
pub struct ProcMacro2IfWriteIsErrTs(proc_macro2::TokenStream);
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
