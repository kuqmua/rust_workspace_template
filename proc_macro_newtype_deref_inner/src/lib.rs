#[proc_macro_derive(DerefInner)]
pub fn deref_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::deref_inner(token_stream.into()).into()
}
