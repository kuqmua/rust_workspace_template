#[proc_macro_derive(PartialEqInner)]
pub fn partial_eq_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::partial_eq_inner(token_stream.into()).into()
}
