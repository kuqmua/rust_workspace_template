#[proc_macro_derive(TryFrom, attributes(try_from))]
pub fn try_from(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::try_from(token_stream.into()).into()
}
