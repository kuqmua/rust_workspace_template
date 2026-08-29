#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AdminDataColumn {
    filters: crate::admin_data_filters::AdminDataFilters,
    label: crate::admin_text::AdminText,
    name: crate::admin_text::AdminText,
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
    #[must_use]
    pub const fn input_kind(&self) -> frontend_contract::input_kind::InputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn label(&self) -> &crate::admin_text::AdminText {
        &self.label
    }
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &crate::admin_text::AdminText {
        &self.name
    }
}
