#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct TokioSingleFlightReceiver(
    pub(super) tokio::sync::watch::Receiver<crate::single_flight_signal::SingleFlightSignal>,
);
