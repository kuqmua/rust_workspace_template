#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::AsRefOwned,
)]
pub(crate) struct SynEndpointRegistryBindings(
    syn::punctuated::Punctuated<
        crate::endpoint_registry_binding::EndpointRegistryBinding,
        syn::Token![,],
    >,
);
