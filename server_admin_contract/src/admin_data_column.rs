use super::{AdminDataFilter, AdminDataFilters};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AdminDataColumn {
    filters: AdminDataFilters,
    label: crate::domain_types::AdminText,
    name: crate::domain_types::AdminText,
    input_kind: frontend_contract::InputKind,
}
impl AdminDataColumn {
    #[must_use]
    pub const fn new(
        filters: AdminDataFilters,
        input_kind: frontend_contract::InputKind,
        label: crate::domain_types::AdminText,
        name: crate::domain_types::AdminText,
    ) -> Self {
        Self {
            filters,
            label,
            name,
            input_kind,
        }
    }
    #[must_use]
    pub const fn filters(&self) -> &[AdminDataFilter] {
        self.filters.as_slice()
    }
    #[must_use]
    pub const fn input_kind(&self) -> frontend_contract::InputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn label(&self) -> &crate::domain_types::AdminText {
        &self.label
    }
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &crate::domain_types::AdminText {
        &self.name
    }
}
