#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataRolePermissionsFlt(
    pub(super) crate::admin_role_permissions::StdOptionalOptionalAdminRolePermissionsWhereMany,
);
