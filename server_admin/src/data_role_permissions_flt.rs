#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    newtype::FromInner,
    newtype::IntoInner,
    generate_accessor::Getters,
)]
pub(crate) struct DataRolePermissionsFlt(
    crate::admin_role_permissions::StdOptionalOptionalAdminRolePermissionsWhereMany,
);
