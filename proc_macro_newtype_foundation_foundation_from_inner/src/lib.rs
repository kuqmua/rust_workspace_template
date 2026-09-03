#[proc_macro_derive(FromInner)]
pub fn foundation_from_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_foundation_shared::foundation_from_inner(token_stream.into()).into()
}
