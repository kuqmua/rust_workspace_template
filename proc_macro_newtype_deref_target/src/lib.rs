#[proc_macro_derive(DerefTarget)]
pub fn deref_target(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::deref_target(token_stream.into()).into()
}
