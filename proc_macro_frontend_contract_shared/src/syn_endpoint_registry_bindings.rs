#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
)]
pub(crate) struct SynEndpointRegistryBindings(
    syn::punctuated::Punctuated<
        crate::endpoint_registry_binding::EndpointRegistryBinding,
        syn::Token![,],
    >,
);
