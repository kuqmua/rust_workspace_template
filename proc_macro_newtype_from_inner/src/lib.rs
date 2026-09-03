#[proc_macro_derive(FromInner)]
pub fn from_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::from_inner(token_stream.into()).into()
}
