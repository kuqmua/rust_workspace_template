#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::ADMIN_AUTH_COLLECTION_MAX_LEN;
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
    pub(super)  bounded_types::domain_types::vector::BoundedVec<
        super::super::AdminPermission,
        0,
        { ADMIN_AUTH_COLLECTION_MAX_LEN },
    >,
);
