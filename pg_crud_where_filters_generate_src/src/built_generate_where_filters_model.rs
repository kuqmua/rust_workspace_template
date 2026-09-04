#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_getters::Getters,
)]
#[getters(bare)]
pub struct BuiltGenerateWhereFiltersModel {
    #[getters(copy)]
    config: crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
    #[getters(copy)]
    contract_valid: crate::filter_spec_valid::FilterSpecValid,
}

impl
    From<(
        crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
        crate::filter_spec_valid::FilterSpecValid,
    )> for BuiltGenerateWhereFiltersModel
{
    fn from(
        value: (
            crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
            crate::filter_spec_valid::FilterSpecValid,
        ),
    ) -> Self {
        Self {
            config: value.0,
            contract_valid: value.1,
        }
    }
}
