#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynEndpointRegistryBindings(
    syn::punctuated::Punctuated<
        crate::endpoint_registry_binding::EndpointRegistryBinding,
        syn::Token![,],
    >,
);
