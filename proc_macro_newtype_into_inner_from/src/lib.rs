#[proc_macro_derive(IntoInnerFrom)]
pub fn into_inner_from(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::into_inner_from(token_stream.into()).into()
}
