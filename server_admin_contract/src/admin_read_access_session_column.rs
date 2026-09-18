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
pub enum AdminReadAccessSessionColumn {
    CreatedAt(crate::admin_no_body::AdminNoBody),
    ExpiresAt(crate::admin_no_body::AdminNoBody),
    Id(crate::admin_no_body::AdminNoBody),
    RevokedAt(crate::admin_no_body::AdminNoBody),
    UserId(crate::admin_no_body::AdminNoBody),
}
