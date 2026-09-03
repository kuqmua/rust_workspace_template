#[proc_macro_derive(BoundedStringWrapper, attributes(bounded_string))]
pub fn bounded_string_wrapper(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::bounded_string_wrapper(token_stream.into()).into()
}
