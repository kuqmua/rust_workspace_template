#[proc_macro_derive(AsRefTarget)]
pub fn as_ref_target(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::as_ref_target(token_stream.into()).into()
}
