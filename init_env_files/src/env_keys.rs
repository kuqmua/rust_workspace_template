#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub(crate) struct EnvKeys(pub(super) bounded_types::BoundedVec<crate::EnvKey, 0, { usize::MAX }>);
