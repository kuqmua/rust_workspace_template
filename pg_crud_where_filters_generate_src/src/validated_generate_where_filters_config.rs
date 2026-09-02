#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct ValidatedGenerateWhereFiltersConfig(
    crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
);
