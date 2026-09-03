#[proc_macro_derive(Accessor)]
pub fn accessor(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::accessor(token_stream.into()).into()
}
