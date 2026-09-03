#[proc_macro_derive(AsRef)]
pub fn as_ref(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::as_ref(token_stream.into()).into()
}
