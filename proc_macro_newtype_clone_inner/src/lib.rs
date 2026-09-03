#[proc_macro_derive(CloneInner)]
pub fn clone_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::clone_inner(token_stream.into()).into()
}
