#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::AsRefOwned,
)]
pub(crate) struct SynRouteRegistryBindings(
    syn::punctuated::Punctuated<
        crate::route_registry_binding::RouteRegistryBinding,
        syn::Token![,],
    >,
);
