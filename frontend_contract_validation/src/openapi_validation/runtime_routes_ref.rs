#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct RuntimeRoutesRef<'value_lt>(
    pub(super) &'value_lt [frontend_contract::domain_types::RouteMetadata],
);
