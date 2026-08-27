#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::Display,
    newtype::ToTokens,
    newtype::FromInner,
)]
pub(crate) struct FilterSqlOperator(&'static str);
