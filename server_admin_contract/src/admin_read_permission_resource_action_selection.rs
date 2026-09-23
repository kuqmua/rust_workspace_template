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
    from = "[crate::admin_read_permission_resource_action_column::AdminReadPermissionResourceActionColumn; 3]"
)]
pub struct AdminReadPermissionResourceActionSelection(
    [crate::admin_read_permission_resource_action_column::AdminReadPermissionResourceActionColumn;
        3],
);

impl Default for AdminReadPermissionResourceActionSelection {
    fn default() -> Self {
        Self::from([
            crate::admin_read_permission_resource_action_column::AdminReadPermissionResourceActionColumn::Id(crate::admin_no_body::AdminNoBody),
            crate::admin_read_permission_resource_action_column::AdminReadPermissionResourceActionColumn::PermissionResourceId(crate::admin_no_body::AdminNoBody),
            crate::admin_read_permission_resource_action_column::AdminReadPermissionResourceActionColumn::PermissionActionId(crate::admin_no_body::AdminNoBody),
        ])
    }
}
