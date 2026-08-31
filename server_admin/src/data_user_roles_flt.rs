#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    newtype::FromInner,
    newtype::IntoInner,
    generate_accessor::Getters,
)]
pub(crate) struct DataUserRolesFlt(
    crate::admin_user_roles::StdOptionalOptionalAdminUserRolesWhereMany,
);
