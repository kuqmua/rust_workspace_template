#[proc_macro_derive(CloneFields)]
pub fn clone_fields(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::clone_fields(token_stream.into()).into()
}
