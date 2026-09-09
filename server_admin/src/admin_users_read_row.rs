#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
    proc_macro_getters::Getters,
)]
pub struct AdminUsersReadRow {
    #[serde(flatten)]
    admin_users_read: crate::admin_users::AdminUsersRead,
    role_ids: server_admin_contract::admin_role_ids::AdminRoleIds,
}
