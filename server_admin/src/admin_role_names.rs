#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::ADMIN_AUTH_COLLECTION_MAX_LEN;
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
pub(crate) struct AdminRoleNames(
    pub(crate)  bounded_types::domain_types::vector::BoundedVec<
        crate::AdminRoleName,
        0,
        { ADMIN_AUTH_COLLECTION_MAX_LEN },
    >,
);
