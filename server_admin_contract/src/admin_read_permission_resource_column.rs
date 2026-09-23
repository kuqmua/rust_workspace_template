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
pub enum AdminReadPermissionResourceColumn {
    Id(crate::admin_no_body::AdminNoBody),
    Key(crate::admin_no_body::AdminNoBody),
}
