#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub struct LeaseRegistry {
    inner: crate::tokio_lease_registry_rw_lock_arc::TokioLeaseRegistryRwLockArc,
}
impl LeaseRegistry {
    pub async fn heartbeat(
        &self,
        id: &crate::lease_id::LeaseId,
    ) -> crate::lease_heartbeat::LeaseHeartbeat {
        {
            let mut inner = self.inner.write().await;
            inner.heartbeat(id, tokio::time::Instant::now())
        }
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn release(
        &self,
        id: &crate::lease_id::LeaseId,
    ) -> crate::lease_heartbeat::LeaseHeartbeat {
        let mut inner = self.inner.write().await;
        inner.release(id)
    }

    pub async fn reserve(
        &self,
        id: crate::lease_id::LeaseId,
        key: crate::lease_key::LeaseKey,
        maximum: crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize,
    ) -> crate::lease_reservation::LeaseReservation {
        {
            let mut inner = self.inner.write().await;
            inner.reserve(id, &key, maximum, tokio::time::Instant::now())
        }
    }

    pub async fn stale(
        &self,
        timeout: crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration,
    ) -> crate::lease_ids::LeaseIds {
        let mut inner = self.inner.write().await;
        inner.stale(tokio::time::Instant::now(), timeout)
    }
}
#[cfg(test)]
mod tests {
    fn id(value: &str) -> crate::lease_id::LeaseId {
        crate::lease_id::LeaseId::try_from(value.to_owned())
            .expect("f1f58adc id invariant must hold")
    }
    fn lease_key(value: &str) -> crate::lease_key::LeaseKey {
        crate::lease_key::LeaseKey::try_from(value.to_owned())
            .expect("699f4283 key invariant must hold")
    }
    fn maximum() -> crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize {
        crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize::from(
            std::num::NonZeroUsize::MIN,
        )
    }

    #[tokio::test]
    async fn reservation_is_unique_by_key_and_limit() {
        let registry = super::LeaseRegistry::new();
        let first_id = id(constants_str::test_fixtures::TEST_LEASE_ID_ONE);
        let first_key = lease_key(constants_str::test_fixtures::TEST_LEASE_KEY_ONE);
        assert_eq!(
            registry
                .reserve(first_id.clone(), first_key.clone(), maximum())
                .await,
            crate::lease_reservation::LeaseReservation::Reserved
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::test_fixtures::TEST_LEASE_ID_TWO),
                    first_key,
                    maximum()
                )
                .await,
            crate::lease_reservation::LeaseReservation::Existing(first_id)
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::test_fixtures::TEST_LEASE_ID_TWO),
                    lease_key(constants_str::test_fixtures::TEST_LEASE_KEY_TWO),
                    maximum(),
                )
                .await,
            crate::lease_reservation::LeaseReservation::LimitReached
        );
    }

    #[tokio::test(start_paused = true)]
    async fn heartbeat_and_stale_transition_are_observable() {
        let registry = super::LeaseRegistry::new();
        let lease_id = id(constants_str::test_fixtures::TEST_LEASE_ID_ONE);
        let _reservation = registry
            .reserve(
                lease_id.clone(),
                lease_key(constants_str::test_fixtures::TEST_LEASE_KEY_ONE),
                maximum(),
            )
            .await;
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Accepted
        );
        tokio::time::advance(std::time::Duration::from_secs(2u64)).await;
        let stale = registry
            .stale(
                crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration::try_from(
                    std::time::Duration::from_secs(1u64),
                )
                .expect(
                    "8cb64054 heartbeat_and_stale_transition_are_observable invariant must hold",
                ),
            )
            .await;
        assert_eq!(stale.as_ref(), std::slice::from_ref(&lease_id));
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Missing
        );
    }
}
