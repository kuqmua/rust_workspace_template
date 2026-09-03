#[proc_macro_derive(FromGetter, attributes(from_getter))]
pub fn from_getter(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::from_getter(token_stream.into()).into()
}
