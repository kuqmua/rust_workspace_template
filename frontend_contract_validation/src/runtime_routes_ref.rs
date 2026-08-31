#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct RuntimeRoutesRef<'value_lt>(
    &'value_lt [frontend_contract::route_metadata::RouteMetadata],
);
