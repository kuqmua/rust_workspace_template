#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
    proc_macro_getters::Getters,
)]
pub struct AdminRolesReadRow {
    #[serde(flatten)]
    admin_roles_read: crate::admin_roles::AdminRolesRead,
    permission_ids: server_admin_contract::admin_permission_ids::AdminPermissionIds,
}
