#[proc_macro_derive(RouteFamily, attributes(route_family, route_family_body_limit))]
pub fn derive_route_family(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::derive_route_family(token_stream.into()).into()
}
