#[proc_macro_derive(IntoInner)]
pub fn into_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::into_inner(token_stream.into()).into()
}
