#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistryBindings(
    syn::punctuated::Punctuated<
        crate::route_registry_binding::RouteRegistryBinding,
        syn::Token![,],
    >,
);
