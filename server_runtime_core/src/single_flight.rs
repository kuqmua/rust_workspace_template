#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[path = "arc_single_flight_rw_lock.rs"]
mod arc_single_flight_rw_lock;
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
#[path = "single_flight_acquire.rs"]
mod single_flight_acquire;
#[path = "single_flight_inner.rs"]
mod single_flight_inner;
#[path = "single_flight_key.rs"]
mod single_flight_key;
#[path = "single_flight_key_error.rs"]
mod single_flight_key_error;
#[path = "single_flight_key_maximum_bytes.rs"]
mod single_flight_key_maximum_bytes;
#[path = "single_flight_maximum_non_zero_usize.rs"]
mod single_flight_maximum_non_zero_usize;
#[path = "single_flight_owner.rs"]
mod single_flight_owner;
#[path = "single_flight_rw_lock_write_guard.rs"]
mod single_flight_rw_lock_write_guard;
#[path = "single_flight_signal.rs"]
mod single_flight_signal;
#[path = "single_flight_wait_outcome.rs"]
mod single_flight_wait_outcome;
#[path = "single_flight_waiter.rs"]
mod single_flight_waiter;
#[path = "tokio_single_flight_receiver.rs"]
mod tokio_single_flight_receiver;
#[path = "tokio_single_flight_sender.rs"]
mod tokio_single_flight_sender;
#[path = "write_inner.rs"]
mod write_inner;

use arc_single_flight_rw_lock::ArcSingleFlightRwLock;
pub use single_flight_acquire::SingleFlightAcquire;
use single_flight_inner::SingleFlightInner;
pub use single_flight_key::SingleFlightKey;
pub use single_flight_key_error::SingleFlightKeyError;
use single_flight_key_maximum_bytes::SINGLE_FLIGHT_KEY_MAXIMUM_BYTES;
pub use single_flight_maximum_non_zero_usize::SingleFlightMaximumNonZeroUsize;
pub use single_flight_owner::SingleFlightOwner;
use single_flight_rw_lock_write_guard::SingleFlightRwLockWriteGuard;
use single_flight_signal::SingleFlightSignal;
pub use single_flight_wait_outcome::SingleFlightWaitOutcome;
pub use single_flight_waiter::SingleFlightWaiter;
use tokio_single_flight_receiver::TokioSingleFlightReceiver;
use tokio_single_flight_sender::TokioSingleFlightSender;
use write_inner::write_inner;

#[cfg(test)]
mod tests {
    fn key() -> super::SingleFlightKey {
        super::SingleFlightKey::try_from(String::from(constants_str::TEST_SINGLE_FLIGHT_KEY))
            .expect("68276323 key invariant must hold")
    }

    #[tokio::test]
    async fn one_owner_notifies_waiters_and_releases_key() {
        let flights = super::SingleFlight::new(super::SingleFlightMaximumNonZeroUsize::from(
            std::num::NonZeroUsize::MIN,
        ));
        let super::SingleFlightAcquire::Owner(owner) = flights.acquire(key()) else {
            panic!("7961dd01");
        };
        let super::SingleFlightAcquire::Waiter(waiter) = flights.acquire(key()) else {
            panic!("5b54e5a1");
        };
        drop(owner);
        assert_eq!(waiter.wait().await, super::SingleFlightWaitOutcome::Retry);
        assert!(matches!(
            flights.acquire(key()),
            super::SingleFlightAcquire::Owner(_)
        ));
    }
}
