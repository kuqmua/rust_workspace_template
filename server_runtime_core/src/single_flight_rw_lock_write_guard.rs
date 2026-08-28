#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::SingleFlightInner;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefMutTarget,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub(super) struct SingleFlightRwLockWriteGuard<'value_lt>(
    pub(super) std::sync::RwLockWriteGuard<'value_lt, SingleFlightInner>,
);
