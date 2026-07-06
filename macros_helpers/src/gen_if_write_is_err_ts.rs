pub fn gen_if_write_is_err_ts(
    prms_ts: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote::quote! {
        if {
            use std::fmt::Write as _;
            write!(#prms_ts)
        }.is_err() {
            #ts
        }
    }
}
