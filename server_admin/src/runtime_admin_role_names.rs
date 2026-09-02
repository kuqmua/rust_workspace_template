#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    proc_macro_newtype::UtoipaSchema,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
#[serde(transparent)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct RuntimeAdminRoleNames(
    bounded_types::bounded_vec::BoundedVec<
        server_admin_contract::admin_role_name::AdminRoleName,
        0,
        { crate::admin_auth_collection_max_len::ADMIN_AUTH_COLLECTION_MAX_LEN },
    >,
);
