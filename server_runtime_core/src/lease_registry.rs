#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub struct LeaseRegistry {
    inner: crate::tokio_lease_registry_rw_lock_arc::TokioLeaseRegistryRwLockArc,
}
impl LeaseRegistry {
    pub async fn heartbeat(
        &self,
        lease_id: &crate::lease_id::LeaseId,
    ) -> crate::lease_heartbeat::LeaseHeartbeat {
        {
            let mut inner = self.inner.write().await;
            inner.heartbeat(lease_id, tokio::time::Instant::now())
        }
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn release(
        &self,
        lease_id: &crate::lease_id::LeaseId,
    ) -> crate::lease_heartbeat::LeaseHeartbeat {
        let mut inner = self.inner.write().await;
        inner.release(lease_id)
    }

    pub async fn reserve(
        &self,
        lease_id: crate::lease_id::LeaseId,
        lease_key: crate::lease_key::LeaseKey,
        lease_registry_maximum_non_zero_usize: crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize,
    ) -> crate::lease_reservation::LeaseReservation {
        {
            let mut inner = self.inner.write().await;
            inner.reserve(
                lease_id,
                &lease_key,
                lease_registry_maximum_non_zero_usize,
                tokio::time::Instant::now(),
            )
        }
    }

    pub async fn stale(
        &self,
        lease_stale_timeout_duration: crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration,
    ) -> crate::lease_ids::LeaseIds {
        let mut inner = self.inner.write().await;
        inner.stale(tokio::time::Instant::now(), lease_stale_timeout_duration)
    }
}
#[cfg(test)]
mod tests {
    fn id(str: &str) -> crate::lease_id::LeaseId {
        crate::lease_id::LeaseId::try_from(str.to_owned())
            .expect(constants_str::DIAGNOSTIC_F1F58ADC)
    }
    fn lease_key(str: &str) -> crate::lease_key::LeaseKey {
        crate::lease_key::LeaseKey::try_from(str.to_owned())
            .expect(constants_str::DIAGNOSTIC_699F4283)
    }
    fn maximum() -> crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize {
        crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize::from(
            std::num::NonZeroUsize::MIN,
        )
    }

