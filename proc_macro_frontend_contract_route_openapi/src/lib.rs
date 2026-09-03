#[proc_macro_attribute]
pub fn route_openapi(
    attribute_token_stream: proc_macro::TokenStream,
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::route_openapi(
        attribute_token_stream.into(),
        input_token_stream.into(),
    )
    .into()
}
