#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct SingleFlight {
    inner: ArcSingleFlightRwLock,
    maximum: SingleFlightMaximumNonZeroUsize,
}
impl SingleFlight {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // the lock-backed return value cannot be constructed in const context
    pub fn acquire(&self, key: SingleFlightKey) -> SingleFlightAcquire {
        let mut inner = write_inner(&self.inner);
        if let Some(sender) = inner.flights.get(&key) {
            return SingleFlightAcquire::Waiter(SingleFlightWaiter::from(
                TokioSingleFlightReceiver::from(sender.0.subscribe()),
            ));
        }
        if inner.flights.len().get() >= self.maximum.0.get() {
            return SingleFlightAcquire::Full;
        }
        let (sender, _) = tokio::sync::watch::channel(SingleFlightSignal::Running);
        let insertion = inner
            .flights
            .try_insert(key.clone(), TokioSingleFlightSender::from(sender));
        if insertion.is_err() {
            return SingleFlightAcquire::Full;
        }
        drop(inner);
        SingleFlightAcquire::Owner(SingleFlightOwner {
            inner: self.inner.clone(),
            key: Some(key),
        })
    }

    #[must_use]
    pub fn new(maximum: SingleFlightMaximumNonZeroUsize) -> Self {
        Self {
            inner: ArcSingleFlightRwLock::default(),
            maximum,
        }
    }
}

use super::arc_single_flight_rw_lock::ArcSingleFlightRwLock;
pub use super::single_flight_acquire::SingleFlightAcquire;
use super::single_flight_inner::SingleFlightInner;
pub use super::single_flight_key::SingleFlightKey;
pub use super::single_flight_key_error::SingleFlightKeyError;
use super::single_flight_key_maximum_bytes::SINGLE_FLIGHT_KEY_MAXIMUM_BYTES;
pub use super::single_flight_maximum_non_zero_usize::SingleFlightMaximumNonZeroUsize;
pub use super::single_flight_owner::SingleFlightOwner;
use super::single_flight_rw_lock_write_guard::SingleFlightRwLockWriteGuard;
use super::single_flight_signal::SingleFlightSignal;
pub use super::single_flight_wait_outcome::SingleFlightWaitOutcome;
pub use super::single_flight_waiter::SingleFlightWaiter;
use super::tokio_single_flight_receiver::TokioSingleFlightReceiver;
use super::tokio_single_flight_sender::TokioSingleFlightSender;
use super::write_inner::write_inner;
#[cfg(test)]
mod tests {
    fn single_flight_key() -> super::SingleFlightKey {
        super::SingleFlightKey::try_from(String::from(constants_str::TEST_SINGLE_FLIGHT_KEY))
            .expect("68276323 key invariant must hold")
    }

    #[tokio::test]
    async fn one_owner_notifies_waiters_and_releases_key() {
        let flights = super::SingleFlight::new(super::SingleFlightMaximumNonZeroUsize::from(
            std::num::NonZeroUsize::MIN,
        ));
        let super::SingleFlightAcquire::Owner(owner) = flights.acquire(single_flight_key()) else {
            panic!("7961dd01");
        };
        let super::SingleFlightAcquire::Waiter(waiter) = flights.acquire(single_flight_key())
        else {
            panic!("5b54e5a1");
        };
        drop(owner);
        assert_eq!(waiter.wait().await, super::SingleFlightWaitOutcome::Retry);
        assert!(matches!(
            flights.acquire(single_flight_key()),
            super::SingleFlightAcquire::Owner(_)
        ));
    }
}
