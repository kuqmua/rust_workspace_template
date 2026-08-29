#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, newtype::FromInner)]
pub(super) struct ArcSingleFlightRwLock(
    pub(super) std::sync::Arc<std::sync::RwLock<crate::single_flight_inner::SingleFlightInner>>,
);
