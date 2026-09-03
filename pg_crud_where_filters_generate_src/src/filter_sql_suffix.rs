#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_as_ref_inner::AsRefInner,
    proc_macro_newtype_to_tokens::ToTokens,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct FilterSqlSuffix(&'static str);
