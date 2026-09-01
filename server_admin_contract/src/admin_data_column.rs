#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AdminDataColumn {
    #[getters(skip)]
    filters: crate::admin_data_filters::AdminDataFilters,
    label: crate::admin_text::AdminText,
    name: crate::admin_text::AdminText,
    #[getters(copy)]
    input_kind: frontend_contract::input_kind::InputKind,
}
impl AdminDataColumn {
    #[must_use]
    pub const fn new(
        filters: crate::admin_data_filters::AdminDataFilters,
        input_kind: frontend_contract::input_kind::InputKind,
        label: crate::admin_text::AdminText,
        name: crate::admin_text::AdminText,
    ) -> Self {
        Self {
            filters,
            label,
            name,
            input_kind,
        }
    }
    #[must_use]
    pub const fn filters(&self) -> &[crate::admin_data_filter::AdminDataFilter] {
        self.filters.as_slice()
    }
}
