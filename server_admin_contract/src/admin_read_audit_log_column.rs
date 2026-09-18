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
pub enum AdminReadAuditLogColumn {
    Action(crate::admin_no_body::AdminNoBody),
    CreatedAt(crate::admin_no_body::AdminNoBody),
    Id(crate::admin_no_body::AdminNoBody),
    RequestId(crate::admin_no_body::AdminNoBody),
    Resource(crate::admin_no_body::AdminNoBody),
    ResourceId(crate::admin_no_body::AdminNoBody),
    Succeeded(crate::admin_no_body::AdminNoBody),
    UserId(crate::admin_no_body::AdminNoBody),
    UserLogin(crate::admin_no_body::AdminNoBody),
}
