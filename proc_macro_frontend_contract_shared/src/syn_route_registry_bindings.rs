#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
)]
pub(crate) struct SynRouteRegistryBindings(
    syn::punctuated::Punctuated<
        crate::route_registry_binding::RouteRegistryBinding,
        syn::Token![,],
    >,
);
