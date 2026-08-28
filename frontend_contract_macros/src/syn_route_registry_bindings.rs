use crate::domain_types::RouteRegistryBinding;

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynRouteRegistryBindings(
    syn::punctuated::Punctuated<RouteRegistryBinding, syn::Token![,]>,
);
