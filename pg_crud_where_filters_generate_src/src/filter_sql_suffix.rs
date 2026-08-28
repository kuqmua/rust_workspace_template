#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::ToTokens,
    newtype::FromInner,
)]
pub(crate) struct FilterSqlSuffix(&'static str);
