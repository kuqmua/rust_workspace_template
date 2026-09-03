#[proc_macro_derive(AsRefInner)]
pub fn as_ref_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::as_ref_inner(token_stream.into()).into()
}
