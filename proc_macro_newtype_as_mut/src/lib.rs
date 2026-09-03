#[proc_macro_derive(AsMut)]
pub fn as_mut(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::as_mut(token_stream.into()).into()
}
