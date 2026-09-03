#[proc_macro_derive(RouteCatalog, attributes(route_catalog, route_catalog_route))]
pub fn derive_route_catalog(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::derive_route_catalog(token_stream.into()).into()
}
