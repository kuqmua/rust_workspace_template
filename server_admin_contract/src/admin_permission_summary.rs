#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AdminPermissionSummary {
    id: crate::domain_types::AdminPermissionId,
    name: crate::domain_types::AdminPermissionValue,
}
impl AdminPermissionSummary {
    #[must_use]
    pub const fn new(
        id: crate::domain_types::AdminPermissionId,
        name: crate::domain_types::AdminPermissionValue,
    ) -> Self {
        Self { id, name }
    }
    #[must_use]
    pub const fn id(&self) -> crate::domain_types::AdminPermissionId {
        self.id
    }
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &crate::domain_types::AdminPermissionValue {
        &self.name
    }
}
