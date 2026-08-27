#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::FromInner)]
pub(crate) struct DataPermissionsFlt(
    pub(super) crate::domain_types::generated_tables::StdOptionalOptionalAdminPermissionsWhereMany,
);
