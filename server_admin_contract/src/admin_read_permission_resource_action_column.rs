#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminReadPermissionResourceActionColumn {
    Id(crate::admin_no_body::AdminNoBody),
    PermissionActionId(crate::admin_no_body::AdminNoBody),
    PermissionResourceId(crate::admin_no_body::AdminNoBody),
}
