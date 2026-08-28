#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[path = "lease_entry.rs"]
mod lease_entry;
use lease_entry::LeaseEntry;
#[path = "lease_heartbeat.rs"]
mod lease_heartbeat;
pub use lease_heartbeat::*;
#[path = "lease_id.rs"]
mod lease_id;
pub use lease_id::*;
#[path = "lease_ids.rs"]
mod lease_ids;
pub use lease_ids::*;
#[path = "lease_key.rs"]
mod lease_key;
pub use lease_key::*;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub struct LeaseRegistry {
    inner: TokioLeaseRegistryRwLockArc,
}
impl LeaseRegistry {
    pub async fn heartbeat(&self, id: &LeaseId) -> LeaseHeartbeat {
        {
            let mut inner = self.inner.0.write().await;
            match inner.by_id.get_mut(id) {
                Some(entry) if entry.state != LeaseState::Stale => {
                    entry.heartbeat = TokioLeaseInstant::from(tokio::time::Instant::now());
                    entry.state = LeaseState::Ready;
                    LeaseHeartbeat::Accepted
                }
                Some(_) | None => LeaseHeartbeat::Missing,
            }
        }
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn release(&self, id: &LeaseId) -> LeaseHeartbeat {
        let mut inner = self.inner.0.write().await;
        let Some(entry) = inner.by_id.remove(id) else {
            return LeaseHeartbeat::Missing;
        };
        let _removed = inner.by_key.remove(&entry.key);
        LeaseHeartbeat::Accepted
    }

    pub async fn reserve(
        &self,
        id: LeaseId,
        key: LeaseKey,
        maximum: LeaseRegistryMaximumNonZeroUsize,
    ) -> LeaseReservation {
        {
            let mut inner = self.inner.0.write().await;
            if let Some(existing_id) = inner.by_key.get(&key)
                && inner
                    .by_id
                    .get(existing_id)
                    .is_some_and(|entry| entry.state != LeaseState::Stale)
            {
                return LeaseReservation::Existing(existing_id.clone());
            }
            #[allow(
                clippy::needless_collect,
                reason = "expired lease keys must be collected before mutating the registry"
            )]
            // ids must be owned before mutating both registry indexes
            let stale = inner
                .by_id
                .iter()
                .filter(|(_id, entry)| entry.state == LeaseState::Stale)
                .map(|(stale_id, _entry)| stale_id.clone())
                .collect::<Vec<_>>();
            stale.into_iter().fold((), |(), stale_id| {
                if let Some(entry) = inner.by_id.remove(&stale_id) {
                    let _removed = inner.by_key.remove(&entry.key);
                }
            });
            if inner.by_id.len().get() >= maximum.0.get() {
                return LeaseReservation::LimitReached;
            }
            if let Some(previous) = inner.by_id.remove(&id) {
                let _removed = inner.by_key.remove(&previous.key);
            }
            if let Some(previous_id) = inner.by_key.remove(&key) {
                let _removed = inner.by_id.remove(&previous_id);
            }
            let id_insertion = inner.by_key.try_insert(key.clone(), id.clone());
            if id_insertion.is_err() {
                return LeaseReservation::LimitReached;
            }
            let entry_insertion = inner.by_id.try_insert(
                id,
                LeaseEntry {
                    heartbeat: TokioLeaseInstant::from(tokio::time::Instant::now()),
                    key: key.clone(),
                    state: LeaseState::Reserved,
                },
            );
            if entry_insertion.is_err() {
                let _removed_id = inner.by_key.remove(&key);
                drop(inner);
                return LeaseReservation::LimitReached;
            }
            drop(inner);
            LeaseReservation::Reserved
        }
    }

    pub async fn stale(&self, timeout: LeaseStaleTimeoutDuration) -> LeaseIds {
        LeaseIds::from({
            let mut inner = self.inner.0.write().await;
            let now = tokio::time::Instant::now();
            let mut stale_ids = bounded_types::domain_types::vector::BoundedVec::default();
            inner
                .by_id
                .iter_mut()
                .filter_map(|(id, entry)| {
                    (now.duration_since(entry.heartbeat.0) > timeout.0).then(|| {
                        entry.state = LeaseState::Stale;
                        id.clone()
                    })
                })
                .for_each(|id| stale_ids.push_max_capacity(id));
            stale_ids
        })
    }
}
#[path = "lease_registry_inner.rs"]
mod lease_registry_inner;
use lease_registry_inner::LeaseRegistryInner;
#[path = "lease_registry_maximum_non_zero_usize.rs"]
mod lease_registry_maximum_non_zero_usize;
pub use lease_registry_maximum_non_zero_usize::*;
#[path = "lease_reservation.rs"]
mod lease_reservation;
pub use lease_reservation::*;
#[path = "lease_stale_timeout_duration.rs"]
mod lease_stale_timeout_duration;
pub use lease_stale_timeout_duration::*;
#[path = "lease_state.rs"]
mod lease_state;
pub use lease_state::*;
#[path = "lease_text_error.rs"]
mod lease_text_error;
pub use lease_text_error::*;
#[path = "lease_text_maximum_bytes.rs"]
mod lease_text_maximum_bytes;
use lease_text_maximum_bytes::LEASE_TEXT_MAXIMUM_BYTES;
#[path = "lease_text_ref.rs"]
mod lease_text_ref;
use lease_text_ref::LeaseTextRef;
#[path = "std_lease_stale_timeout_error.rs"]
mod std_lease_stale_timeout_error;
pub use std_lease_stale_timeout_error::*;
#[path = "tokio_lease_instant.rs"]
mod tokio_lease_instant;
use tokio_lease_instant::TokioLeaseInstant;
#[path = "tokio_lease_registry_rw_lock_arc.rs"]
mod tokio_lease_registry_rw_lock_arc;
use tokio_lease_registry_rw_lock_arc::TokioLeaseRegistryRwLockArc;
#[path = "validate_lease_text.rs"]
mod validate_lease_text;
use validate_lease_text::validate_lease_text;

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
