#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct EndpointRegistryArgs {
    bindings: crate::syn_endpoint_registry_bindings::SynEndpointRegistryBindings,
    state: crate::syn_endpoint_registry_state::SynEndpointRegistryState,
}
impl syn::parse::Parse for EndpointRegistryArgs {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let state_name = parse_stream.parse::<syn::Ident>()?;
        if state_name != constants_str::STATE {
            return Err(syn::Error::new_spanned(
                state_name,
                constants_str::ENDPOINT_REGISTRY_REQUIRES_STATE,
            ));
        }
        let _equals = parse_stream.parse::<syn::Token![=]>()?;
        let state = crate::syn_endpoint_registry_state::SynEndpointRegistryState::from(
            parse_stream.parse::<syn::Type>()?,
        );
        let _semicolon = parse_stream.parse::<syn::Token![;]>()?;
        let bindings = syn::punctuated::Punctuated::<
            crate::endpoint_registry_binding::EndpointRegistryBinding,
            syn::Token![,],
        >::parse_terminated(parse_stream)?;
        if bindings.is_empty() {
            return Err(parse_stream.error(constants_str::ENDPOINT_REGISTRY_REQUIRES_BINDING));
        }
        Ok(Self::new(
            crate::syn_endpoint_registry_bindings::SynEndpointRegistryBindings::from(bindings),
            state,
        ))
    }
}
