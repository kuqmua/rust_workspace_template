#[proc_macro_derive(AsSlice)]
pub fn as_slice(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::as_slice(token_stream.into()).into()
}
