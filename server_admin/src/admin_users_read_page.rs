#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
    proc_macro_getters::Getters,
)]
pub struct AdminUsersReadPage {
    #[schema(value_type = Vec<crate::admin_users_read_row::AdminUsersReadRow>)]
    items: pg_crud_common::list_items::ListItems<crate::admin_users_read_row::AdminUsersReadRow>,
    roles: server_admin_contract::admin_role_summaries::AdminRoleSummaries,
    total: pg_crud_common::list_total::ListTotal,
}
