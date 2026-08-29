#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    generate_constructor::New,
)]
pub struct AdminPermissionSummary {
    id: crate::admin_permission_id::AdminPermissionId,
    name: crate::admin_permission_value::AdminPermissionValue,
}
impl AdminPermissionSummary {
    #[must_use]
    pub const fn id(&self) -> crate::admin_permission_id::AdminPermissionId {
        self.id
    }
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &crate::admin_permission_value::AdminPermissionValue {
        &self.name
    }
}
