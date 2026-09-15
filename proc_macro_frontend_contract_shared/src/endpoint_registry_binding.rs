#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryBinding {
    contract: crate::syn_endpoint_registry_contract::SynEndpointRegistryContract,
    endpoint: crate::syn_endpoint_registry_endpoint::SynEndpointRegistryEndpoint,
}
impl syn::parse::Parse for EndpointRegistryBinding {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _parenthesis = syn::parenthesized!(content in parse_stream);
        let contract = crate::syn_endpoint_registry_contract::SynEndpointRegistryContract::from(
            content.parse::<syn::Expr>()?,
        );
        let _contract_comma = content.parse::<syn::Token![,]>()?;
        let endpoint = crate::syn_endpoint_registry_endpoint::SynEndpointRegistryEndpoint::from(
            content.parse::<syn::Path>()?,
        );
        Ok(Self::new(contract, endpoint))
    }
}
