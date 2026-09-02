#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub struct RuntimeRoutesRef<'value_lt>(
    &'value_lt [frontend_contract::route_metadata::RouteMetadata],
);
