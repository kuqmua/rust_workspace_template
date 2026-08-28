use crate::domain_types::EndpointRegistryBinding;

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynEndpointRegistryBindings(
    syn::punctuated::Punctuated<EndpointRegistryBinding, syn::Token![,]>,
);
