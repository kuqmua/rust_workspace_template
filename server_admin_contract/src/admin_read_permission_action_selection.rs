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
#[serde(from = "[crate::admin_read_permission_action_column::AdminReadPermissionActionColumn; 2]")]
pub struct AdminReadPermissionActionSelection(
    [crate::admin_read_permission_action_column::AdminReadPermissionActionColumn; 2],
);
impl Default for AdminReadPermissionActionSelection {
    fn default() -> Self {
        Self::from([
            crate::admin_read_permission_action_column::AdminReadPermissionActionColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_permission_action_column::AdminReadPermissionActionColumn::Key(
                crate::admin_no_body::AdminNoBody,
            ),
        ])
    }
}
