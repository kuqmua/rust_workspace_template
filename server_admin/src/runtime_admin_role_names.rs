#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    newtype::UtoipaSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
#[serde(transparent)]
#[derive(generate_accessor::Getters)]
pub(crate) struct RuntimeAdminRoleNames(
    bounded_types::bounded_vec::BoundedVec<
        server_admin_contract::admin_role_name::AdminRoleName,
        0,
        { crate::admin_auth_collection_max_len::ADMIN_AUTH_COLLECTION_MAX_LEN },
    >,
);