    #[tokio::test]
    async fn test_reservation_is_unique_by_key_and_limit() {
        let registry = super::LeaseRegistry::new();
        let first_id = id(constants_str::TEST_LEASE_ID_ONE);
        let first_key = lease_key(constants_str::TEST_LEASE_KEY_ONE);
        assert_eq!(
            registry
                .reserve(first_id.clone(), first_key.clone(), maximum())
                .await,
            crate::lease_reservation::LeaseReservation::Reserved
        );
        assert_eq!(
            registry
                .reserve(id(constants_str::TEST_LEASE_ID_TWO), first_key, maximum())
                .await,
            crate::lease_reservation::LeaseReservation::Existing(first_id)
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::TEST_LEASE_ID_TWO),
                    lease_key(constants_str::TEST_LEASE_KEY_TWO),
                    maximum(),
                )
                .await,
            crate::lease_reservation::LeaseReservation::LimitReached
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_rebinding_existing_lease_at_capacity_preserves_indexes() {
        let registry = super::LeaseRegistry::new();
        let lease_id = id(constants_str::TEST_LEASE_ID_ONE);
        assert_eq!(
            registry
                .reserve(
                    lease_id.clone(),
                    lease_key(constants_str::TEST_LEASE_KEY_ONE),
                    maximum(),
                )
                .await,
            crate::lease_reservation::LeaseReservation::Reserved
        );
        assert_eq!(
            registry
                .reserve(
                    lease_id.clone(),
                    lease_key(constants_str::TEST_LEASE_KEY_TWO),
                    maximum(),
                )
                .await,
            crate::lease_reservation::LeaseReservation::Reserved
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::TEST_LEASE_ID_TWO),
                    lease_key(constants_str::TEST_LEASE_KEY_TWO),
                    maximum(),
                )
                .await,
            crate::lease_reservation::LeaseReservation::Existing(lease_id.clone())
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::TEST_LEASE_ID_TWO),
                    lease_key(constants_str::TEST_LEASE_KEY_ONE),
                    maximum(),
                )
                .await,
            crate::lease_reservation::LeaseReservation::LimitReached
        );
        assert_eq!(
            registry.release(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Accepted
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::TEST_LEASE_ID_TWO),
                    lease_key(constants_str::TEST_LEASE_KEY_TWO),
                    maximum(),
                )
                .await,
            crate::lease_reservation::LeaseReservation::Reserved
        );
    }

    fn lease_registry_timeout_fixture()
    -> crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration {
        crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration::try_from(
            std::time::Duration::from_secs(1u64),
        )
        .expect(constants_str::DIAGNOSTIC_8CB64054)
    }

    #[tokio::test(start_paused = true)]
    async fn test_heartbeat_and_stale_transition_are_observable() {
        let registry = super::LeaseRegistry::new();
        let lease_id = id(constants_str::TEST_LEASE_ID_ONE);
        let _reservation = registry
            .reserve(
                lease_id.clone(),
                lease_key(constants_str::TEST_LEASE_KEY_ONE),
                maximum(),
            )
            .await;
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Accepted
        );
        tokio::time::advance(std::time::Duration::from_secs(2u64)).await;
        let stale = registry.stale(lease_registry_timeout_fixture()).await;
        assert_eq!(stale.as_ref(), std::slice::from_ref(&lease_id));
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Missing
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_lease_expiration_boundary_and_heartbeat_refresh_preserve_admission() {
        let timeout = lease_registry_timeout_fixture();
        let registry = super::LeaseRegistry::new();
        let lease_id = id(constants_str::TEST_LEASE_ID_ONE);
        assert_eq!(
            registry
                .reserve(
                    lease_id.clone(),
                    lease_key(constants_str::TEST_LEASE_KEY_ONE),
                    maximum()
                )
                .await,
            crate::lease_reservation::LeaseReservation::Reserved
        );
        tokio::time::advance(std::time::Duration::from_secs(1u64)).await;
        assert!(registry.stale(timeout).await.as_ref().is_empty());
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Accepted
        );
        tokio::time::advance(std::time::Duration::from_secs(1u64)).await;
        assert!(registry.stale(timeout).await.as_ref().is_empty());
        tokio::time::advance(std::time::Duration::from_nanos(1u64)).await;
        assert_eq!(
            registry.stale(timeout).await.as_ref(),
            std::slice::from_ref(&lease_id)
        );
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Missing
        );
        assert_eq!(
            registry.release(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Accepted
        );
        assert!(registry.stale(timeout).await.as_ref().is_empty());
        assert_eq!(
            registry.release(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Missing
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_expired_lease_replacement_reclaims_capacity_and_removes_previous_identity() {
        let timeout = lease_registry_timeout_fixture();
        let registry = super::LeaseRegistry::new();
        let previous_id = id(constants_str::TEST_LEASE_ID_ONE);
        let replacement_id = id(constants_str::TEST_LEASE_ID_TWO);
        assert_eq!(
            registry
                .reserve(
                    previous_id.clone(),
                    lease_key(constants_str::TEST_LEASE_KEY_ONE),
                    maximum()
                )
                .await,
            crate::lease_reservation::LeaseReservation::Reserved
        );
        tokio::time::advance(std::time::Duration::from_secs(2u64)).await;
        assert_eq!(
            registry.stale(timeout).await.as_ref(),
            std::slice::from_ref(&previous_id)
        );
        assert_eq!(
            registry
                .reserve(
                    replacement_id.clone(),
                    lease_key(constants_str::TEST_LEASE_KEY_ONE),
                    maximum()
                )
                .await,
            crate::lease_reservation::LeaseReservation::Reserved
        );
        assert_eq!(
            registry.heartbeat(&previous_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Missing
        );
        assert_eq!(
            registry.heartbeat(&replacement_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Accepted
        );
        assert!(registry.stale(timeout).await.as_ref().is_empty());
        assert_eq!(
            registry
                .reserve(
                    previous_id,
                    lease_key(constants_str::TEST_LEASE_KEY_TWO),
                    maximum()
                )
                .await,
            crate::lease_reservation::LeaseReservation::LimitReached
        );
        assert_eq!(
            registry.release(&replacement_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Accepted
        );
    }
}
