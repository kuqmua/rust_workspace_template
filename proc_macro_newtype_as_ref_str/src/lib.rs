#[proc_macro_derive(AsRefStr)]
pub fn as_ref_str(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::as_ref_str(token_stream.into()).into()
}
