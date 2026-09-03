#[proc_macro_derive(IntoVec)]
pub fn into_vec(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::into_vec(token_stream.into()).into()
}
