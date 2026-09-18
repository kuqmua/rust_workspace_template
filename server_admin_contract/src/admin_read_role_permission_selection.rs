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
#[serde(from = "[crate::admin_read_role_permission_column::AdminReadRolePermissionColumn; 4]")]
pub struct AdminReadRolePermissionSelection(
    [crate::admin_read_role_permission_column::AdminReadRolePermissionColumn; 4],
);

impl Default for AdminReadRolePermissionSelection {
    fn default() -> Self {
        Self::from(std::array::from_fn(|index| {
            match index {
            0 => crate::admin_read_role_permission_column::AdminReadRolePermissionColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            1 => crate::admin_read_role_permission_column::AdminReadRolePermissionColumn::RoleId(
                crate::admin_no_body::AdminNoBody,
            ),
            2 => crate::admin_read_role_permission_column::AdminReadRolePermissionColumn::PermissionId(
                crate::admin_no_body::AdminNoBody,
            ),
            _ => crate::admin_read_role_permission_column::AdminReadRolePermissionColumn::CreatedAt(
                crate::admin_no_body::AdminNoBody,
            ),
        }
        }))
    }
}
