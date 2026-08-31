#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    newtype::FromInner,
    newtype::IntoInner,
    generate_accessor::Getters,
)]
pub(crate) struct DataPermissionsFlt(
    crate::admin_permissions::StdOptionalOptionalAdminPermissionsWhereMany,
);
