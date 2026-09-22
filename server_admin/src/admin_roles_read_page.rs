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
    #[schema(value_type = Vec<crate::admin_roles::AdminRolesRead>)]
    items: pg_crud_common::list_items::ListItems<crate::admin_roles::AdminRolesRead>,
    total: pg_crud_common::list_total::ListTotal,
}
