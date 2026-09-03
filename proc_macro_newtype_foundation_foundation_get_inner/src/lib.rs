#[proc_macro_derive(GetInner, attributes(accessor, borrow))]
pub fn foundation_get_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_foundation_shared::foundation_get_inner(token_stream.into()).into()
}
