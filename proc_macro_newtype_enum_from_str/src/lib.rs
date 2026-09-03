#[proc_macro_derive(EnumFromStr)]
pub fn enum_from_str(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::enum_from_str(token_stream.into()).into()
}
