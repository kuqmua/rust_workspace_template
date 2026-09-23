#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_newtype_from_inner::FromInner,
    utoipa::ToSchema,
)]
#[serde(
    from = "[crate::admin_read_permission_resource_column::AdminReadPermissionResourceColumn; 2]"
)]
pub struct AdminReadPermissionResourceSelection(
    [crate::admin_read_permission_resource_column::AdminReadPermissionResourceColumn; 2],
);
impl AdminReadPermissionResourceSelection {
    const DEFAULT_COLUMNS:
        [crate::admin_read_permission_resource_column::AdminReadPermissionResourceColumn; 2] = [
        crate::admin_read_permission_resource_column::AdminReadPermissionResourceColumn::Id(
            crate::admin_no_body::AdminNoBody,
        ),
        crate::admin_read_permission_resource_column::AdminReadPermissionResourceColumn::Key(
            crate::admin_no_body::AdminNoBody,
        ),
    ];
}
impl Default for AdminReadPermissionResourceSelection {
    fn default() -> Self {
        Self::from(Self::DEFAULT_COLUMNS)
    }
}
