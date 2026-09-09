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
pub enum AdminReadUserColumn {
    DisplayName(crate::admin_no_body::AdminNoBody),
    Id(crate::admin_no_body::AdminNoBody),
    IsBanned(crate::admin_no_body::AdminNoBody),
    Login(crate::admin_no_body::AdminNoBody),
}
