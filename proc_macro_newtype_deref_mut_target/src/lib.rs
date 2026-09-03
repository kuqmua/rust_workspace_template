#[proc_macro_derive(DerefMutTarget)]
pub fn deref_mut_target(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::deref_mut_target(token_stream.into()).into()
}
