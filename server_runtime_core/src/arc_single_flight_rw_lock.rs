#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::SingleFlightInner;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, newtype::FromInner)]
pub(super) struct ArcSingleFlightRwLock(
    pub(super) std::sync::Arc<std::sync::RwLock<SingleFlightInner>>,
);
