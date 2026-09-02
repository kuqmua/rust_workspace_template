#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::Display,
    proc_macro_newtype::ToTokens,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct FilterSqlOperator(&'static str);
