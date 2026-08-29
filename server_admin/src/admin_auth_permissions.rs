#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    newtype::AsRefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
#[serde(transparent)]
pub(crate) struct AdminAuthPermissions(
    pub(crate)  bounded_types::bounded_vec::BoundedVec<
        server_admin_contract::admin_permission::AdminPermission,
        0,
        { crate::admin_auth_collection_max_len::ADMIN_AUTH_COLLECTION_MAX_LEN },
    >,
);
