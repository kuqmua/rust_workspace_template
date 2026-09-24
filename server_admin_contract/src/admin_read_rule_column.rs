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
pub enum AdminReadRuleColumn {
    Id(crate::admin_no_body::AdminNoBody),
    PermissionResourceActionId(crate::admin_no_body::AdminNoBody),
    BasemapId(crate::admin_no_body::AdminNoBody),
    LayerGroupId(crate::admin_no_body::AdminNoBody),
    LayerId(crate::admin_no_body::AdminNoBody),
    ProjectGroupId(crate::admin_no_body::AdminNoBody),
    ProjectId(crate::admin_no_body::AdminNoBody),
    PropertyId(crate::admin_no_body::AdminNoBody),
    RoleId(crate::admin_no_body::AdminNoBody),
    UserId(crate::admin_no_body::AdminNoBody),
    FeatureId(crate::admin_no_body::AdminNoBody),
    ValueItemId(crate::admin_no_body::AdminNoBody),
    CreatedAt(crate::admin_no_body::AdminNoBody),
}
