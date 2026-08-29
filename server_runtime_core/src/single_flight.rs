#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct SingleFlight {
    inner: crate::arc_single_flight_rw_lock::ArcSingleFlightRwLock,
    maximum: crate::single_flight_maximum_non_zero_usize::SingleFlightMaximumNonZeroUsize,
}
impl SingleFlight {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // the lock-backed return value cannot be constructed in const context
    pub fn acquire(
        &self,
        key: crate::single_flight_key::SingleFlightKey,
    ) -> crate::single_flight_acquire::SingleFlightAcquire {
        let mut inner = crate::write_inner::write_inner(&self.inner);
        if let Some(sender) = inner.flights.get(&key) {
            return crate::single_flight_acquire::SingleFlightAcquire::Waiter(
                crate::single_flight_waiter::SingleFlightWaiter::from(
                    crate::tokio_single_flight_receiver::TokioSingleFlightReceiver::from(
                        sender.0.subscribe(),
                    ),
                ),
            );
        }
        if inner.flights.len().get() >= self.maximum.0.get() {
            return crate::single_flight_acquire::SingleFlightAcquire::Full;
        }
        let (sender, _) =
            tokio::sync::watch::channel(crate::single_flight_signal::SingleFlightSignal::Running);
        let insertion = inner.flights.try_insert(
            key.clone(),
            crate::tokio_single_flight_sender::TokioSingleFlightSender::from(sender),
        );
        if insertion.is_err() {
            return crate::single_flight_acquire::SingleFlightAcquire::Full;
        }
        drop(inner);
        crate::single_flight_acquire::SingleFlightAcquire::Owner(
            crate::single_flight_owner::SingleFlightOwner {
                inner: self.inner.clone(),
                key: Some(key),
            },
        )
    }

    #[must_use]
    pub fn new(
        maximum: crate::single_flight_maximum_non_zero_usize::SingleFlightMaximumNonZeroUsize,
    ) -> Self {
        Self {
            inner: crate::arc_single_flight_rw_lock::ArcSingleFlightRwLock::default(),
            maximum,
        }
    }
}
#[cfg(test)]
mod tests {
    fn single_flight_key() -> crate::single_flight_key::SingleFlightKey {
        crate::single_flight_key::SingleFlightKey::try_from(String::from(
            constants_str::test_fixtures::TEST_SINGLE_FLIGHT_KEY,
        ))
        .expect("68276323 key invariant must hold")
    }

    #[tokio::test]
    async fn one_owner_notifies_waiters_and_releases_key() {
        let flights = super::SingleFlight::new(
            crate::single_flight_maximum_non_zero_usize::SingleFlightMaximumNonZeroUsize::from(
                std::num::NonZeroUsize::MIN,
            ),
        );
        let crate::single_flight_acquire::SingleFlightAcquire::Owner(owner) =
            flights.acquire(single_flight_key())
        else {
            panic!("7961dd01");
        };
        let crate::single_flight_acquire::SingleFlightAcquire::Waiter(waiter) =
            flights.acquire(single_flight_key())
        else {
            panic!("5b54e5a1");
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
