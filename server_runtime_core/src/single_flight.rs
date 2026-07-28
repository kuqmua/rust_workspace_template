const SINGLE_FLIGHT_KEY_MAXIMUM_BYTES: usize = 1024usize;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SingleFlightKeyError {
    #[error("single-flight key contains a NUL character")]
    ContainsNul,
    #[error("single-flight key must not be empty")]
    Empty,
    #[error("single-flight key exceeds its maximum length")]
    TooLong,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct StdSingleFlightMaximum(std::num::NonZeroUsize);

#[derive(Clone, Debug)]
pub struct SingleFlight {
    inner: StdArcStdSingleFlightRwLock,
    maximum: StdSingleFlightMaximum,
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
        if inner.flights.len() >= self.maximum.0.get() {
            return SingleFlightAcquire::Full;
        }
        let (sender, receiver) = tokio::sync::watch::channel(SingleFlightSignal::Running);
        drop(receiver);
        let _previous = inner
            .flights
            .insert(key.clone(), TokioSingleFlightSender::from(sender));
        drop(inner);
        SingleFlightAcquire::Owner(SingleFlightOwner {
            inner: self.inner.clone(),
            key: Some(key),
        })
    }

    #[must_use]
    pub fn new(maximum: StdSingleFlightMaximum) -> Self {
        Self {
            inner: StdArcStdSingleFlightRwLock::default(),
            maximum,
        }
    }
}

#[derive(Debug)]
pub enum SingleFlightAcquire {
    Full,
    Owner(SingleFlightOwner),
    Waiter(SingleFlightWaiter),
}

#[derive(Debug)]
#[must_use]
pub struct SingleFlightOwner {
    inner: StdArcStdSingleFlightRwLock,
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

#[derive(Debug, newtype::FromInner)]
pub struct SingleFlightWaiter(TokioSingleFlightReceiver);

impl SingleFlightWaiter {
    pub async fn wait(mut self) -> SingleFlightWaitOutcome {
        match self.0.0.changed().await {
            Ok(()) | Err(_) => SingleFlightWaitOutcome::Retry,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SingleFlightWaitOutcome {
    Retry,
}

#[derive(Debug, Default)]
struct SingleFlightInner {
    flights: std::collections::HashMap<SingleFlightKey, TokioSingleFlightSender>,
}

#[derive(Clone, Debug, Default, newtype::FromInner)]
struct StdArcStdSingleFlightRwLock(std::sync::Arc<std::sync::RwLock<SingleFlightInner>>);

#[derive(Debug, newtype::DerefMutTarget, newtype::DerefTarget, newtype::FromInner)]
struct StdSingleFlightWriteGuard<'value_lt>(
    std::sync::RwLockWriteGuard<'value_lt, SingleFlightInner>,
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SingleFlightSignal {
    Retry,
    Running,
}

#[derive(Clone, Debug, newtype::FromInner)]
struct TokioSingleFlightReceiver(tokio::sync::watch::Receiver<SingleFlightSignal>);

#[derive(Clone, Debug, newtype::FromInner)]
struct TokioSingleFlightSender(tokio::sync::watch::Sender<SingleFlightSignal>);

fn write_inner(inner: &StdArcStdSingleFlightRwLock) -> StdSingleFlightWriteGuard<'_> {
    match inner.0.write() {
        Ok(guard) => StdSingleFlightWriteGuard::from(guard),
        Err(poisoned) => StdSingleFlightWriteGuard::from(poisoned.into_inner()),
    }
}

#[cfg(test)]
mod tests {
    fn key() -> super::SingleFlightKey {
        super::SingleFlightKey::try_from(String::from(str_constants::TEST_SINGLE_FLIGHT_KEY))
            .expect("68276323")
    }

    #[tokio::test]
    async fn one_owner_notifies_waiters_and_releases_key() {
        let flights = super::SingleFlight::new(super::StdSingleFlightMaximum::from(
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
