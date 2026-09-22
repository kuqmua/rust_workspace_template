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
#[serde(from = "[crate::admin_read_permission_column::AdminReadPermissionColumn; 3]")]
pub struct AdminReadPermissionSelection(
    [crate::admin_read_permission_column::AdminReadPermissionColumn; 3],
);
impl Default for AdminReadPermissionSelection {
    fn default() -> Self {
        Self::from([
            crate::admin_read_permission_column::AdminReadPermissionColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_permission_column::AdminReadPermissionColumn::Name(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_permission_column::AdminReadPermissionColumn::CreatedAt(
                crate::admin_no_body::AdminNoBody,
            ),
        ])
    }
}
