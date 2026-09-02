#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct SingleFlight {
    inner: crate::arc_single_flight_rw_lock::ArcSingleFlightRwLock,
    maximum: crate::single_flight_maximum_non_zero_usize::SingleFlightMaximumNonZeroUsize,
}
impl SingleFlight {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // the lock-backed return value cannot be constructed in const context
    pub fn acquire(
        &self,
        single_flight_key: crate::single_flight_key::SingleFlightKey,
    ) -> crate::single_flight_acquire::SingleFlightAcquire {
        let mut inner = crate::write_inner::write_inner(&self.inner);
        if let Some(sender) = inner.get(&single_flight_key) {
            return crate::single_flight_acquire::SingleFlightAcquire::Waiter(
                crate::single_flight_waiter::SingleFlightWaiter::from(
                    crate::tokio_single_flight_receiver::TokioSingleFlightReceiver::from(
                        sender.subscribe(),
                    ),
                ),
            );
        }
        if inner.len().get() >= self.maximum.get() {
            return crate::single_flight_acquire::SingleFlightAcquire::Full;
        }
        let (sender, _) =
            tokio::sync::watch::channel(crate::single_flight_signal::SingleFlightSignal::Running);
        let insertion = inner.try_insert(
            single_flight_key.clone(),
            crate::tokio_single_flight_sender::TokioSingleFlightSender::from(sender),
        );
        if insertion.is_err() {
            return crate::single_flight_acquire::SingleFlightAcquire::Full;
        }
        drop(inner);
        crate::single_flight_acquire::SingleFlightAcquire::Owner(
            crate::single_flight_owner::SingleFlightOwner::new(
                self.inner.clone(),
                Some(single_flight_key),
            ),
        )
    }

    #[must_use]
    pub fn new(
        single_flight_maximum_non_zero_usize: crate::single_flight_maximum_non_zero_usize::SingleFlightMaximumNonZeroUsize,
    ) -> Self {
        Self {
            inner: crate::arc_single_flight_rw_lock::ArcSingleFlightRwLock::default(),
            maximum: single_flight_maximum_non_zero_usize,
        }
    }
}
#[cfg(test)]
mod tests {
    fn single_flight_key() -> crate::single_flight_key::SingleFlightKey {
        crate::single_flight_key::SingleFlightKey::try_from(String::from(
            constants_str::TEST_SINGLE_FLIGHT_KEY,
        ))
        .expect(constants_str::DIAGNOSTIC_68276323)
    }

    #[tokio::test]
    async fn test_one_owner_notifies_waiters_and_releases_key() {
        let flights = super::SingleFlight::new(
            crate::single_flight_maximum_non_zero_usize::SingleFlightMaximumNonZeroUsize::from(
                std::num::NonZeroUsize::MIN,
            ),
        );
        let crate::single_flight_acquire::SingleFlightAcquire::Owner(owner) =
            flights.acquire(single_flight_key())
        else {
            std::panic::panic_any(constants_str::PANIC_7961DD01);
        };
        let crate::single_flight_acquire::SingleFlightAcquire::Waiter(waiter) =
            flights.acquire(single_flight_key())
        else {
            std::panic::panic_any(constants_str::PANIC_5B54E5A1);
        };
        drop(owner);
        assert_eq!(
            waiter.wait().await,
            crate::single_flight_wait_outcome::SingleFlightWaitOutcome::Retry
        );
        assert!(matches!(
            flights.acquire(single_flight_key()),
            crate::single_flight_acquire::SingleFlightAcquire::Owner(_)
        ));
    }
}
