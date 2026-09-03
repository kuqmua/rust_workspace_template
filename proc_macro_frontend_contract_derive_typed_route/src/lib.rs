#[proc_macro_derive(TypedRoute, attributes(typed_route))]
pub fn derive_typed_route(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::derive_typed_route(token_stream.into()).into()
}
