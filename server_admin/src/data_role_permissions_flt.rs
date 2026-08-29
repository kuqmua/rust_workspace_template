use super::domain_types::generated_tables::StdOptionalOptionalAdminRolePermissionsWhereMany;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataRolePermissionsFlt(
    pub(crate) StdOptionalOptionalAdminRolePermissionsWhereMany,
);
