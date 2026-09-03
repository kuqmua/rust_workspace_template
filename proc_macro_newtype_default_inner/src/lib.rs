#[proc_macro_derive(DefaultInner)]
pub fn default_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::default_inner(token_stream.into()).into()
}
