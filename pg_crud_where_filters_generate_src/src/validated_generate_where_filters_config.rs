#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct ValidatedGenerateWhereFiltersConfig(
    crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
);
