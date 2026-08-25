const SINGLE_FLIGHT_KEY_MAXIMUM_BYTES: usize = 1024usize;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SingleFlightKey(String);
impl TryFrom<String> for SingleFlightKey {
    type Error = SingleFlightKeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SINGLE_FLIGHT_KEY_MAXIMUM_BYTES {
            return Err(SingleFlightKeyError::TooLong);
        }
        if value.is_empty() {
            Err(SingleFlightKeyError::Empty)
        } else if value.contains('\0') {
            Err(SingleFlightKeyError::ContainsNul)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum SingleFlightKeyError {
    #[error("single-flight key contains a NUL character")]
    ContainsNul,
    #[error("single-flight key must not be empty")]
    Empty,
    #[error("single-flight key exceeds its maximum length")]
    TooLong,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct SingleFlightMaximumNonZeroUsize(std::num::NonZeroUsize);

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

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub enum SingleFlightAcquire {
    Full,
    Owner(SingleFlightOwner),
    Waiter(SingleFlightWaiter),
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct SingleFlightOwner {
    inner: ArcSingleFlightRwLock,
    key: Option<SingleFlightKey>,
}
impl Drop for SingleFlightOwner {
    fn drop(&mut self) {
        let Some(key) = self.key.take() else {
            return;
        };
        let optional_sender = write_inner(&self.inner).flights.remove(&key);
        if let Some(sender) = optional_sender {
            let _send_result = sender.0.send(SingleFlightSignal::Retry);
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SingleFlightWaiter(TokioSingleFlightReceiver);

impl SingleFlightWaiter {
    pub async fn wait(mut self) -> SingleFlightWaitOutcome {
        match self.0.0.changed().await {
            Ok(()) | Err(_) => SingleFlightWaitOutcome::Retry,
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SingleFlightWaitOutcome {
    Retry,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
struct SingleFlightInner {
    flights: bounded_types::domain_types::hash::BoundedHashMap<
        SingleFlightKey,
        TokioSingleFlightSender,
        { usize::MAX },
    >,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, newtype::FromInner)]
struct ArcSingleFlightRwLock(std::sync::Arc<std::sync::RwLock<SingleFlightInner>>);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefMutTarget,
    newtype::DerefTarget,
    newtype::FromInner,
)]
struct SingleFlightRwLockWriteGuard<'value_lt>(
    std::sync::RwLockWriteGuard<'value_lt, SingleFlightInner>,
);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
enum SingleFlightSignal {
    Retry,
    Running,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
struct TokioSingleFlightReceiver(tokio::sync::watch::Receiver<SingleFlightSignal>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
struct TokioSingleFlightSender(tokio::sync::watch::Sender<SingleFlightSignal>);

fn write_inner(inner: &ArcSingleFlightRwLock) -> SingleFlightRwLockWriteGuard<'_> {
    match inner.0.write() {
        Ok(guard) => SingleFlightRwLockWriteGuard::from(guard),
        Err(poisoned) => SingleFlightRwLockWriteGuard::from(poisoned.into_inner()),
    }
}

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
