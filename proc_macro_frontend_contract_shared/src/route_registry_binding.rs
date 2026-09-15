#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryBinding {
    endpoint: crate::syn_route_registry_endpoint::SynRouteRegistryEndpoint,
    route: crate::syn_route_registry_route::SynRouteRegistryRoute,
}
impl syn::parse::Parse for RouteRegistryBinding {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _parenthesis = syn::parenthesized!(content in parse_stream);
        let route = crate::syn_route_registry_route::SynRouteRegistryRoute::from(
            content.parse::<syn::Type>()?,
        );
        let _comma = content.parse::<syn::Token![,]>()?;
        let endpoint = crate::syn_route_registry_endpoint::SynRouteRegistryEndpoint::from(
            content.parse::<syn::Path>()?,
        );
        Ok(Self::new(endpoint, route))
    }
}
