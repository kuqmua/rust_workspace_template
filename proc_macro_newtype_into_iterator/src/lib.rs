#[proc_macro_derive(IntoIterator)]
pub fn into_iterator(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::into_iterator(token_stream.into()).into()
}
