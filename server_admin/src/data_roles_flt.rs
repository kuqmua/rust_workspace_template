#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct DataRolesFlt(crate::admin_roles::StdOptionalOptionalAdminRolesWhereMany);
