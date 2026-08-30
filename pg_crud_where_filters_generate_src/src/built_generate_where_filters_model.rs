#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct BuiltGenerateWhereFiltersModel {
    config: crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
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

impl BuiltGenerateWhereFiltersModel {
    #[must_use]
    pub(crate) const fn into_parts(
        self,
    ) -> (
        crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
        crate::filter_spec_valid::FilterSpecValid,
    ) {
        (self.config, self.contract_valid)
    }
}
