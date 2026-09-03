#[proc_macro_attribute]
pub fn errors_with_location(
    attribute_token_stream: proc_macro::TokenStream,
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_location_shared::errors_with_location(
        attribute_token_stream.into(),
        input_token_stream.into(),
    )
    .into()
}
