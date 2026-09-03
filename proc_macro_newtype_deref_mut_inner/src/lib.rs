#[proc_macro_derive(DerefMutInner)]
pub fn deref_mut_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::deref_mut_inner(token_stream.into()).into()
}
