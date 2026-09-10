#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
    proc_macro_getters::Getters,
)]
pub struct AdminRolesReadPage {
    #[schema(value_type = Vec<crate::admin_roles_read_row::AdminRolesReadRow>)]
    items: pg_crud_common::list_items::ListItems<crate::admin_roles_read_row::AdminRolesReadRow>,
    permissions: server_admin_contract::admin_permission_summaries::AdminPermissionSummaries,
    total: pg_crud_common::list_total::ListTotal,
}
