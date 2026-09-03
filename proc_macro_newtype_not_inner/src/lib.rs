#[proc_macro_derive(NotInner)]
pub fn not_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::not_inner(token_stream.into()).into()
}
