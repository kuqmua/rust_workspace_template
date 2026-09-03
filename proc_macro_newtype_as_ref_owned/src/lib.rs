#[proc_macro_derive(AsRefOwned)]
pub fn as_ref_owned(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::as_ref_owned(token_stream.into()).into()
}
