#[path = "lease_registry/lease_entry.rs"]
mod lease_entry;
use lease_entry::*;
#[path = "lease_registry/lease_heartbeat.rs"]
mod lease_heartbeat;
pub use lease_heartbeat::*;
#[path = "lease_registry/lease_id.rs"]
mod lease_id;
pub use lease_id::*;
#[path = "lease_registry/lease_ids.rs"]
mod lease_ids;
pub use lease_ids::*;
#[path = "lease_registry/lease_key.rs"]
mod lease_key;
pub use lease_key::*;
#[path = "lease_registry/lease_registry.rs"]
mod lease_registry;
pub use lease_registry::*;
#[path = "lease_registry/lease_registry_inner.rs"]
mod lease_registry_inner;
use lease_registry_inner::*;
#[path = "lease_registry/lease_registry_maximum_non_zero_usize.rs"]
mod lease_registry_maximum_non_zero_usize;
pub use lease_registry_maximum_non_zero_usize::*;
#[path = "lease_registry/lease_reservation.rs"]
mod lease_reservation;
pub use lease_reservation::*;
#[path = "lease_registry/lease_stale_timeout_duration.rs"]
mod lease_stale_timeout_duration;
pub use lease_stale_timeout_duration::*;
#[path = "lease_registry/lease_state.rs"]
mod lease_state;
pub use lease_state::*;
#[path = "lease_registry/lease_text_error.rs"]
mod lease_text_error;
pub use lease_text_error::*;
#[path = "lease_registry/lease_text_maximum_bytes.rs"]
mod lease_text_maximum_bytes;
use lease_text_maximum_bytes::*;
#[path = "lease_registry/lease_text_ref.rs"]
mod lease_text_ref;
use lease_text_ref::*;
#[path = "lease_registry/std_lease_stale_timeout_error.rs"]
mod std_lease_stale_timeout_error;
pub use std_lease_stale_timeout_error::*;
#[path = "lease_registry/tokio_lease_instant.rs"]
mod tokio_lease_instant;
use tokio_lease_instant::*;
#[path = "lease_registry/tokio_lease_registry_rw_lock_arc.rs"]
mod tokio_lease_registry_rw_lock_arc;
use tokio_lease_registry_rw_lock_arc::*;
#[path = "lease_registry/validate_lease_text.rs"]
mod validate_lease_text;
use validate_lease_text::*;

#[cfg(test)]
mod tests {
    fn id(value: &str) -> super::LeaseId {
        super::LeaseId::try_from(value.to_owned()).expect("f1f58adc id invariant must hold")
    }
    fn key(value: &str) -> super::LeaseKey {
        super::LeaseKey::try_from(value.to_owned()).expect("699f4283 key invariant must hold")
    }
    fn maximum() -> super::LeaseRegistryMaximumNonZeroUsize {
        super::LeaseRegistryMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN)
    }

    #[tokio::test]
    async fn reservation_is_unique_by_key_and_limit() {
        let registry = super::LeaseRegistry::new();
        let first_id = id(constants_str::TEST_LEASE_ID_ONE);
        let first_key = key(constants_str::TEST_LEASE_KEY_ONE);
        assert_eq!(
            registry
                .reserve(first_id.clone(), first_key.clone(), maximum())
                .await,
            super::LeaseReservation::Reserved
        );
        assert_eq!(
            registry
                .reserve(id(constants_str::TEST_LEASE_ID_TWO), first_key, maximum())
                .await,
            super::LeaseReservation::Existing(first_id)
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::TEST_LEASE_ID_TWO),
                    key(constants_str::TEST_LEASE_KEY_TWO),
                    maximum(),
                )
                .await,
            super::LeaseReservation::LimitReached
        );
    }

    #[tokio::test(start_paused = true)]
    async fn heartbeat_and_stale_transition_are_observable() {
        let registry = super::LeaseRegistry::new();
        let lease_id = id(constants_str::TEST_LEASE_ID_ONE);
        let _reservation = registry
            .reserve(
                lease_id.clone(),
                key(constants_str::TEST_LEASE_KEY_ONE),
                maximum(),
            )
            .await;
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            super::LeaseHeartbeat::Accepted
        );
        tokio::time::advance(std::time::Duration::from_secs(2u64)).await;
        let stale = registry
            .stale(
                super::LeaseStaleTimeoutDuration::try_from(std::time::Duration::from_secs(1u64)).expect(
                    "8cb64054 heartbeat_and_stale_transition_are_observable invariant must hold",
                ),
            )
            .await;
        assert_eq!(stale.as_ref(), std::slice::from_ref(&lease_id));
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            super::LeaseHeartbeat::Missing
        );
    }
}
