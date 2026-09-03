#[proc_macro_derive(AsRefInner)]
pub fn foundation_as_ref_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_foundation_shared::foundation_as_ref_inner(token_stream.into()).into()
}
